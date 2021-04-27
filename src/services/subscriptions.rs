use crate::database::subscriptions::{new_subscription_repository, SubscriptionRepository};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub label: String,
}

#[derive(Serialize)]
pub struct Subscription {
    pub id: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub url: String,
    pub feed_url: String,
}

impl From<crate::database::subscriptions::Subscription> for Subscription {
    fn from(db_subscription: crate::database::subscriptions::Subscription) -> Self {
        Subscription {
            id: db_subscription.id,
            title: db_subscription.title,
            description: db_subscription.description,
            url: db_subscription.url,
            feed_url: db_subscription.feed_url,
            categories: db_subscription
                .categories()
                .into_iter()
                .map(|category_str| Category {
                    id: format!("user/{}/label/{}", db_subscription.user_id, category_str),
                    label: category_str.to_string(),
                })
                .collect(),
        }
    }
}

impl Subscription {
    fn to_db(self, user_id: &str) -> crate::database::subscriptions::Subscription {
        crate::database::subscriptions::Subscription {
            user_id: user_id.to_string(),
            id: self.id,
            url: self.url,
            title: self.title,
            description: self.description,
            feed_url: self.feed_url,
            joined_categories: self
                .categories
                .iter()
                .map(|category| category.label)
                .collect::<Vec<String>>()
                .join(","),
            last_fetch_ms: 0,
        }
    }
}

#[rocket::async_trait]
pub trait SubscriptionService {
    async fn add_subscription(&self, user_id: &str, subscription: Subscription) -> Result<()>;

    async fn remove_subscription(&self, user_id: &str, id: &str) -> Result<()>;

    async fn list_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>>;
}

struct SubscriptionServiceImpl {
    subscription_repository: Box<dyn SubscriptionRepository + Send + Sync>,
}

#[rocket::async_trait]
impl SubscriptionService for SubscriptionServiceImpl {
    async fn add_subscription(&self, user_id: &str, subscription: Subscription) -> Result<()> {
        self.subscription_repository
            .insert_subscription(subscription.to_db(user_id))
            .await?;
        Ok(())
    }

    async fn remove_subscription(&self, user_id: &str, id: &str) -> Result<()> {
        self.subscription_repository
            .remove_subscription(user_id, id)
            .await?;
        Ok(())
    }

    async fn list_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>> {
        let subscriptions = self
            .subscription_repository
            .list_user_subscriptions(user_id)
            .await?;
        return Ok(subscriptions
            .into_iter()
            .map(|sub| Subscription::from(sub))
            .collect());
    }
}

pub async fn new_subscription_service() -> Box<dyn SubscriptionService + Send + Sync> {
    let repository = new_subscription_repository("sqlite::memory:")
        .await
        .unwrap();
    Box::new(SubscriptionServiceImpl {
        subscription_repository: repository,
    })
}