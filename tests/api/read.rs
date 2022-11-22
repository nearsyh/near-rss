use crate::data::Contents;
use crate::helpers::{spawn_app, spawn_app_by_type};
use std::collections::HashSet;

#[tokio::test]
async fn anonymous_mark_as_read_should_fail() {
    let app = spawn_app_by_type(false).await;

    let response = app.mark_as_read(&vec![]).await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
}

#[tokio::test]
async fn mark_empty_items_as_read_should_return_200() {
    let mut app = spawn_app_by_type(false).await;
    app.test_user_login().await;

    let response = app.mark_as_read(&vec![]).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn items_marked_as_read_should_not_return_in_unread_list() {
    let mut app = spawn_app_by_type(false).await;
    app.test_user_login().await;
    app.add_subscription("https://hnrss.org/newest", None, None)
        .await;

    let unread_ids = app
        .get_unread_items(None, None)
        .await
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents")
        .items
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<String>>();
    assert!(!unread_ids.is_empty());

    let id_1 = unread_ids[0].clone();
    let id_2 = unread_ids[1].clone();
    app.mark_as_read(&[&id_1]).await;
    let unread_ids = app
        .get_unread_items(None, None)
        .await
        .json::<Contents>()
        .await
        .expect("Failed to deserialize to Contents")
        .items
        .into_iter()
        .map(|item| item.id)
        .collect::<HashSet<String>>();

    assert!(!unread_ids.contains(&id_1));
    assert!(unread_ids.contains(&id_2));
}
