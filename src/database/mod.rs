pub mod items;
pub mod subscriptions;
pub mod users;

use log::LevelFilter;
use sqlx::{SqlitePool, ConnectOptions, sqlite::SqliteConnectOptions};
use std::str::FromStr;

pub async fn in_memory_pool() -> SqlitePool {
    let mut option = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    option.log_statements(LevelFilter::Off);
    SqlitePool::connect_with(option).await.unwrap()
}
