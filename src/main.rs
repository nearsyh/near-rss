#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod middlewares;
mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/accounts", routes![routes::accounts::client_login])
        .mount("/reader", routes![routes::reader::ping])
}