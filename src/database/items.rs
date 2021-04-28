use crate::common::{Page, PageOption};
use anyhow::Result;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
pub struct ItemId {
    user_id: String,
    subscription_id: String,
    id: String,
}

#[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
pub struct Item {
    user_id: String,
    subscription_id: String,
    id: String,
    title: String,
    content: String,
    author: String,
    url: String,
    created_at_ms: i64,
    fetched_at_ms: i64,
    starred: bool,
    read: bool,
}

impl Item {
    fn parse_offset(offset: &str) -> (i64, &str) {
        let parts: Vec<&str> = offset.split("-").collect();
        (parts[0].parse::<i64>().unwrap(), parts[1])
    }

    fn as_offset(&self) -> String {
        format!("{}-{}", self.created_at_ms, self.id)
    }
}

pub enum State {
    STARRED,
    UNSTARRED,
    READ,
    UNREAD,
}

impl State {
    fn column(&self) -> &'static str {
        match self {
            Self::STARRED | Self::UNSTARRED => "starred",
            Self::READ | Self::UNREAD => "read",
        }
    }

    fn value(&self) -> bool {
        match self {
            Self::STARRED | Self::READ => true,
            Self::UNSTARRED | Self::UNREAD => true,
        }
    }
}

#[rocket::async_trait]
pub trait ItemRepository {
    async fn get_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>>;

    async fn get_unread_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>>;

    async fn get_starred_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>>;

    async fn insert_items(&self, items: Vec<Item>) -> Result<()>;

    async fn delete_items(&self, user_id: &str, earlier_than: i64) -> Result<()>;

    async fn mark_as(&self, item_id: ItemId, state: State) -> Result<()>;

    async fn mark_all_as_read(&self, user_id: &str) -> Result<()>;

    async fn mark_older_as_read(&self, user_id: &str, older_than: i64) -> Result<()>;
}

struct ItemRepositorySqlite {
    pool: SqlitePool,
}

unsafe impl Send for ItemRepositorySqlite {}
unsafe impl Sync for ItemRepositorySqlite {}

impl ItemRepositorySqlite {
    pub async fn new(pool: SqlitePool) -> Result<ItemRepositorySqlite> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS Items (
      user_id TEXT NOT NULL,
      subscription_id TEXT NOT NULL,
      id TEXT NOT NULL,
      title TEXT NOT NULL,
      content TEXT NOT NULL,
      author TEXT NOT NULL,
      url TEXT NOT NULL,
      created_at_ms INTEGER NOT NULL,
      fetched_at_ms INTEGER NOT NULL,
      starred BOOL NOT NULL,
      read BOOL NOT NULL)",
        )
        .execute(&pool)
        .await?;
        Ok(ItemRepositorySqlite { pool: pool })
    }

    fn build_page_query(page_option: &PageOption<String>) -> String {
        let operator = if page_option.desc { ">=" } else { "<=" };
        let pagination = match page_option.offset {
            Some(ref offset) => {
                let (created_at_ms, id) = Item::parse_offset(offset);
                format!(
                    "AND created_at_ms {} {} AND id {} {}",
                    operator, created_at_ms, operator, id
                )
            }
            None => String::new(),
        };
        let order_and_limit = format!(
            "ORDER BY created_at_ms, id {} LIMIT {}",
            if page_option.desc { "DESC" } else { "" },
            page_option.limit
        );
        format!("{} {}", pagination, order_and_limit)
    }

    async fn get_items_with_query(
        &self,
        query: &str,
        page_option: &PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let mut items = sqlx::query_as::<_, Item>(&query)
            .fetch_all(&self.pool)
            .await?;
        let next_page_offset = if items.len() > page_option.limit {
            let last_item = items.pop().unwrap();
            Some(last_item.as_offset())
        } else {
            None
        };
        Ok(Page::<Item, String> {
            items: items,
            next_page_offset: next_page_offset,
        })
    }
}

#[rocket::async_trait]
impl ItemRepository for ItemRepositorySqlite {
    async fn get_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = {} {}",
            user_id,
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(&query, &page_option).await
    }

    async fn get_unread_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = {} AND read = false {}",
            user_id,
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(&query, &page_option).await
    }

    async fn get_starred_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = {} AND starred = true {}",
            user_id,
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(&query, &page_option).await
    }

    async fn insert_items(&self, items: Vec<Item>) -> Result<()> {
        let mut query = String::from("
    INSERT INTO Items 
    (user_id, subscription_id, id, title, content, author, url, created_at_ms, fetched_at_ms, starred, read)
    VALUES ");
        for item in items {
            query.push_str(&format!(
                "({},{},{},{},{},{},{},{},{},{},{})",
                item.user_id,
                item.subscription_id,
                item.id,
                item.title,
                item.content,
                item.author,
                item.url,
                item.created_at_ms,
                item.fetched_at_ms,
                item.starred,
                item.read
            ));
        }
        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_items(&self, user_id: &str, earlier_than: i64) -> Result<()> {
        sqlx::query("DELETE FROM Items WHERE user_id = ? AND created_at_ms <= ?")
            .bind(user_id)
            .bind(earlier_than)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_as(&self, item_id: ItemId, state: State) -> Result<()> {
        let query = format!(
            "UPDATE Items SET {} = {} WHERE user_id = ?, subscription_id = ?, id = ?",
            state.column(),
            state.value()
        );
        sqlx::query(&query)
            .bind(&item_id.user_id)
            .bind(&item_id.subscription_id)
            .bind(&item_id.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: &str) -> Result<()> {
        sqlx::query("UPDATE Items SET read = true WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_older_as_read(&self, user_id: &str, older_than: i64) -> Result<()> {
        sqlx::query("UPDATE Items SET read = true WHERE user_id = ? AND created_at_ms <= ?")
            .bind(user_id)
            .bind(older_than)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
