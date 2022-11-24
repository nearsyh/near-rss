use crate::helpers::spawn_app;

#[tokio::test]
async fn anonymous_ping_should_fail() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(format!("{}/reader/ping", app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 403);
}

#[tokio::test]
async fn ping_should_work() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app
        .api_client
        .get(format!("{}/reader/ping", app.address))
        .header(
            "Authorization",
            format!("GoogleLogin auth={}", app.token.as_deref().unwrap_or("")),
        )
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), "OK");
}
