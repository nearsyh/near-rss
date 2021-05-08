use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use std::collections::HashMap;
use crate::middlewares::auth::AuthUiUser;

#[get("/")]
pub fn already_login(_auth_user: AuthUiUser) -> Template {
  let context = HashMap::<String, String>::new();
  Template::render("index", &context)
}

#[get("/", rank = 2)]
pub fn not_login() -> Redirect {
  Redirect::to(uri!("/ui", super::login::login))
}
