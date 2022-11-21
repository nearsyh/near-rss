use crate::common::Services;
use actix_web::{web, HttpResponse};
use rocket::form::Form;
use rocket::response::status::Forbidden;
use rocket::State;

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequest {
    email: String,
    passwd: String,
}

pub async fn client_login(
    request: web::Form<LoginRequest>,
    services: web::Data<Services>,
) -> HttpResponse {
    match services
        .user_service
        .login(&request.email, &request.passwd)
        .await
    {
        Ok(ref creds) => HttpResponse::Ok().body(format!(
            "SID={}\nLSID={}\nAuth={}",
            creds.sid, creds.lsid, creds.cltoken
        )),
        Err(_) => HttpResponse::Forbidden().body("Error=BadAuthentication"),
    }
}

#[derive(FromForm)]
pub struct OldLoginRequest {
    #[field(name = "Email")]
    email: String,
    #[field(name = "Passwd")]
    password: String,
}

#[post("/ClientLogin", data = "<request>")]
pub async fn old_client_login(
    request: Form<OldLoginRequest>,
    services: &State<Services>,
) -> Result<String, Forbidden<String>> {
    match services
        .user_service
        .login(&request.email, &request.password)
        .await
    {
        Ok(ref creds) => Ok(format!(
            "SID={}\nLSID={}\nAuth={}",
            creds.sid, creds.lsid, creds.cltoken
        )),
        Err(_) => Err(Forbidden(Some(String::from("Error=BadAuthentication")))),
    }
}
