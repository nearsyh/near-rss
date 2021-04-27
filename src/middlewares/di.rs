use crate::common::error::Errors;
use crate::database::subscriptions::new_subscription_repository;
use crate::database::users::new_user_repository;
use crate::services::subscriptions::{new_subscription_service, SubscriptionService};
use crate::services::users::{new_user_service, UserService};
use async_once::AsyncOnce;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::SqlitePool;

pub struct Services {
  pub user_service: Box<dyn UserService + Send + Sync>,
  pub subscription_service: Box<dyn SubscriptionService + Send + Sync>,
}

impl Services {
  async fn new(pool: SqlitePool) -> Services {
    let user_repository = new_user_repository(pool.clone()).await.unwrap();
    let subscription_repository = new_subscription_repository(pool.clone()).await.unwrap();

    Services {
      user_service: new_user_service(user_repository),
      subscription_service: new_subscription_service(subscription_repository),
    }
  }
}

unsafe impl Send for Services {}
unsafe impl Sync for Services {}

lazy_static! {
  pub static ref SERVICES: AsyncOnce<Services> = AsyncOnce::new(async {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let ret = Services::new(pool).await;
    let user = ret.user_service.register("nearsy.h@gmail.com", "1234")
        .await
        .unwrap();
    ret.subscription_service
        .add_subscription_from_url(&user.id, "https://www.daemonology.net/hn-daily/index.rss")
        .await
        .unwrap();
    ret
  });
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &Services {
  type Error = Errors;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    Outcome::Success(SERVICES.get().await)
  }
}
