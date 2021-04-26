#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod common;
mod database;
mod middlewares;
mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/accounts", routes![routes::accounts::client_login])
        .mount(
            "/reader",
            routes![
                routes::reader::ping,
                routes::reader::subscriptions::list_subscriptions,
                routes::reader::users::get_user_info,
                routes::reader::stream::get_item_ids,
                routes::reader::stream::get_contents,
            ],
        )
        .register("/", catchers![routes::unauthorized])
}
