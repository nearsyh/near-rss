use crate::middlewares::auth::AuthUser;
use crate::middlewares::di::Services;
use crate::common::PageOption;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub id: i64,
  pub direct_stream_ids: Vec<String>,
  pub timestamp_usec: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
  pub item_refs: Vec<Item>,
  pub continuation: String,
}

enum FilterType {
  UNREAD,
  STARRED,
}

impl FilterType {
  fn from_params(s: Option<&str>, _xt: Option<&str>) -> FilterType {
    if s.is_some() && s.unwrap().ends_with("/state/com.google/starred") {
      FilterType::STARRED
    } else {
      FilterType::UNREAD
    }
  }
}

#[get("/api/0/stream/items/ids?<s>&<xt>&<n>&<r>&<c>")]
pub async fn get_item_ids(
  _auth_user: AuthUser,
  _services: &Services,
  s: Option<&str>,
  xt: Option<&str>,
  n: Option<usize>,
  r: Option<&str>,
  c: Option<&str>,
) -> Json<Vec<Item>> {
  let _page_option = PageOption::<String> {
    offset: c.map(|s| String::from(s)),
    limit: n.unwrap_or(100usize),
    desc: !r.unwrap_or("").eq("o")
  };
  let items = match FilterType::from_params(s, xt) {
    FilterType::STARRED => Json(vec![]),
    FilterType::UNREAD => Json(vec![]),
  };
  items
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
