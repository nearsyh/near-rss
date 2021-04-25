use crate::services::users::new_user_service;
use rocket::request::{Form, FromForm};
use rocket::response::status::Forbidden;

#[derive(FromForm)]
pub struct LoginRequest {
  #[form(field = "Email")]
  email: String,
  #[form(field = "Passwd")]
  password: String,
}

#[post("/accounts/ClientLogin", data = "<request>")]
pub fn client_login(request: Form<LoginRequest>) -> Result<String, Forbidden<String>> {
  match new_user_service().login(&request.email, &request.password) {
    Ok(ref creds) => Ok(format!(
      "SID={}\nLSID={}\nAuth={}",
      creds.sid, creds.lsid, creds.cltoken
    )),
    Err(_) => Err(Forbidden(Some(String::from("Error=BadAuthentication")))),
  }
}
