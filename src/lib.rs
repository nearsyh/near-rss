#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod common;
pub mod configuration;
pub mod database;
mod middlewares;
mod routes;
mod services;

use crate::common::Services;
use crate::configuration::Configuration;
use crate::middlewares::auth::reject_anonymous_user;
use crate::middlewares::di::{SERVICES, THREAD};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use anyhow::{Context, Result};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Build, Config, Request, Response, Rocket};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::io::Cursor;
use std::net::{IpAddr, TcpListener};

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

pub enum ServerWrapper {
    RocketServer(Rocket<Build>),
    ActixServer(Server),
}

pub struct Application {
    pub server: ServerWrapper,
    pub pool: SqlitePool,
    pub port: u16,
}

impl Application {
    pub async fn create_rocket_server(configuration: &Configuration) -> Result<Application> {
        SERVICES.get().await;
        THREAD.get().await;

        let sqlite_pool =
            SqlitePoolOptions::new().connect_lazy_with(configuration.database.connect_options());
        let services = Services::new(sqlite_pool.clone()).await;

        let config = Config {
            port: configuration.application.port,
            address: configuration
                .application
                .host
                .parse::<IpAddr>()
                .expect("Failed to parse host address"),
            ..Config::debug_default()
        };

        let rocket = rocket::custom(config)
            .mount(
                "/reader",
                routes![
                    routes::reader::old_ping,
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
                    routes::api::old_get_unread_items,
                    routes::api::get_unread_items_options,
                    routes::api::old_mark_as_read,
                    routes::api::mark_as_read_options,
                    routes::api::old_add_subscription,
                    routes::api::add_subscription_options,
                ],
            )
            .mount("/accounts", routes![routes::accounts::old_client_login])
            .mount("/", routes![routes::refresh])
            .attach(CORS())
            .register("/", catchers![routes::unauthorized])
            .manage(services);
        Ok(Application {
            pool: sqlite_pool,
            server: ServerWrapper::RocketServer(rocket),
            port: configuration.application.port,
        })
    }

    pub async fn create_actix_server(configuration: &Configuration) -> Result<Application> {
        let sqlite_pool =
            SqlitePoolOptions::new().connect_lazy_with(configuration.database.connect_options());
        let services = web::Data::new(Services::new(sqlite_pool.clone()).await);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(services.clone())
                .service(web::scope("/accounts").route(
                    "/ClientLogin",
                    web::post().to(routes::accounts::client_login),
                ))
                .service(
                    web::scope("/api")
                        .wrap(from_fn(reject_anonymous_user))
                        .app_data(services.clone())
                        .route(
                            "/addSubscription",
                            web::post().to(routes::api::add_subscription),
                        )
                        .route("/markAsRead", web::post().to(routes::api::mark_as_read))
                        .route("/unread", web::get().to(routes::api::get_unread_items)),
                )
                .service(
                    web::scope("/reader")
                        .wrap(from_fn(reject_anonymous_user))
                        .app_data(services.clone())
                        .route("/ping", web::get().to(routes::reader::ping)),
                )
                .service(actix_files::Files::new("/", "./public"))
        })
        .listen(listener)?
        .run();

        Ok(Application {
            pool: sqlite_pool,
            server: ServerWrapper::ActixServer(server),
            port,
        })
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        match self.server {
            ServerWrapper::RocketServer(rocket) => rocket
                .launch()
                .await
                .context("Failed to launch rocket server"),
            ServerWrapper::ActixServer(server) => {
                server.await.context("Failed to launch actix server")
            }
        }
    }
}
