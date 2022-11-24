use crate::common::error::Errors;
use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(sqlx::FromRow, PartialEq, Eq, Debug, Clone)]
pub struct Subscription {
    pub user_id: String,
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub feed_url: String,
    pub joined_categories: String,
    pub last_fetch_ms: i64,
}

impl Subscription {
    pub fn categories(&self) -> Vec<&str> {
        if self.joined_categories.is_empty() {
            vec![]
        } else {
            self.joined_categories
                .split(',')
                .filter(|cat| !cat.is_empty())
                .collect()
        }
    }

    pub fn add_categories(&mut self, to_add: &Vec<&str>) {
        let mut categories: Vec<&str> = self.joined_categories.split(',').collect();
        for c in to_add {
            if !c.is_empty() {
                categories.push(c)
            }
        }
        categories.dedup();
        self.joined_categories = categories.join(",");
    }

    pub fn remove_categories(&mut self, to_remove: &Vec<&str>) {
        let mut categories: HashSet<&str> = self.joined_categories.split(',').collect();
        for c in to_remove {
            categories.remove(c);
        }
        self.joined_categories = categories.into_iter().collect::<Vec<&str>>().join(",");
    }
}

#[async_trait]
pub trait SubscriptionRepository {
    async fn insert_subscription(&self, subscription: Subscription) -> Result<()>;
    async fn update_subscription(&self, subscription: Subscription) -> Result<Subscription>;
    async fn remove_subscription(&self, user_id: &str, id: &str) -> Result<()>;
    async fn get_subscription(&self, user_id: &str, id: &str) -> Result<Option<Subscription>>;
    async fn get_subscriptions(
        &self,
        user_id: &str,
        ids: &Vec<&str>,
    ) -> Result<HashMap<String, Subscription>>;
    async fn list_user_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>>;
    async fn list_all_subscriptions(&self) -> Result<Vec<Subscription>>;
}

struct SubscriptionRepositorySqlite {
    pool: SqlitePool,
}

unsafe impl Send for SubscriptionRepositorySqlite {}
unsafe impl Sync for SubscriptionRepositorySqlite {}

impl SubscriptionRepositorySqlite {
    pub async fn new(pool: SqlitePool) -> Result<SubscriptionRepositorySqlite> {
        Ok(SubscriptionRepositorySqlite { pool })
    }
}

