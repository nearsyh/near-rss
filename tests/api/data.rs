use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    href: String,
    #[serde(rename = "type")]
    type_f: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    direction: String,
    content: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    stream_id: String,
    title: String,
    html_url: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemContent {
    pub crawl_time_msec: String,
    pub timestamp_usec: String,
    pub id: String,
    pub categories: Vec<String>,
    // Seconds
    pub published: i64,
    // Seconds
    pub updated: i64,
    pub canonical: Vec<Url>,
    pub alternate: Vec<Url>,
    pub summary: Summary,
    pub title: String,
    pub author: String,
    pub origin: Origin,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub items: Vec<ItemContent>,
    pub next_page_offset: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
    pub user_profile_id: String,
    pub user_email: String,
    pub is_blogged_user: bool,
    pub signup_time_sec: u64,
    pub is_multi_login_enabled: bool,
}

#[derive(Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub label: String,
}

#[derive(Deserialize, Clone)]
pub struct Subscription {
    pub id: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub url: String,
    pub feed_url: String,
}

#[derive(Deserialize)]
pub struct Subscriptions {
    pub subscriptions: Vec<Subscription>,
}
