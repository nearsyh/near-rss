use crate::common::{Page, PageOption};

pub struct Item {}

#[rocket::async_trait]
pub trait StreamService {
    async fn get_unread_items(
        &self,
        user_id: &str,
        page_option: PageOption<i64>,
    ) -> Page<Item, i64>;
}

struct StreamServiceImpl {}

#[rocket::async_trait]
impl StreamService for StreamServiceImpl {
    async fn get_unread_items(
        &self,
        _user_id: &str,
        _page_option: PageOption<i64>,
    ) -> Page<Item, i64> {
        Page::<Item, i64> {
            items: vec![],
            next_page_offset: Option::None,
        }
    }
}
