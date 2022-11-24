use crate::common::Services;
use crate::database::items::new_item_repository;
use crate::database::subscriptions::new_subscription_repository;
use crate::services::stream::new_stream_service;
use crate::services::subscriptions::new_subscription_service;
use sqlx::SqlitePool;

impl Services {
    pub async fn new(pool: SqlitePool) -> Services {
        Services {
            subscription_service: new_subscription_service(
                new_subscription_repository(pool.clone()).await.unwrap(),
                new_item_repository(pool.clone()).await.unwrap(),
            ),
            stream_service: new_stream_service(
                new_item_repository(pool.clone()).await.unwrap(),
                new_subscription_repository(pool.clone()).await.unwrap(),
            ),
        }
    }
}
