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
pub mod configuration;

use crate::middlewares::di::{SERVICES, THREAD};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::{ContentType, Header, Method};
use rocket::{Build, Config, Request, Response, Rocket};
use std::io::Cursor;
use std::net::IpAddr;
use sqlx::sqlite::SqlitePoolOptions;
use crate::common::Services;
use crate::configuration::Configuration;

pub struct CORS();

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(0, Cursor::new(""));
        }
    }
}

pub async fn create(configuration: &Configuration) -> Rocket<Build> {
    SERVICES.get().await;
    THREAD.get().await;

    let sqlite_pool = SqlitePoolOptions::new().connect_lazy_with(configuration.database.connect_options());
    let services = Services::new(sqlite_pool).await;

    let config = Config {
        port: configuration.application.port,
        address: configuration.application.host.parse::<IpAddr>().expect("Failed to parse host address"),
        ..Config::debug_default()
    };

    rocket::custom(config)
        .mount("/", FileServer::from(relative!("public")))
        .mount("/accounts", routes![routes::accounts::client_login])
        .mount(
            "/reader",
            routes![
                routes::reader::ping,
                routes::reader::subscriptions::list_subscriptions,
                routes::reader::subscriptions::add_subscription,
                routes::reader::subscriptions::edit_subscription,
                routes::reader::users::get_user_info,
                routes::reader::users::token,
                routes::reader::stream::get_item_ids,
                routes::reader::stream::get_contents,
                routes::reader::edit::edit_tag,
            ],
        )
        .mount(
            "/api",
            routes![
                routes::api::get_unread_items,
                routes::api::get_unread_items_options,
                routes::api::mark_as_read,
                routes::api::mark_as_read_options,
                routes::api::add_subscription,
                routes::api::add_subscription_options,
            ],
        )
        .mount("/", routes![routes::refresh])
        .attach(CORS())
        .register("/", catchers![routes::unauthorized])
        .manage(services)
}