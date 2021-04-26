use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct Item {
  pub id: i64,
  pub direct_stream_ids: Vec<String>,
  pub timestamp_usec: i64,
}

#[get("/api/0/stream/items/ids")]
pub async fn get_item_ids() -> Json<Vec<Item>> {
  Json(vec![])
}

#[derive(Serialize)]
pub struct Contents {
  direction: String,
  id: String,
  title: String,
  description: String,
  updated: u64,
  items: Vec<Content>
}

#[derive(Serialize)]
pub struct Content {

}

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