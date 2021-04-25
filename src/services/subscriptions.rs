use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub label: String
}

#[derive(Serialize)]
pub struct Subscription {
    pub id: String,
    pub title: String,
    pub categories: Vec<Category>,
    pub url: String,
}

#[rocket::async_trait]
pub trait SubscriptionService {
    async fn list_subscriptions(&self, user_id: &str, offset: usize, limit: usize) -> Vec<Subscription>;
}

struct FakeSubscriptionService {}

#[rocket::async_trait]
impl SubscriptionService for FakeSubscriptionService {
    async fn list_subscriptions(&self, user_id: &str, offset: usize, limit: usize) -> Vec<Subscription> {
        return vec![
            Subscription {
                id: "feed/http://www.daemonology.net/hn-daily/index.rss".to_string(),
                title: "Hacker News Daily".to_string(),
                categories: vec![],
                url: "https://www.daemonology.net/hn-daily/".to_string()
            }
        ];
    }
}

pub fn new_subscription_service() -> Box<dyn SubscriptionService + Send + Sync> {
    Box::new(FakeSubscriptionService {})
}