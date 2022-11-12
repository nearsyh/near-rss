use std::str::FromStr;
use log::LevelFilter;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, SqlitePool};
use near_rss::configuration::{Configuration, get_configuration};
use near_rss::create;

pub struct TestApp {
    pub port: u16,
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to get configuration.");
    configure_database(&configuration).await;

    let rocket = create(&configuration).await;
    let _ = tokio::spawn(rocket.launch());
    TestApp {
        port: configuration.application.port,
        address: format!("http://127.0.0.1:{}", configuration.application.port),
    }
}

async fn configure_database(configuration: &Configuration) {
    let option = SqliteConnectOptions::from_str("sqlite::memory:")
        .expect("Failed to create connect options");
    let sqlite_pool = SqlitePoolOptions::new().connect_lazy_with(option);
    sqlx::migrate!("./migrations")
        .run(&sqlite_pool)
        .await
        .expect("Failed to migrate the database.");
}