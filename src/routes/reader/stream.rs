use crate::common::{Page, PageOption};
use crate::middlewares::auth::AuthUser;
use crate::middlewares::di::Services;
use crate::services::stream::ItemId;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIds {
  pub item_refs: Vec<ItemId>,
  pub continuation: Option<String>,
}

impl From<Page<ItemId, String>> for ItemIds {
  fn from(page: Page<ItemId, String>) -> ItemIds {
    ItemIds {
      item_refs: page.items,
      continuation: page.next_page_offset,
    }
  }
}

enum FilterType {
  UNREAD,
  STARRED,
  ALL,
}

impl FilterType {
  fn from_params(s: &str, xt: Option<&str>) -> FilterType {
    if s.ends_with("/state/com.google/starred") {
      FilterType::STARRED
    } else {
      if xt.is_some() && xt.unwrap().ends_with("/state/com.google/read") {
        FilterType::UNREAD
      } else {
        FilterType::ALL
      }
    }
  }
}

#[get("/api/0/stream/items/ids?<s>&<xt>&<n>&<r>&<c>")]
pub async fn get_item_ids(
  auth_user: AuthUser,
  services: &Services,
  s: &str,
  xt: Option<&str>,
  n: Option<usize>,
  r: Option<&str>,
  c: Option<&str>,
) -> Json<ItemIds> {
  let user = auth_user.user;
  let page_option = PageOption::<String> {
    offset: c.map(|s| String::from(s)),
    limit: n.unwrap_or(100usize),
    desc: !r.unwrap_or("").eq("o"),
  };
  let item_ids_page = match FilterType::from_params(s, xt) {
    FilterType::STARRED => services
      .stream_service
      .get_unread_item_ids(&user.id, page_option)
      .await
      .unwrap(),
    FilterType::UNREAD => Page::<ItemId, String>::empty(),
    FilterType::ALL => Page::<ItemId, String>::empty(),
  };
  Json(ItemIds::from(item_ids_page))
}

#[derive(Serialize)]
pub struct Contents {
  direction: String,
  id: String,
  title: String,
  description: String,
  updated: u64,
  items: Vec<Content>,
}

#[derive(Serialize)]
pub struct Content {}

#[get("/api/0/stream/contents")]
pub async fn get_contents() -> Json<Contents> {
  Json(Contents {
    direction: "ltr".to_string(),
    id: "id".to_string(),
    title: "Reading List".to_string(),
    description: "Reading List".to_string(),
    updated: 1619362681,
    items: vec![],
  })
}
