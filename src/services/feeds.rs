use feed_rs::model::Feed;
use feed_rs::parser;
use reqwest;
use anyhow::Result;

#[rocket::async_trait]
pub trait FeedService {
  async fn get_feed(&self, url: &str) -> Result<Feed>;
}

struct FeedServiceImpl {}

#[rocket::async_trait]
impl FeedService for FeedServiceImpl {
  async fn get_feed(&self, url: &str) -> Result<Feed> {
    let body = reqwest::get(url).await?.text().await?;
    let feed = parser::parse(body.as_bytes())?;
    Ok(feed)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rocket::async_test]
  async fn get_feed_should_work() {
    let feed_service = FeedServiceImpl {};
    let feed: Feed = feed_service.get_feed("https://www.daemonology.net/hn-daily/index.rss").await.unwrap();
    assert_eq!(&feed.title.as_ref().unwrap().content, "Hacker News Daily");
  }
}