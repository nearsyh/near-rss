use crate::middlewares::auth::AuthUser;
use actix_web::{web, HttpResponse};
use serde::Serialize;

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

pub async fn get_user_info(auth_user: web::ReqData<AuthUser>) -> HttpResponse {
    HttpResponse::Ok().json(UserInfo {
        user_id: auth_user.id.clone(),
        user_name: auth_user.email.clone(),
        user_profile_id: auth_user.id.clone(),
        user_email: auth_user.email.clone(),
        is_blogged_user: true,
        signup_time_sec: 12345678,
        is_multi_login_enabled: true,
    })
}

pub async fn token(auth_user: web::ReqData<AuthUser>) -> HttpResponse {
    HttpResponse::Ok().body(auth_user.token.clone())
}
