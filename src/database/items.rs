use crate::common::{current_time_ms, Page, PageOption};
use anyhow::Result;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
pub struct ItemId {
    user_id: String,
    subscription_id: String,
    id: i64,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Item {
    pub user_id: String,
    pub subscription_id: String,
    pub external_id: String,
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author: String,
    pub url: String,
    pub created_at_ms: i64,
    pub fetched_at_ms: i64,
    pub starred: bool,
    pub read: bool,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        return self.subscription_id == other.subscription_id
            && self.external_id == other.external_id
            && self.user_id == other.user_id;
    }
}

impl Eq for Item {}

impl Item {
    fn parse_offset(offset: &str) -> (i64, i64) {
        let parts: Vec<&str> = offset.split("-").collect();
        (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        )
    }

    fn as_offset(&self) -> String {
        format!("{}-{}", self.created_at_ms, self.id)
    }

    pub fn new_item(
        user_id: &str,
        subscription_id: &str,
        external_id: &str,
        title: &str,
        content: &str,
        author: &str,
        url: &str,
        created_at_ms: i64,
    ) -> Item {
        Item {
            user_id: user_id.to_owned(),
            subscription_id: subscription_id.to_owned(),
            external_id: external_id.to_owned(),
            id: 0,
            title: title.to_owned(),
            content: content.to_owned(),
            author: author.to_owned(),
            url: url.to_owned(),
            created_at_ms: created_at_ms,
            fetched_at_ms: current_time_ms(),
            starred: false,
            read: false,
        }
    }

    pub fn categories(&self) -> Vec<String> {
        let mut ret = vec![];
        ret.push("user/-/state/com.google/reading-list".to_owned());
        if self.read {
            ret.push("user/-/state/com.google/read".to_owned());
        } else {
            ret.push("user/-/state/com.google/fresh".to_owned());
        }
        if self.starred {
            ret.push("user/-/state/com.google/starred".to_owned());
        }
        ret
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
            Self::UNSTARRED | Self::UNREAD => false,
        }
    }
}

#[rocket::async_trait]
pub trait ItemRepository {
    async fn get_items_by_id(&self, user_id: &str, ids: &Vec<&str>) -> Result<Vec<Item>>;

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

    async fn get_read_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>>;

    async fn get_starred_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>>;

    async fn insert_items(&self, mut items: Vec<Item>) -> Result<()>;

    async fn delete_items(&self, user_id: &str, earlier_than: i64) -> Result<()>;

    async fn mark_as(&self, item_id: ItemId, state: State) -> Result<()>;

    async fn mark_items_as(&self, user_id: &str, ids: &Vec<&str>, state: State) -> Result<()>;

    async fn mark_all_as_read(&self, user_id: &str) -> Result<()>;

    async fn mark_older_as_read(&self, user_id: &str, older_than: i64) -> Result<()>;
}

