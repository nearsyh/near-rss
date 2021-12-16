use crate::middlewares::auth::AuthUser;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
pub async fn get_user_info(auth_user: AuthUser) -> Json<UserInfo> {
    let user = auth_user.user;
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

#[get("/api/0/token")]
pub async fn token(auth_user: AuthUser) -> String {
    auth_user.user.token
}
