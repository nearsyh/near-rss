use crate::common::{PageOption, Services};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::ItemContent;
use actix_web::{web, HttpResponse};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    items: Vec<ItemContent>,
    next_page_offset: Option<String>,
}

#[get("/unread?<offset>&<limit>")]
pub async fn old_get_unread_items(
    auth_user: AuthUser,
    services: &Services,
    offset: Option<String>,
    limit: Option<usize>,
) -> Json<Contents> {
    let user_id = &auth_user.user.id;
    let contents = services
        .stream_service
        .get_unread_item_contents(
            user_id,
            PageOption {
                offset: offset,
                limit: limit.unwrap_or(100),
                desc: true,
            },
        )
        .await
        .unwrap();
    Json(Contents {
        items: contents.items,
        next_page_offset: contents.next_page_offset,
    })
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
    let user_id = &auth_user.user.id;
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

#[options("/unread")]
pub async fn get_unread_items_options() -> &'static str {
    ""
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ids {
    ids: Vec<String>,
}

#[post("/markAsRead", data = "<ids>")]
pub async fn old_mark_as_read(
    auth_user: AuthUser,
    services: &Services,
    ids: Json<Ids>,
) -> &'static str {
    let str_ids = ids.ids.iter().map(|s| &**s).collect();
    services
        .stream_service
        .mark_as_read(&auth_user.user.id, &str_ids)
        .await
        .unwrap();
    "OK"
}

pub async fn mark_as_read(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    ids: web::Json<Ids>,
) -> HttpResponse {
    let str_ids = ids.ids.iter().map(|s| &**s).collect();
    services
        .stream_service
        .mark_as_read(&auth_user.user.id, &str_ids)
        .await
        .unwrap();
    HttpResponse::Ok().body("OK")
}

#[options("/markAsRead")]
pub async fn mark_as_read_options() -> &'static str {
    ""
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionData {
    link: String,
    title: Option<String>,
    folder: Option<String>,
}

#[post("/addSubscription", data = "<subscription>")]
pub async fn old_add_subscription(
    auth_user: AuthUser,
    services: &Services,
    subscription: Json<SubscriptionData>,
) -> &'static str {
    let added = services
        .subscription_service
        .add_subscription_from_url(&auth_user.user.id, &subscription.link)
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
            .edit_subscription(&auth_user.user.id, &added.id, &title, &vec![&tag], &vec![])
            .await
            .unwrap();
    }
    services
        .subscription_service
        .load_all_subscription_items()
        .await
        .unwrap();
    "OK"
}

pub async fn add_subscription(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    subscription: web::Json<SubscriptionData>,
) -> &'static str {
    let added = services
        .subscription_service
        .add_subscription_from_url(&auth_user.user.id, &subscription.link)
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
            .edit_subscription(&auth_user.user.id, &added.id, &title, &vec![&tag], &vec![])
            .await
            .unwrap();
    }
    services
        .subscription_service
        .load_all_subscription_items()
        .await
        .unwrap();
    "OK"
}

#[options("/addSubscription")]
pub async fn add_subscription_options() -> &'static str {
    ""
}
