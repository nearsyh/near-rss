use crate::services::users::new_user_service;
use rocket::form::Form;
use rocket::response::status::Forbidden;

#[derive(FromForm)]
pub struct LoginRequest {
    #[field(name = "Email")]
    email: String,
    #[field(name = "Passwd")]
    password: String,
}

#[post("/ClientLogin", data = "<request>")]
pub async fn client_login(request: Form<LoginRequest>) -> Result<String, Forbidden<String>> {
    match new_user_service().await
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
