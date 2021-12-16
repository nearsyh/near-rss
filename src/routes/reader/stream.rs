use crate::common::Services;
use crate::common::{current_time_s, Page, PageOption};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::{ItemContent, ItemId};
use rocket::form::Form;
// use rocket::response::content::Json;
use serde::Serialize;
use rocket::serde::{Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
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
    let filter_type = FilterType::from_params(s, xt);
    let item_ids_page = match filter_type {
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

#[derive(Serialize, Debug)]
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
    let mut item_contents = match ids.i {
        Some(ref i) => {
            let ids_in_hex = super::convert_to_long_form_ids(i);
            services
                .stream_service
                .get_item_contents(&user.id, &ids_in_hex.iter().map(|s| &**s).collect())
                .await
                .unwrap()
        }
        None => vec![],
    };
    item_contents.sort_by(|a, b| b.published.cmp(&a.published));
    Json(Contents {
        direction: "ltr".to_string(),
        id: "user/-/state/com.google/reading-list".to_string(),
        title: "Reading List".to_string(),
        description: "Reading List".to_string(),
        updated: current_time_s() as u64,
        items: item_contents,
    })
}
