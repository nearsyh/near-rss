#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod services;

fn main() {
    rocket::ignite().mount("/", routes![routes::accounts::client_login]).launch();
}
