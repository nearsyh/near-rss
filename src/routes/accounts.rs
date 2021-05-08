use crate::common::{Services, login::LoginRequest};
use rocket::form::Form;
use rocket::response::status::Forbidden;

#[post("/ClientLogin", data = "<request>")]
pub async fn client_login(
    request: Form<LoginRequest>,
    services: &Services,
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
