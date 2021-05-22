pub mod debug;
pub mod error;
pub mod token;

use crate::services::stream::StreamService;
use crate::services::subscriptions::SubscriptionService;
use crate::services::users::UserService;
use reqwest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PageOption<T> {
  pub offset: Option<T>,
  pub limit: usize,
  pub desc: bool,
}

impl<T> PageOption<T> {
  pub fn new(limit: usize, desc: bool) -> Self {
    Self {
      offset: None,
      limit: limit,
      desc: desc,
    }
  }
}

pub struct Page<T, OT> {
  pub items: Vec<T>,
  pub next_page_offset: Option<OT>,
}

impl<T, OT> Page<T, OT> {
  pub fn empty() -> Self {
    Self {
      items: vec![],
      next_page_offset: None,
    }
  }

  pub fn convert<R, F>(self, f: F) -> Page<R, OT>
  where
    F: FnMut(T) -> R,
  {
    Page::<R, OT> {
      items: self.items.into_iter().map(f).collect::<Vec<R>>(),
      next_page_offset: self.next_page_offset,
    }
  }
}

use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;

pub fn new_id(length: usize) -> String {
  iter::repeat(())
    .map(|()| rand::thread_rng().sample(Alphanumeric))
    .map(char::from)
    .take(length)
    .collect()
}

pub fn current_time_ms() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as i64
}

pub fn current_time_s() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64
}

pub fn refresh() {
  let endpoint = format!("{}/refresh", std::env::var("ENDPOINT").unwrap());
  println!("Refreshing {}", endpoint);
  if let Err(err) = reqwest::blocking::get(&endpoint) {
    println!("Refreshing fails {:?}", err);
  }
}

pub struct Services {
  pub user_service: Box<dyn UserService + Send + Sync>,
  pub subscription_service: Box<dyn SubscriptionService + Send + Sync>,
  pub stream_service: Box<dyn StreamService + Send + Sync>,
}
