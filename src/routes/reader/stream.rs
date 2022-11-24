use crate::common::Services;
use crate::common::{current_time_s, Page, PageOption};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::{ItemContent, ItemId};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct Query {
    s: String,
    xt: Option<String>,
    n: Option<usize>,
    r: Option<String>,
    c: Option<String>,
}

pub async fn get_item_ids(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    query: web::Query<Query>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    let page_option = PageOption::<String> {
        offset: query.c.as_deref().map(|s| String::from(s)),
        limit: query.n.unwrap_or(100usize),
        desc: !query.r.as_deref().unwrap_or("").eq("o"),
    };
    let filter_type = FilterType::from_params(&query.s, query.xt.as_deref());
    let item_ids_page = match filter_type {
        FilterType::STARRED => services
            .stream_service
            .get_starred_item_ids(user_id, page_option)
            .await
            .unwrap(),
        FilterType::UNREAD => services
            .stream_service
            .get_unread_item_ids(user_id, page_option)
            .await
            .unwrap(),
        FilterType::READ => services
            .stream_service
            .get_read_item_ids(user_id, page_option)
            .await
            .unwrap(),
        FilterType::ALL => services
            .stream_service
            .get_all_item_ids(user_id, page_option)
            .await
            .unwrap(),
    };
    HttpResponse::Ok().json(ItemIds::from(item_ids_page))
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

#[derive(serde::Deserialize)]
pub struct Ids {
    pub i: Option<Vec<String>>,
}

pub async fn get_contents(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    ids: web::Form<Ids>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    let mut item_contents = match ids.i {
        Some(ref i) => {
            let ids_in_hex =
                super::convert_to_long_form_ids(&i.iter().map(|s| s.as_str()).collect());
            services
                .stream_service
                .get_item_contents(user_id, &ids_in_hex.iter().map(|s| &**s).collect())
                .await
                .unwrap()
        }
        None => vec![],
    };
    item_contents.sort_by(|a, b| b.published.cmp(&a.published));
    HttpResponse::Ok().json(Contents {
        direction: "ltr".to_string(),
        id: "user/-/state/com.google/reading-list".to_string(),
        title: "Reading List".to_string(),
        description: "Reading List".to_string(),
        updated: current_time_s() as u64,
        items: item_contents,
    })
}
