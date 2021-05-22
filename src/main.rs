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
use crate::middlewares::di::{SERVICES, THREAD};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Request, Response};
use std::io::Cursor;

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

#[launch]
async fn rocket() -> _ {
  SERVICES.get().await;
  THREAD.get().await;

  rocket::build()
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
}
