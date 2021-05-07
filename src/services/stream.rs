use crate::common::{Page, PageOption};
use crate::database::items::{Item, ItemRepository};
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
            direct_stream_ids: vec![],
            timestamp_usec: (item.created_at_ms * 1000).to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    href: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    direction: String,
    content: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    stream_id: String,
    title: String,
    html_url: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemContent {
    crawl_time_msec: i64,
    timestamp_usec: i64,
    id: String,
    categories: Vec<String>,
    // Seconds
    published: i64,
    // Seconds
    updated: i64,
    canonical: Url,
    summary: Summary,
    title: String,
    author: String,
    origin: Origin,
}

impl From<Item> for ItemContent {
    fn from(item: Item) -> ItemContent {
        ItemContent {
            crawl_time_msec: item.fetched_at_ms,
            timestamp_usec: item.fetched_at_ms * 1000,
            id: item.id.to_string(),
            categories: vec![],
            published: item.created_at_ms / 1000,
            updated: item.created_at_ms / 1000,
            canonical: Url { href: item.url },
            summary: Summary {
                direction: String::from("ltr"),
                content: item.content,
            },
            title: item.title,
            author: item.author,
            origin: Origin {
                stream_id: item.subscription_id,
                title: "".to_owned(),
                html_url: "".to_owned(),
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

    async fn get_item_contents(&self, user_id: &str, ids: Vec<String>) -> Result<Vec<ItemContent>>;
}

struct StreamServiceImpl {
    item_repository: Box<dyn ItemRepository + Send + Sync>,
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

    async fn get_item_contents(&self, user_id: &str, ids: Vec<String>) -> Result<Vec<ItemContent>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let items = self.item_repository.get_items_by_id(user_id, ids).await?;
        Ok(items
            .into_iter()
            .map(|item| ItemContent::from(item))
            .collect())
    }
}

pub fn new_stream_service(
    repository: Box<dyn ItemRepository + Send + Sync>,
) -> Box<dyn StreamService + Send + Sync> {
    Box::new(StreamServiceImpl {
        item_repository: repository,
    })
}
