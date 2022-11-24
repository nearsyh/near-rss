#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
mod common;
pub mod configuration;
pub mod database;
mod middlewares;
pub mod refresh;
mod routes;
mod services;

use crate::common::Services;
use crate::configuration::Configuration;
use crate::middlewares::auth::reject_anonymous_user;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;

pub struct Application {
    pub server: Server,
    pub pool: SqlitePool,
    pub port: u16,
}

impl Application {
    pub async fn create(configuration: &Configuration) -> Result<Application> {
        let sqlite_pool =
            SqlitePoolOptions::new().connect_lazy_with(configuration.database.connect_options());
        let services = web::Data::new(Services::new(sqlite_pool.clone()).await);

        println!("{:?}", configuration);
        services
            .user_service
            .register(
                &configuration.application.email,
                &configuration.application.password,
            )
            .await
            .expect("Failed to register the user.");

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
                        .route("/ping", web::get().to(routes::reader::ping))
                        .service(
                            web::scope("/api/0")
                                .route(
                                    "/user-info",
                                    web::get().to(routes::reader::users::get_user_info),
                                )
                                .route("/token", web::get().to(routes::reader::users::token))
                                .route(
                                    "/subscription/list",
                                    web::get()
                                        .to(routes::reader::subscriptions::list_subscriptions),
                                )
                                .route(
                                    "/subscription/quickadd",
                                    web::post().to(routes::reader::subscriptions::add_subscription),
                                )
                                .route(
                                    "/subscription/edit",
                                    web::post()
                                        .to(routes::reader::subscriptions::edit_subscription),
                                )
                                .route("/edit-tag", web::post().to(routes::reader::edit::edit_tag))
                                .route(
                                    "/stream/items/ids",
                                    web::get().to(routes::reader::stream::get_item_ids),
                                )
                                .route(
                                    "/stream/items/contents",
                                    web::post().to(routes::reader::stream::get_contents),
                                ),
                        ),
                )
                .service(actix_files::Files::new("/", "./public"))
        })
        .listen(listener)?
        .run();

        Ok(Application {
            pool: sqlite_pool,
            server,
            port,
        })
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.context("Failed to launch actix server")
    }
}
