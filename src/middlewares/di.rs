use crate::common::{debug::init_data, error::Errors, refresh, Services};
use crate::database::items::new_item_repository;
use crate::database::subscriptions::new_subscription_repository;
use crate::database::users::new_user_repository;
use crate::services::stream::new_stream_service;
use crate::services::subscriptions::new_subscription_service;
use crate::services::users::new_user_service;
use async_once::AsyncOnce;
use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use log::warn;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::SqlitePool;
use std::time::Duration;

impl Services {
    async fn new(pool: SqlitePool) -> Services {
        Services {
            user_service: new_user_service(new_user_repository(pool.clone()).await.unwrap()),
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

unsafe impl Send for Services {}
unsafe impl Sync for Services {}

lazy_static! {
    pub static ref SERVICES: AsyncOnce<Services> = AsyncOnce::new(async {
        let pool = if let Ok(db_path) = std::env::var("DB") {
            crate::database::db_pool(&db_path).await
        } else {
            warn!("Using inmemory dabase instance");
            crate::database::in_memory_pool().await
        };
        let ret = Services::new(pool).await;
        init_data(&ret).await;
        ret
    });
    pub static ref THREAD: AsyncOnce<ScheduleHandle> = AsyncOnce::new(async {
        let mut scheduler = Scheduler::new();
        scheduler.every(10.minutes()).run(move || {
            refresh();
        });
        scheduler.watch_thread(Duration::from_secs(60))
    });
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &Services {
    type Error = Errors;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(SERVICES.get().await)
    }
}
