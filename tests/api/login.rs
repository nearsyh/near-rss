use std::thread::sleep;
use crate::helpers::spawn_app;

#[tokio::test]
async fn unknown_user_login_return_403() {
    let app = spawn_app().await;

    let response = app.login("test@gmail.com", "1234").await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Error=BadAuthentication")
}