use crate::helpers::spawn_app;
use std::thread::sleep;

#[tokio::test]
async fn unknown_user_login_return_403() {
    let app = spawn_app().await;

    let response = app.login("random@gmail.com", "1234").await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Error=BadAuthentication")
}

#[tokio::test]
async fn invalid_password_login_return_403() {
    let app = spawn_app().await;

    let response = app.login(&app.test_user.email, "1234").await;

    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.text().await.unwrap(), "Error=BadAuthentication")
}

#[tokio::test]
async fn valid_password_login_return_200_and_token() {
    let app = spawn_app().await;

    let response = app
        .login(&app.test_user.email, &app.test_user.password)
        .await;

    assert_eq!(response.status().as_u16(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("SID="));
    assert!(body.contains("LSID="));
    assert!(body.contains("Auth="));
}
