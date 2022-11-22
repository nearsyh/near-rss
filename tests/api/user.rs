use crate::data::UserInfo;
use crate::helpers::spawn_app;

#[tokio::test]
async fn anonymous_user_should_fail() {
    let app = spawn_app().await;

    let response = app.get_user_info().await;
    assert_eq!(response.status().as_u16(), 403);

    let response = app.get_user_token().await;
    assert_eq!(response.status().as_u16(), 403);
}

#[tokio::test]
async fn get_user_info_should_work() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app.get_user_info().await;
    assert_eq!(response.status().as_u16(), 200);
    let user_info = response.json::<UserInfo>().await.unwrap();
    assert!(!user_info.user_id.is_empty());
    assert_eq!(user_info.user_email, app.test_user.email);
}

#[tokio::test]
async fn get_user_token_should_work() {
    let mut app = spawn_app().await;
    app.test_user_login().await;

    let response = app.get_user_token().await;
    assert_eq!(response.status().as_u16(), 200);

    let token = response.text().await.unwrap();
    assert_eq!(token, app.token.unwrap());
}
