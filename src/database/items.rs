use crate::common::{Page, PageOption};

#[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
pub struct Item {}

#[rocket::async_trait]
pub trait ItemRepository {
  async fn get_items(&self, user_id: &str, page_option: PageOption<String>) -> Result<Page<Item>>;

  async fn get_unread_items(
    &self,
    user_id: &str,
    page_option: PageOption<String>,
  ) -> Result<Page<Item>>;

  async fn get_starred_items(
    &self,
    user_id: &str,
    page_option: PageOption<String>,
  ) -> Result<Page<Item>>;
}