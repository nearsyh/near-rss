pub mod token;

pub struct PageOption<T> {
  pub offset: T,
  pub limit: u32,
  pub desc: bool,
}

pub struct Page<T, OT> {
  pub items: Vec<T>,
  pub next_page_offset: Option<OT>,
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