#[async_trait]
impl SubscriptionRepository for SubscriptionRepositorySqlite {
    async fn insert_subscription(&self, subscription: Subscription) -> Result<()> {
        sqlx::query(
            "INSERT INTO Subscriptions 
      (user_id, id, url, title, description, feed_url, joined_categories, last_fetch_ms)
      VALUES (?,?,?,?,?,?,?,?)",
        )
        .bind(&subscription.user_id)
        .bind(&subscription.id)
        .bind(&subscription.url)
        .bind(&subscription.title)
        .bind(&subscription.description)
        .bind(&subscription.feed_url)
        .bind(&subscription.joined_categories)
        .bind(subscription.last_fetch_ms)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_subscription(&self, subscription: Subscription) -> Result<Subscription> {
        if self
            .get_subscription(&subscription.user_id, &subscription.id)
            .await?
            .is_none()
        {
            return Err(Error::new(Errors::SubscriptionNotFound));
        }
        sqlx::query(
            "UPDATE Subscriptions SET
        url = ?,
        title = ?,
        description = ?,
        feed_url = ?,
        joined_categories = ?,
        last_fetch_ms = ?
        WHERE user_id = ? AND id = ?",
        )
        .bind(&subscription.url)
        .bind(&subscription.title)
        .bind(&subscription.description)
        .bind(&subscription.feed_url)
        .bind(&subscription.joined_categories)
        .bind(subscription.last_fetch_ms)
        .bind(&subscription.user_id)
        .bind(&subscription.id)
        .execute(&self.pool)
        .await?;
        Ok(subscription)
    }

    async fn remove_subscription(&self, user_id: &str, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM Subscriptions WHERE user_id = ? AND id = ?")
            .bind(user_id)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_subscription(&self, user_id: &str, id: &str) -> Result<Option<Subscription>> {
        let subscription_opt = sqlx::query_as::<_, Subscription>(
            "SELECT * FROM Subscriptions WHERE user_id = ? AND id = ?",
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(subscription_opt)
    }

    async fn get_subscriptions(
        &self,
        user_id: &str,
        ids: &Vec<&str>,
    ) -> Result<HashMap<String, Subscription>> {
        let query_str = format!(
            "SELECT * FROM Subscriptions WHERE user_id = ? AND ({})",
            ids.iter()
                .map(|_| "id = ?")
                .collect::<Vec<&str>>()
                .join(" OR ")
        );
        let mut query = sqlx::query_as::<_, Subscription>(&query_str).bind(user_id);
        for id in ids {
            query = query.bind(id);
        }
        let subscriptions = query.fetch_all(&self.pool).await?;
        Ok(subscriptions
            .into_iter()
            .map(|sub| (sub.id.clone(), sub))
            .collect())
    }

    async fn list_user_subscriptions(&self, user_id: &str) -> Result<Vec<Subscription>> {
        let subscriptions =
            sqlx::query_as::<_, Subscription>("SELECT * FROM Subscriptions WHERE user_id = ?")
                .bind(user_id)
                .fetch_all(&self.pool)
                .await?;
        Ok(subscriptions)
    }

    async fn list_all_subscriptions(&self) -> Result<Vec<Subscription>> {
        let subscriptions = sqlx::query_as::<_, Subscription>("SELECT * FROM Subscriptions")
            .fetch_all(&self.pool)
            .await?;
        Ok(subscriptions)
    }
}

pub async fn new_subscription_repository(
    pool: SqlitePool,
) -> Result<Box<dyn SubscriptionRepository + Send + Sync>> {
    let repository = SubscriptionRepositorySqlite::new(pool).await?;
    Ok(Box::new(repository))
}

#[cfg(test)]
mod tests {
    use super::super::in_memory_pool;
    use super::*;

    #[rocket::async_test]
    pub async fn insert_and_get_subscription_should_succeed() {
        let repository = new_subscription_repository(in_memory_pool().await)
            .await
            .unwrap();
        let subscription = Subscription {
            user_id: "user_id".to_string(),
            id: "id".to_string(),
            url: "url".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            feed_url: "feed_url".to_string(),
            joined_categories: "joined_categories".to_string(),
            last_fetch_ms: 0,
        };
        repository
            .insert_subscription(subscription.clone())
            .await
            .unwrap();
        assert_eq!(
            repository
                .get_subscription(&subscription.user_id, &subscription.id)
                .await
                .unwrap()
                .unwrap(),
            subscription
        );
    }

    #[rocket::async_test]
    pub async fn remove_subscription_should_succeed() {
        let repository = new_subscription_repository(in_memory_pool().await)
            .await
            .unwrap();
        let subscription = Subscription {
            user_id: "user_id".to_string(),
            id: "id".to_string(),
            url: "url".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            feed_url: "feed_url".to_string(),
            joined_categories: "joined_categories".to_string(),
            last_fetch_ms: 0,
        };
        repository
            .insert_subscription(subscription.clone())
            .await
            .unwrap();
        repository
            .remove_subscription(&subscription.user_id, &subscription.id)
            .await
            .unwrap();
        assert!(repository
            .get_subscription(&subscription.user_id, &subscription.id)
            .await
            .unwrap()
            .is_none());
    }

    #[rocket::async_test]
    pub async fn update_subscription_should_succeed() {
        let repository = new_subscription_repository(in_memory_pool().await)
            .await
            .unwrap();
        let subscription = Subscription {
            user_id: "user_id".to_string(),
            id: "id".to_string(),
            url: "url".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            feed_url: "feed_url".to_string(),
            joined_categories: "joined_categories".to_string(),
            last_fetch_ms: 0,
        };
        repository
            .insert_subscription(subscription.clone())
            .await
            .unwrap();

        let mut updated_subscription = subscription.clone();
        updated_subscription.url = "url_2".to_string();
        updated_subscription.title = "title_2".to_string();
        updated_subscription.description = "description_2".to_string();
        updated_subscription.feed_url = "feed_url_2".to_string();
        updated_subscription.joined_categories = "joined_categories_2".to_string();
        updated_subscription.last_fetch_ms = 1;
        repository
            .update_subscription(updated_subscription.clone())
            .await
            .unwrap();
        assert_eq!(
            repository
                .get_subscription(&subscription.user_id, &subscription.id)
                .await
                .unwrap()
                .unwrap(),
            updated_subscription
        );
    }

    #[rocket::async_test]
    pub async fn list_subscriptions_should_succeed() {
        let repository = new_subscription_repository(in_memory_pool().await)
            .await
            .unwrap();
        let subscription_1 = Subscription {
            user_id: "user_id".to_string(),
            id: "id".to_string(),
            url: "url".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            feed_url: "feed_url".to_string(),
            joined_categories: "joined_categories".to_string(),
            last_fetch_ms: 0,
        };
        repository
            .insert_subscription(subscription_1.clone())
            .await
            .unwrap();

        let mut subscription_2 = subscription_1.clone();
        subscription_2.id = "id_2".to_string();
        subscription_2.url = "url_2".to_string();
        subscription_2.title = "title_2".to_string();
        subscription_2.description = "description_2".to_string();
        subscription_2.feed_url = "feed_url_2".to_string();
        subscription_2.joined_categories = "joined_categories_2".to_string();
        subscription_2.last_fetch_ms = 1;
        repository
            .insert_subscription(subscription_2.clone())
            .await
            .unwrap();

        let all_subscriptions = repository
            .list_user_subscriptions(&subscription_1.user_id)
            .await
            .unwrap();
        assert_eq!(all_subscriptions.len(), 2);
        assert!(all_subscriptions.contains(&subscription_1));
        assert!(all_subscriptions.contains(&subscription_2));
    }
}
