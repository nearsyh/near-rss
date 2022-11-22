use crate::common::Services;
use crate::middlewares::auth::AuthUser;
use crate::services::subscriptions::Subscription;
use actix_web::{web, HttpResponse};
use rocket::form::Form;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(FromForm)]
pub struct EditTagRequest<'r> {
    pub i: Option<Vec<&'r str>>,
    pub a: Option<&'r str>,
    pub r: Option<&'r str>,
}

#[derive(Serialize, Deserialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

#[get("/api/0/subscription/list")]
pub async fn old_list_subscriptions(
    auth_user: AuthUser,
    services: &Services,
) -> Json<Subscriptions> {
    let user = auth_user.user;
    // TODO: handle error properly
    let subscriptions = services
        .subscription_service
        .list_subscriptions(&user.id)
        .await
        .unwrap();
    Json(Subscriptions {
        subscriptions: subscriptions,
    })
}

pub async fn list_subscriptions(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
) -> HttpResponse {
    let user = &auth_user.user;
    // TODO: handle error properly
    let subscriptions = services
        .subscription_service
        .list_subscriptions(&user.id)
        .await
        .unwrap();
    HttpResponse::Ok().json(Subscriptions { subscriptions })
}

#[derive(FromForm)]
pub struct OldAddRequest {
    quickadd: String,
}

#[derive(serde::Deserialize)]
pub struct AddRequest {
    quickadd: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSubscriptionResponse {
    query: String,
    num_results: i64,
    stream_id: String,
}

#[post("/api/0/subscription/quickadd", data = "<request>")]
pub async fn old_add_subscription(
    auth_user: AuthUser,
    services: &Services,
    request: Form<OldAddRequest>,
) -> Json<AddSubscriptionResponse> {
    let user = auth_user.user;
    // TODO: handle error properly
    let subscription = services
        .subscription_service
        .add_subscription_from_url(&user.id, &request.quickadd)
        .await
        .unwrap();
    Json(AddSubscriptionResponse {
        query: request.quickadd.clone(),
        num_results: 1,
        stream_id: subscription.id,
    })
}

pub async fn add_subscription(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    request: web::Form<AddRequest>,
) -> HttpResponse {
    let user = &auth_user.user;
    // TODO: handle error properly
    let subscription = services
        .subscription_service
        .add_subscription_from_url(&user.id, &request.quickadd)
        .await
        .unwrap();
    HttpResponse::Ok().json(AddSubscriptionResponse {
        query: request.quickadd.clone(),
        num_results: 1,
        stream_id: subscription.id,
    })
}

#[derive(FromForm)]
pub struct SubscriptionEditRequest<'r> {
    // Action
    ac: &'r str,
    // Subscription id
    s: &'r str,
    // Title
    t: Option<&'r str>,
    // Tag to add
    a: Vec<&'r str>,
    // Tag to remove
    r: Vec<&'r str>,
}

#[post("/api/0/subscription/edit", data = "<request>")]
pub async fn edit_subscription(
    auth_user: AuthUser,
    services: &Services,
    request: Form<SubscriptionEditRequest<'_>>,
) -> &'static str {
    let user = auth_user.user;
    if let Some(feed_url) = request.s.strip_prefix("feed/") {
        match request.ac {
            "subscribe" => {
                services
                    .subscription_service
                    .add_subscription_from_url(&user.id, feed_url)
                    .await
                    .unwrap();
            }
            "unsubscribe" => {
                services
                    .subscription_service
                    .remove_subscription(&user.id, &request.s)
                    .await
                    .unwrap();
            }
            _ => {}
        };
    }
    if request.ac == "edit" {
        services
            .subscription_service
            .edit_subscription(&user.id, &request.s, &request.t, &request.a, &request.r)
            .await
            .unwrap();
    }
    "OK"
}
