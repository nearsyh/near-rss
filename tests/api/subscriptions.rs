use crate::data::Subscriptions;
use crate::helpers::{spawn_app, TestApp};

#[tokio::test]
async fn anonymous_add_subscription_should_fail() {
    let app = spawn_app().await;

    let response = app.add_subscription("link", None, None).await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
}

#[tokio::test]
async fn anonymous_list_subscriptions_should_fail() {
    let app = spawn_app().await;

    let response = app.list_subscriptions().await;

    assert_eq!(response.status().as_u16(), 403);
}

#[tokio::test]
async fn anonymous_quick_add_subscriptions_should_fail() {
    let app = spawn_app().await;

    let response = app
        .quick_add_subscription("https://blogs.nearsyh.me/atom.xml")
        .await;

    assert_eq!(response.status().as_u16(), 403);
}

#[tokio::test]
async fn add_new_subscription() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app
        .add_subscription("https://blogs.nearsyh.me/atom.xml", None, None)
        .await;
    assert_eq!(response.status().as_u16(), 200);
    assert!(has_subscription(&app, "https://blogs.nearsyh.me/atom.xml").await);
}

#[tokio::test]
async fn list_subscriptions() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app.list_subscriptions().await;

    assert_eq!(response.status().as_u16(), 200);
    let subscriptions = response.json::<Subscriptions>().await.unwrap();
    assert!(subscriptions.subscriptions.is_empty());

    app.add_subscription("https://blogs.nearsyh.me/atom.xml", None, None)
        .await;

    assert!(has_subscription(&app, "https://blogs.nearsyh.me/atom.xml").await);
}

#[tokio::test]
async fn quick_add_new_subscription() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app
        .quick_add_subscription("https://blogs.nearsyh.me/atom.xml")
        .await;
    assert_eq!(response.status().as_u16(), 200);

    assert!(has_subscription(&app, "https://blogs.nearsyh.me/atom.xml").await);
}

async fn has_subscription(app: &TestApp, link: &str) -> bool {
    let response = app.list_subscriptions().await;
    let subscriptions = response.json::<Subscriptions>().await.unwrap();
    subscriptions.subscriptions[0].feed_url.eq(link)
}
