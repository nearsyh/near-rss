use crate::data::Contents;
use crate::helpers::{spawn_app, spawn_app_by_type};

#[tokio::test]
async fn anonymous_list_unread_should_fail() {
    let app = spawn_app().await;

    let response = app.get_unread_items(None, None).await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
}

#[tokio::test]
async fn list_unread_items_should_return_200() {
    let mut app = spawn_app().await;
    app.test_user_login().await;
    app.add_subscription("https://rsshub.app/36kr/information/web_news", None, None)
        .await;

    let response = app.get_unread_items(None, None).await;

    assert_eq!(response.status().as_u16(), 200);
    let contents = response
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents");
    assert!(!contents.items.is_empty());
}

#[tokio::test]
async fn list_unread_items_with_limit_should_return_200() {
    let mut app = spawn_app().await;
    app.test_user_login().await;
    app.add_subscription("https://rsshub.app/36kr/information/web_news", None, None)
        .await;

    let response = app.get_unread_items(None, Some(10)).await;

    assert_eq!(response.status().as_u16(), 200);
    let contents = response
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents");
    assert_eq!(contents.items.len(), 10);
}

#[tokio::test]
async fn list_unread_items_with_offset_should_return_200() {
    let mut app = spawn_app().await;
    app.test_user_login().await;
    app.add_subscription("https://rsshub.app/36kr/information/web_news", None, None)
        .await;

    let first_page = app
        .get_unread_items(None, Some(10))
        .await
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents");

    let second_page = app
        .get_unread_items(first_page.next_page_offset.clone(), Some(10))
        .await
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents");
    assert_ne!(first_page.items[0].id, second_page.items[0].id);
}
