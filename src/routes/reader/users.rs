use crate::middlewares::auth::AuthToken;
use rocket_contrib::json::Json;
use serde::Serialize;
use crate::services::users::new_user_service;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    user_id: String,
    user_name: String,
    user_profile_id: String,
    user_email: String,
    is_blogged_user: bool,
    signup_time_sec: u64,
    is_multi_login_enabled: bool,
}

#[get("/api/0/user-info")]
pub async fn get_user_info(token: AuthToken<'_>) -> Json<UserInfo> {
    let user = new_user_service().get_user(token.0).await;
    Json(UserInfo {
        user_id: user.id.clone(),
        user_name: user.email.clone(),
        user_profile_id: user.id.clone(),
        user_email: user.email.clone(),
        is_blogged_user: true,
        signup_time_sec: 12345678,
        is_multi_login_enabled: true,
    })
}
