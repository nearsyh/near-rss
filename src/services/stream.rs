use crate::common::{Page, PageOption};
use crate::database::items::ItemRepository;
use super::feeds::{new_feed_service, FeedService};
use serde::Serialize;
use anyhow::Result;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemId {
    id: String,
    direct_stream_ids: Vec<String>,
    timestamp_usec: i64
}

impl From<crate::database::items::Item> for ItemId {
    fn from(item: crate::database::items::Item) -> ItemId {
        ItemId {
            id: item.id,
            direct_stream_ids: vec![],
            timestamp_usec: item.created_at_ms * 1000
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
}

struct StreamServiceImpl {
    item_repository: Box<dyn ItemRepository + Send + Sync>,
    feed_service: Box<dyn FeedService + Send + Sync>,
}

#[rocket::async_trait]
impl StreamService for StreamServiceImpl {
    async fn get_unread_item_ids(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<ItemId, String>> {
        let page = self.item_repository.get_items(user_id, page_option).await?;
        Ok(Page::<ItemId, String> {
            items: page.items.into_iter().map(|item| {
                ItemId::from(item)
            }).collect(),
            next_page_offset: page.next_page_offset,
        })
    }
}

pub fn new_stream_service(
    repository: Box<dyn ItemRepository + Send + Sync>,
) -> Box<dyn StreamService + Send + Sync> {
    Box::new(StreamServiceImpl {
        item_repository: repository,
        feed_service: new_feed_service(),
    })
}
