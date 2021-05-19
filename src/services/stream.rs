use crate::common::{Page, PageOption};
use crate::database::items::{Item, ItemRepository, State};
use crate::database::subscriptions::{Subscription, SubscriptionRepository};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemId {
    pub id: String,
    pub direct_stream_ids: Vec<String>,
    pub timestamp_usec: String,
}

impl From<Item> for ItemId {
    fn from(item: Item) -> ItemId {
        ItemId {
            id: item.id.to_string(),
            // id: format!("tag:google.com,2005:reader/item/{:016x}", item.id),
            direct_stream_ids: vec![],
            timestamp_usec: (item.created_at_ms * 1000).to_string(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    href: String,
    #[serde(rename = "type")]
    type_f: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    direction: String,
    content: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    stream_id: String,
    title: String,
    html_url: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemContent {
    pub crawl_time_msec: String,
    pub timestamp_usec: String,
    pub id: String,
    pub categories: Vec<String>,
    // Seconds
    pub published: i64,
    // Seconds
    pub updated: i64,
    pub canonical: Vec<Url>,
    pub alternate: Vec<Url>,
    pub summary: Summary,
    pub title: String,
    pub author: String,
    pub origin: Origin,
}

impl ItemContent {
    fn from(item: Item, sub: &Subscription) -> ItemContent {
        ItemContent {
            crawl_time_msec: item.fetched_at_ms.to_string(),
            timestamp_usec: (item.created_at_ms * 1000).to_string(),
            id: format!("tag:google.com,2005:reader/item/{:016x}", item.id),
            categories: item.categories(),
            published: item.created_at_ms / 1000,
            updated: item.created_at_ms / 1000,
            canonical: vec![Url {
                href: item.url.clone(),
                type_f: Option::None,
            }],
            alternate: vec![Url {
                href: item.url,
                type_f: Option::Some("text/html".to_owned()),
            }],
            summary: Summary {
                direction: String::from("ltr"),
                content: item.content,
            },
            title: item.title,
            author: item.author,
            origin: Origin {
                stream_id: item.subscription_id,
                title: sub.title.to_owned(),
                html_url: sub.url.to_owned(),
            },
        }
    }
}

#[rocket::async_trait]
pub trait StreamService {
    async fn get_unread_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>>;

    async fn get_unread_item_contents(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemContent, String>>;

    async fn get_read_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>>;

    async fn get_starred_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>>;

    async fn get_all_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>>;

    async fn get_item_contents(&self, user_id: &str, ids: &Vec<&str>) -> Result<Vec<ItemContent>>;

    async fn mark_as_read(&self, user_id: &str, ids: &Vec<&str>) -> Result<()>;

    async fn mark_as_unread(&self, user_id: &str, ids: &Vec<&str>) -> Result<()>;

    async fn mark_as_starred(&self, user_id: &str, ids: &Vec<&str>) -> Result<()>;

    async fn mark_as_unstarred(&self, user_id: &str, ids: &Vec<&str>) -> Result<()>;
}

struct StreamServiceImpl {
    item_repository: Box<dyn ItemRepository + Send + Sync>,
    subscription_repository: Box<dyn SubscriptionRepository + Send + Sync>,
}

#[rocket::async_trait]
impl StreamService for StreamServiceImpl {
    async fn get_unread_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>> {
        let page = self
            .item_repository
            .get_unread_items(user_id, page_option)
            .await?;
        Ok(page.convert::<ItemId, _>(|item| ItemId::from(item)))
    }

    async fn get_unread_item_contents(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemContent, String>> {
        let page = self
            .item_repository
            .get_unread_items(user_id, page_option)
            .await?;
        if page.items.len() == 0 {
          return Ok(Page::empty());
        }
        let subscription_ids: Vec<&str> = page
            .items
            .iter()
            .map(|item| &*item.subscription_id)
            .collect();
        let subscriptions = self
            .subscription_repository
            .get_subscriptions(user_id, &subscription_ids)
            .await?;
        Ok(page.convert::<ItemContent, _>(|item| {
            let subscription = subscriptions.get(&item.subscription_id).unwrap();
            ItemContent::from(item, subscription)
        }))
    }

    async fn get_read_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>> {
        let page = self
            .item_repository
            .get_read_items(user_id, page_option)
            .await?;
        Ok(page.convert::<ItemId, _>(|item| ItemId::from(item)))
    }

    async fn get_starred_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>> {
        let page = self
            .item_repository
            .get_starred_items(user_id, page_option)
            .await?;
        Ok(page.convert::<ItemId, _>(|item| ItemId::from(item)))
    }

    async fn get_all_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>> {
        let page = self.item_repository.get_items(user_id, page_option).await?;
        Ok(page.convert::<ItemId, _>(|item| ItemId::from(item)))
    }

    async fn get_item_contents(&self, user_id: &str, ids: &Vec<&str>) -> Result<Vec<ItemContent>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let items = self.item_repository.get_items_by_id(user_id, ids).await?;
        let subscription_ids: Vec<&str> = items.iter().map(|item| &*item.subscription_id).collect();
        let subscriptions = self
            .subscription_repository
            .get_subscriptions(user_id, &subscription_ids)
            .await?;
        Ok(items
            .into_iter()
            .map(|item| {
                let subscription = subscriptions.get(&item.subscription_id).unwrap();
                ItemContent::from(item, subscription)
            })
            .collect())
    }

    async fn mark_as_read(&self, user_id: &str, ids: &Vec<&str>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        self.item_repository
            .mark_items_as(user_id, ids, State::READ)
            .await
    }

    async fn mark_as_unread(&self, user_id: &str, ids: &Vec<&str>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        self.item_repository
            .mark_items_as(user_id, ids, State::UNREAD)
            .await
    }

    async fn mark_as_starred(&self, user_id: &str, ids: &Vec<&str>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        self.item_repository
            .mark_items_as(user_id, ids, State::STARRED)
            .await
    }

    async fn mark_as_unstarred(&self, user_id: &str, ids: &Vec<&str>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        self.item_repository
            .mark_items_as(user_id, ids, State::UNSTARRED)
            .await
    }
}

pub fn new_stream_service(
    item_repository: Box<dyn ItemRepository + Send + Sync>,
    subscription_repository: Box<dyn SubscriptionRepository + Send + Sync>,
) -> Box<dyn StreamService + Send + Sync> {
    Box::new(StreamServiceImpl {
        item_repository: item_repository,
        subscription_repository: subscription_repository,
    })
}
