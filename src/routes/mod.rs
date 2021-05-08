use rocket::response::Redirect;

pub mod accounts;
pub mod reader;
pub mod ui;

#[catch(403)]
pub fn unauthorized() -> &'static str {
    "Unauthorized"
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/ui", ui::index::already_login))
}
