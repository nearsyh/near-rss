pub struct PageOption<T> {
  pub offset: T,
  pub limit: u32,
  pub desc: bool 
}

pub struct Page<T, OT> {
  pub items: Vec<T>,
  pub next_page_offset: Option<OT>,
}