fn id_str_to_i64(id: &str) -> i64 {
    if id.starts_with("tag:google.com,2005:reader/item/") {
        let id_hex = id.strip_prefix("tag:google.com,2005:reader/item/").unwrap();
        i64::from_str_radix(id_hex, 16).unwrap_or(-1)
    } else {
        id.parse::<i64>().unwrap_or(-1)
    }
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
      id INTEGER PRIMARY KEY,
      user_id TEXT NOT NULL,
      subscription_id TEXT NOT NULL,
      external_id TEXT NOT NULL,
      title TEXT NOT NULL,
      content TEXT NOT NULL,
      author TEXT NOT NULL,
      url TEXT NOT NULL,
      created_at_ms INTEGER NOT NULL,
      fetched_at_ms INTEGER NOT NULL,
      starred BOOL NOT NULL,
      read BOOL NOT NULL,
      UNIQUE(user_id, subscription_id, external_id))",
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
                    "AND created_at_ms {} {} AND id {} \"{}\"",
                    operator, created_at_ms, operator, id
                )
            }
            None => String::new(),
        };
        let order_and_limit = format!(
            "ORDER BY created_at_ms {}, id {} LIMIT {}",
            if page_option.desc { "DESC" } else { "" },
            if page_option.desc { "DESC" } else { "" },
            page_option.limit + 1
        );
        format!("{} {}", pagination, order_and_limit)
    }

    async fn get_items_with_query(
        &self,
        user_id: &str,
        query: String,
        page_option: &PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let mut items = sqlx::query_as::<_, Item>(&query)
            .bind(user_id)
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
    async fn get_items_by_id(&self, user_id: &str, ids: &Vec<&str>) -> Result<Vec<Item>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let base_query = String::from("SELECT * FROM Items WHERE user_id = ?");
        let conditions = ids
            .iter()
            .map(|_| " id = ? ")
            .collect::<Vec<&str>>()
            .join("OR");
        let query_str = format!("{} AND ({})", base_query, conditions);
        let mut query = sqlx::query_as::<_, Item>(&query_str).bind(user_id);
        for id in ids {
            query = query.bind(id_str_to_i64(id));
        }
        Ok(query.fetch_all(&self.pool).await?)
    }

    async fn get_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = ? {}",
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(user_id, query, &page_option)
            .await
    }

    async fn get_unread_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = ? AND read = false {}",
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(user_id, query, &page_option)
            .await
    }

    async fn get_read_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = ? AND read = true {}",
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(user_id, query, &page_option)
            .await
    }

    async fn get_starred_items(
        &self,
        user_id: &str,
        page_option: PageOption<String>,
    ) -> Result<Page<Item, String>> {
        let query = format!(
            "SELECT * FROM Items WHERE user_id = ? AND starred = true {}",
            Self::build_page_query(&page_option)
        );
        self.get_items_with_query(user_id, query, &page_option)
            .await
    }

    async fn insert_items(&self, mut items: Vec<Item>) -> Result<()> {
        let base = String::from("
    INSERT INTO Items 
    (user_id, subscription_id, external_id, title, content, author, url, created_at_ms, fetched_at_ms, starred, read)
    VALUES ");
        items.sort_by(|a, b| a.created_at_ms.cmp(&b.created_at_ms));
        let values = items
            .iter()
            .map(|_| "(?,?,?,?,?,?,?,?,?,?,?)")
            .collect::<Vec<&str>>()
            .join(",");
        let query_str = format!(
            "{}{} ON CONFLICT(user_id, subscription_id, external_id) DO NOTHING",
            base, values
        );
        let mut query = sqlx::query(&query_str);
        for item in items.iter() {
            query = query
                .bind(&item.user_id)
                .bind(&item.subscription_id)
                .bind(&item.external_id)
                .bind(&item.title)
                .bind(item.content.trim().to_owned())
                .bind(&item.author)
                .bind(&item.url)
                .bind(item.created_at_ms)
                .bind(item.fetched_at_ms)
                .bind(item.starred)
                .bind(item.read)
        }
        query.execute(&self.pool).await?;
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
            "UPDATE Items SET {} = ? WHERE user_id = ? AND subscription_id = ? AND id = ?",
            state.column()
        );
        sqlx::query(&query)
            .bind(state.value())
            .bind(&item_id.user_id)
            .bind(&item_id.subscription_id)
            .bind(&item_id.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_items_as(&self, user_id: &str, ids: &Vec<&str>, state: State) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        let conditions = ids
            .iter()
            .map(|_| " id = ? ")
            .collect::<Vec<&str>>()
            .join("OR");
        let query_str = format!(
            "UPDATE Items SET {} = ? WHERE user_id = ? AND {}",
            state.column(),
            conditions
        );
        let mut query = sqlx::query(&query_str).bind(state.value()).bind(user_id);
        for id in ids {
            query = query.bind(id_str_to_i64(id));
        }
        query.execute(&self.pool).await?;
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

pub async fn new_item_repository(
    pool: SqlitePool,
) -> Result<Box<dyn ItemRepository + Send + Sync>> {
    let repository = ItemRepositorySqlite::new(pool).await?;
    Ok(Box::new(repository))
}

#[cfg(test)]
mod tests {
    use super::super::in_memory_pool;
    use super::*;

    fn new_fake_item(external_id: &str, created_at_ms: i64) -> Item {
        Item::new_item(
            "user_id",
            "subscription_id",
            external_id,
            "title",
            "content",
            "author",
            "url",
            created_at_ms,
        )
    }

    #[rocket::async_test]
    pub async fn insert_items_should_succeed() {
        let repository = new_item_repository(in_memory_pool().await).await.unwrap();
        let items = vec![
            new_fake_item("1", 1),
            new_fake_item("2", 2),
            new_fake_item("3", 3),
        ];
        repository.insert_items(items.clone()).await.unwrap();
        let fetched_items = repository
            .get_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items;
        assert_eq!(fetched_items, items);
    }

    #[rocket::async_test]
    pub async fn insert_same_items_should_do_nothing() {
        let repository = new_item_repository(in_memory_pool().await).await.unwrap();
        let mut items = vec![new_fake_item("1", 1), new_fake_item("2", 2)];
        repository.insert_items(items.clone()).await.unwrap();
        // Insert same items again.
        repository
            .insert_items(vec![new_fake_item("1", 1), new_fake_item("3", 3)])
            .await
            .unwrap();
        items = repository
            .get_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items;
        assert_eq!(items.len(), 3);

        repository
            .mark_as(items[0].key(), State::READ)
            .await
            .unwrap();
        repository
            .insert_items(vec![new_fake_item("1", 1), new_fake_item("3", 3)])
            .await
            .unwrap();
        let item = &repository
            .get_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items[0];
        assert!(item.read);
    }

    #[rocket::async_test]
    pub async fn mark_all_as_read_should_succeed() {
        let repository = new_item_repository(in_memory_pool().await).await.unwrap();
        let items = vec![
            new_fake_item("1", 1),
            new_fake_item("2", 2),
            new_fake_item("3", 3),
        ];
        repository.insert_items(items.clone()).await.unwrap();
        repository.mark_all_as_read("user_id").await.unwrap();
        assert!(repository
            .get_unread_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items
            .is_empty());
    }

    #[rocket::async_test]
    pub async fn mark_older_as_read_should_succeed() {
        let repository = new_item_repository(in_memory_pool().await).await.unwrap();
        let items = vec![
            new_fake_item("1", 1),
            new_fake_item("2", 2),
            new_fake_item("3", 3),
        ];
        repository.insert_items(items.clone()).await.unwrap();
        repository.mark_older_as_read("user_id", 1).await.unwrap();
        assert_eq!(
            repository
                .get_unread_items("user_id", PageOption::<String>::new(10, false))
                .await
                .unwrap()
                .items,
            &items[1..]
        );
        repository.mark_older_as_read("user_id", 3).await.unwrap();
        assert!(repository
            .get_unread_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items
            .is_empty());
    }

    #[rocket::async_test]
    pub async fn mark_item_should_succeed() {
        let repository = new_item_repository(in_memory_pool().await).await.unwrap();
        let mut items = vec![
            new_fake_item("1", 1),
            new_fake_item("2", 2),
            new_fake_item("3", 3),
        ];
        repository.insert_items(items.clone()).await.unwrap();

        items = repository
            .get_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items;
        repository
            .mark_as(items[0].key(), State::STARRED)
            .await
            .unwrap();
        assert_eq!(
            repository
                .get_starred_items("user_id", PageOption::<String>::new(10, false))
                .await
                .unwrap()
                .items,
            items[0..1]
                .iter()
                .map(|item| {
                    let mut ret = item.clone();
                    ret.starred = true;
                    ret
                })
                .collect::<Vec<Item>>()
        );
        repository
            .mark_as(items[0].key(), State::UNSTARRED)
            .await
            .unwrap();
        assert!(repository
            .get_starred_items("user_id", PageOption::<String>::new(10, false))
            .await
            .unwrap()
            .items
            .is_empty());

        repository
            .mark_as(items[2].key(), State::READ)
            .await
            .unwrap();
        assert_eq!(
            repository
                .get_unread_items("user_id", PageOption::<String>::new(10, false))
                .await
                .unwrap()
                .items,
            &items[0..2]
        );
        assert_eq!(
            repository
                .get_read_items("user_id", PageOption::<String>::new(10, false))
                .await
                .unwrap()
                .items,
            &items[2..]
        );
        repository
            .mark_as(items[2].key(), State::UNREAD)
            .await
            .unwrap();
        assert_eq!(
            repository
                .get_unread_items("user_id", PageOption::<String>::new(10, false))
                .await
                .unwrap()
                .items,
            items
        );
    }
}
