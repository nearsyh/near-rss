pub mod subscriptions;
pub mod users;

use sqlx::SqlitePool;

pub async fn in_memory_pool() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:").await.unwrap()
}
