pub mod error;
pub mod token;

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
