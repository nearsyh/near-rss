use rocket::request::{Form, FromForm};

#[derive(FromForm)]
pub struct LoginRequest {
  #[form(field="Email")]
  email: String,
  #[form(field="Passwd")]
  password: String,
}

#[post("/accounts/ClientLogin", data = "<request>")]
pub fn client_login(request: Form<LoginRequest>) -> String {
  format!("Email {} Password {}", request.email, request.password)
}