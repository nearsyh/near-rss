use rocket::response::Redirect;
use crate::common::Services;

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

#[get("/refresh")]
pub async fn refresh(services: &Services) -> &'static str {
    services
        .subscription_service
        .load_all_subscription_items()
        .await
        .unwrap();
    "OK"
}
