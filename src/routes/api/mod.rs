use crate::common::{PageOption, Services};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::ItemContent;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    items: Vec<ItemContent>,
    next_page_offset: Option<String>,
}

#[derive(Deserialize)]
pub struct Page {
    offset: Option<String>,
    limit: Option<usize>,
}

pub async fn get_unread_items(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    page: web::Query<Page>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    let contents = services
        .stream_service
        .get_unread_item_contents(
            user_id,
            PageOption {
                offset: page.offset.clone(),
                limit: page.limit.unwrap_or(100),
                desc: true,
            },
        )
        .await
        .unwrap();
    HttpResponse::Ok().json(Contents {
        items: contents.items,
        next_page_offset: contents.next_page_offset,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ids {
    ids: Vec<String>,
}

pub async fn mark_as_read(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    ids: web::Json<Ids>,
) -> HttpResponse {
    let str_ids = ids.ids.iter().map(|s| &**s).collect();
    services
        .stream_service
        .mark_as_read(&auth_user.id, &str_ids)
        .await
        .unwrap();
    HttpResponse::Ok().body("OK")
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionData {
    link: String,
    title: Option<String>,
    folder: Option<String>,
}

pub async fn add_subscription(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    subscription: web::Json<SubscriptionData>,
) -> HttpResponse {
    let added = services
        .subscription_service
        .add_subscription_from_url(&auth_user.id, &subscription.link)
        .await
        .unwrap();
    if let Some(ref f) = subscription.folder {
        let tag = format!("user/-/label/{}", f);
        let title: Option<&str> = if subscription.title.is_some() {
            Some(&**subscription.title.as_ref().unwrap())
        } else {
            None
        };
        services
            .subscription_service
            .edit_subscription(&auth_user.id, &added.id, &title, &vec![&tag], &vec![])
            .await
            .unwrap();
    }
    services
        .subscription_service
        .load_all_subscription_items()
        .await
        .unwrap();
    HttpResponse::Ok().body("OK")
}
