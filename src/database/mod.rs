pub mod items;
pub mod subscriptions;
pub mod users;

use log::LevelFilter;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn in_memory_pool() -> SqlitePool {
    let mut option = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    option.log_statements(LevelFilter::Off);
    let pool = SqlitePool::connect_with(option).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database.");
    pool
}

pub async fn db_pool(path: &str) -> SqlitePool {
    let db = format!("sqlite:{}", path);
    let mut option = SqliteConnectOptions::from_str(&db)
        .unwrap()
        .create_if_missing(true);
    option.log_statements(LevelFilter::Off);
    SqlitePool::connect_with(option).await.unwrap()
}
