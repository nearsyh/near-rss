#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod common;
mod database;
mod middlewares;
mod routes;
mod services;

use rocket_contrib::templates::Template;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/accounts", routes![routes::accounts::client_login])
        .mount(
            "/reader",
            routes![
                routes::reader::ping,
                routes::reader::subscriptions::list_subscriptions,
                routes::reader::subscriptions::add_subscription,
                routes::reader::users::get_user_info,
                routes::reader::users::token,
                routes::reader::stream::get_item_ids,
                routes::reader::stream::get_contents,
                routes::reader::edit::edit_tag,
            ],
        )
        .mount(
            "/ui",
            routes![
                routes::ui::index::already_login,
                routes::ui::index::not_login,
                routes::ui::login::login,
            ],
        )
        .mount("/", routes![routes::index])
        .attach(Template::fairing())
        .register("/", catchers![routes::unauthorized])
}
