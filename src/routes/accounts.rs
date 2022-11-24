use crate::user::UserService;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequest {
    email: String,
    passwd: String,
}

pub async fn client_login(
    request: web::Form<LoginRequest>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    match user_service.login(&request.email, &request.passwd).await {
        Ok(ref creds) => HttpResponse::Ok().body(format!(
            "SID={}\nLSID={}\nAuth={}",
            creds.sid, creds.lsid, creds.cltoken
        )),
        Err(_) => HttpResponse::Forbidden().body("Error=BadAuthentication"),
    }
}
