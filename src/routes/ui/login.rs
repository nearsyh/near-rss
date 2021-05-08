use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/login")]
pub fn login() -> Template {
  let context = HashMap::<String, String>::new();
  Template::render("login", &context)
}