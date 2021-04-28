use crate::common::current_time_ms;
use crate::database::items::{Item, ItemRepository};
use crate::database::subscriptions::SubscriptionRepository;
use crate::services::feeds::{new_feed_service, FeedService};
use anyhow::Result;
use feed_rs::model::Feed;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Category {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Clone)]
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
        let categories = db_subscription
            .categories()
            .into_iter()
            .map(|category_str| Category {
                id: format!("user/{}/label/{}", db_subscription.user_id, category_str),
                label: category_str.to_string(),
            })
            .collect();
        Subscription {
            id: db_subscription.id,
            title: db_subscription.title,
            description: db_subscription.description,
            url: db_subscription.url,
            feed_url: db_subscription.feed_url,
            categories: categories,
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
                .map(|category| category.label.clone())
                .collect::<Vec<String>>()
                .join(","),
            last_fetch_ms: 0,
        }
    }

    fn from_feed(url: &str, feed: Feed) -> Subscription {
        Subscription {
            id: format!("feed/{}", url),
            title: feed.title.map_or(String::new(), |t| t.content),
            description: feed.description.map_or(String::new(), |t| t.content),
            categories: vec![],
            url: if feed.links.is_empty() {
                url.to_string()
            } else {
                feed.links[0].href.clone()
            },
            feed_url: url.to_string(),
        }
    }
}

#[rocket::async_trait]
pub trait SubscriptionService {
    async fn get_subscription_from_url(&self, url: &str) -> Result<Subscription>;

    async fn add_subscription_from_url(&self, user_id: &str, url: &str) -> Result<Subscription>;

    async fn add_subscription(&self, user_id: &str, subscription: Subscription) -> Result<()>;

    async fn remove_subscription(&self, user_id: &str, id: &str) -> Result<()>;

    async fn list_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>>;

    async fn load_subscription_items(&self, user_id: &str) -> Result<()>;
}

struct SubscriptionServiceImpl {
    subscription_repository: Box<dyn SubscriptionRepository + Send + Sync>,
    item_repository: Box<dyn ItemRepository + Send + Sync>,
    feed_service: Box<dyn FeedService + Send + Sync>,
}

fn extract_items_from_feed(user_id: &str, subscription_id: &str, feed: &Feed) -> Vec<Item> {
    feed.entries
        .iter()
        .map(|entry| {
            Item::new_item(
                user_id,
                subscription_id,
                &entry.id,
                &entry.title.as_ref().map_or("", |t| &t.content),
                &entry
                    .content
                    .as_ref()
                    .map_or("", |t| t.body.as_deref().unwrap_or("")),
                &entry
                    .authors
                    .iter()
                    .map(|p| -> &str { &p.name })
                    .collect::<Vec<&str>>()
                    .join(","),
                &entry.links[0].href,
                entry
                    .published
                    .map_or(current_time_ms(), |d| d.timestamp_millis()),
            )
        })
        .collect()
}

#[rocket::async_trait]
impl SubscriptionService for SubscriptionServiceImpl {
    async fn get_subscription_from_url(&self, url: &str) -> Result<Subscription> {
        let feed = self.feed_service.get_feed(url).await?;
        Ok(Subscription::from_feed(url, feed))
    }

    async fn add_subscription_from_url(&self, user_id: &str, url: &str) -> Result<Subscription> {
        let subscription = self.get_subscription_from_url(url).await?;
        self.add_subscription(user_id, subscription.clone()).await?;
        Ok(subscription)
    }

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

    async fn load_subscription_items(&self, user_id: &str) -> Result<()> {
        let subscriptions = self
            .subscription_repository
            .list_user_subscriptions(user_id)
            .await?;
        let urls = subscriptions
            .iter()
            .map(|sub| -> &str { &sub.feed_url })
            .collect::<Vec<&str>>();
        let feeds = self.feed_service.get_feeds(urls).await;
        for subscription in subscriptions {
            let url = &subscription.feed_url;
            match feeds.get(url) {
                Some(Ok(feed)) => {
                    self.item_repository
                        .insert_items(extract_items_from_feed(user_id, &subscription.id, feed))
                        .await?;
                }
                _ => continue,
            };
        }
        Ok(())
    }
}

pub fn new_subscription_service(
    subscription_repository: Box<dyn SubscriptionRepository + Send + Sync>,
    item_repository: Box<dyn ItemRepository + Send + Sync>,
) -> Box<dyn SubscriptionService + Send + Sync> {
    Box::new(SubscriptionServiceImpl {
        subscription_repository: subscription_repository,
        item_repository: item_repository,
        feed_service: new_feed_service(),
    })
}
