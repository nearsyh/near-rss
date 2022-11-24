use again::RetryPolicy;
use anyhow::Result;
use async_trait::async_trait;
use feed_rs::model::Feed;
use feed_rs::parser;
use futures::future::FutureExt;
use futures::select;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use reqwest;
use std::collections::HashMap;
use std::time::Duration;

#[async_trait]
pub trait FeedService {
    async fn get_feed(&self, url: &str) -> Result<Feed>;

    async fn get_feeds(&self, urls: Vec<&str>) -> HashMap<String, Result<Feed>>;
}

struct FeedServiceImpl {}

#[async_trait]
impl FeedService for FeedServiceImpl {
    async fn get_feed(&self, url: &str) -> Result<Feed> {
        let policy = RetryPolicy::fixed(Duration::from_millis(100))
            .with_max_retries(3)
            .with_jitter(true);
        let response = policy.retry(|| reqwest::get(url));
        let body = response.await?.text().await?;
        let feed = parser::parse(body.as_bytes())?;
        Ok(feed)
    }

    async fn get_feeds(&self, urls: Vec<&str>) -> HashMap<String, Result<Feed>> {
        let mut all_feeds = FuturesUnordered::new();
        urls.into_iter().for_each(|url| {
            all_feeds.push(
                self.get_feed(url)
                    .map(move |feed| (String::from(url), feed)),
            );
        });
        let mut feeds_map = HashMap::new();
        loop {
            select! {
              completed_feed = all_feeds.select_next_some() => {
                feeds_map.insert(completed_feed.0, completed_feed.1);
              },
              complete => break,
            }
        }
        feeds_map
    }
}

pub fn new_feed_service() -> Box<dyn FeedService + Send + Sync> {
    Box::new(FeedServiceImpl {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rocket::async_test]
    async fn get_feed_should_work() {
        let feed_service = FeedServiceImpl {};
        let feed: Feed = feed_service
            .get_feed("https://www.daemonology.net/hn-daily/index.rss")
            .await
            .unwrap();
        assert_eq!(&feed.title.as_ref().unwrap().content, "Hacker News Daily");
    }

    #[rocket::async_test]
    async fn get_feeds_should_work() {
        let feed_service = FeedServiceImpl {};
        let feed_1 = "https://www.daemonology.net/hn-daily/index.rss";
        let feed_2 = "https://blogs.nearsyh.me/atom.xml";
        let feeds_map: HashMap<String, Result<Feed>> =
            feed_service.get_feeds(vec![feed_1, feed_2]).await;
        assert_eq!(
            feeds_map[feed_1]
                .as_ref()
                .unwrap()
                .title
                .as_ref()
                .unwrap()
                .content,
            "Hacker News Daily"
        );
        assert_eq!(
            feeds_map[feed_2]
                .as_ref()
                .unwrap()
                .title
                .as_ref()
                .unwrap()
                .content,
            "瞎扯"
        );
    }
}
