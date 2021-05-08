use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::form::Form;
use crate::common::Services;
use rocket::response::Redirect;

#[get("/login")]
pub fn login() -> Template {
  let context = HashMap::<String, String>::new();
  Template::render("login", &context)
}

#[derive(FromForm)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login", data = "<request>")]
pub async fn login_with_creds(
    request: Form<LoginRequest>,
    services: &Services,
) -> Redirect {
    match services
        .user_service
        .login(&request.email, &request.password)
        .await
    {
        Ok(ref creds) => Redirect::to("/ui"),
        Err(_) => Redirect::to("/ui/login"),
    }
}
