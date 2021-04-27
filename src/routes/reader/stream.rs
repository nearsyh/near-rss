use crate::middlewares::auth::AuthUser;
use crate::middlewares::di::Services;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Item {
  pub id: i64,
  pub direct_stream_ids: Vec<String>,
  pub timestamp_usec: i64,
}

enum FilterType {
  UNREAD,
  STARRED,
}

impl FilterType {
  fn from_params(s: &str, xt: &str) -> FilterType {
    if s.ends_with("/state/com.google/starred") {
      FilterType::STARRED
    } else {
      FilterType::UNREAD
    }
  }
}

#[get("/api/0/stream/items/ids?<s>&<xt>&<n>&<r>")]
pub async fn get_item_ids(
  auth_user: AuthUser,
  services: &Services,
  s: &str,
  xt: &str,
  n: usize,
  r: &str,
) -> Json<Vec<Item>> {
  match FilterType::from_params(s, xt) {
    FilterType::STARRED => Json(vec![]),
    FilterType::UNREAD => Json(vec![]),
  }
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
