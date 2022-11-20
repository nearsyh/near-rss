use crate::helpers::spawn_app;

#[tokio::test]
async fn anonymous_add_subscription_should_fail() {
    let app = spawn_app().await;

    let response = app.add_subscription("link", None, None).await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
}

#[tokio::test]
async fn add_new_subscription() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app.add_subscription("link", None, None).await;
    assert_eq!(response.status().as_u16(), 200);
}
