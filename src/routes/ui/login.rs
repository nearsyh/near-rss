use crate::common::Services;
use crate::middlewares::auth::AuthUiUser;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/login")]
pub fn login_with_creds(_auth_user: AuthUiUser) -> Redirect {
    Redirect::to("/ui")
}

#[get("/login", rank = 2)]
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
pub async fn login_action(
    request: Form<LoginRequest>,
    services: &Services,
    cookies: &CookieJar<'_>,
) -> Redirect {
    match services
        .user_service
        .login(&request.email, &request.password)
        .await
    {
        Ok(creds) => {
            cookies.add(Cookie::new("cltoken", creds.cltoken));
            Redirect::to("/ui")
        }
        Err(_) => Redirect::to("/ui/login"),
    }
}
