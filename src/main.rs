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

async fn init() {}

#[launch]
async fn rocket() -> _ {
    init().await;
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
            ],
        )
        .register("/", catchers![routes::unauthorized])
}
