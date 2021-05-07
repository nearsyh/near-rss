use crate::common::Services;
use crate::common::{current_time_s, Page, PageOption};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::{ItemContent, ItemId};
use rocket::form::Form;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIds {
    pub item_refs: Vec<ItemId>,
    pub continuation: Option<String>,
}

impl From<Page<ItemId, String>> for ItemIds {
    fn from(page: Page<ItemId, String>) -> ItemIds {
        ItemIds {
            item_refs: page.items,
            continuation: page.next_page_offset,
        }
    }
}

enum FilterType {
    UNREAD,
    STARRED,
    READ,
    ALL,
}

impl FilterType {
    fn from_params(s: &str, xt: Option<&str>) -> FilterType {
        if s.ends_with("/state/com.google/starred") {
            FilterType::STARRED
        } else if s.ends_with("/state/com.google/read") {
            FilterType::READ
        } else {
            if xt.is_some() && xt.unwrap().ends_with("/state/com.google/read") {
                FilterType::UNREAD
            } else {
                FilterType::ALL
            }
        }
    }
}

#[get("/api/0/stream/items/ids?<s>&<xt>&<n>&<r>&<c>")]
pub async fn get_item_ids(
    auth_user: AuthUser,
    services: &Services,
    s: &str,
    xt: Option<&str>,
    n: Option<usize>,
    r: Option<&str>,
    c: Option<&str>,
) -> Json<ItemIds> {
    let user = auth_user.user;
    let page_option = PageOption::<String> {
        offset: c.map(|s| String::from(s)),
        limit: n.unwrap_or(100usize),
        desc: !r.unwrap_or("").eq("o"),
    };
    services
        .subscription_service
        .load_subscription_items(&user.id)
        .await
        .unwrap();
    let item_ids_page = match FilterType::from_params(s, xt) {
        FilterType::STARRED => services
            .stream_service
            .get_starred_item_ids(&user.id, page_option)
            .await
            .unwrap(),
        FilterType::UNREAD => services
            .stream_service
            .get_unread_item_ids(&user.id, page_option)
            .await
            .unwrap(),
        FilterType::READ => services
            .stream_service
            .get_read_item_ids(&user.id, page_option)
            .await
            .unwrap(),
        FilterType::ALL => services
            .stream_service
            .get_all_item_ids(&user.id, page_option)
            .await
            .unwrap(),
    };
    Json(ItemIds::from(item_ids_page))
}

#[derive(Serialize)]
pub struct Contents {
    direction: String,
    id: String,
    title: String,
    description: String,
    updated: u64,
    items: Vec<ItemContent>,
}

#[derive(FromForm)]
pub struct Ids<'r> {
   pub i: Option<Vec<&'r str>>,
}

#[post("/api/0/stream/items/contents", data = "<ids>")]
pub async fn get_contents(
    auth_user: AuthUser,
    services: &Services,
    ids: Form<Ids<'_>>,
) -> Json<Contents> {
    let user = auth_user.user;
    let item_contents = match ids.i {
        Some(ref i) => services
            .stream_service
            .get_item_contents(&user.id, i)
            .await
            .unwrap(),
        None => vec![],
    };
    Json(Contents {
        direction: "ltr".to_string(),
        id: "id".to_string(),
        title: "Reading List".to_string(),
        description: "Reading List".to_string(),
        updated: current_time_s() as u64,
        items: item_contents,
    })
}
