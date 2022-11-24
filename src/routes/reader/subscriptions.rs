use crate::common::Services;
use crate::middlewares::auth::AuthUser;
use crate::services::subscriptions::Subscription;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

pub async fn list_subscriptions(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    // TODO: handle error properly
    let subscriptions = services
        .subscription_service
        .list_subscriptions(user_id)
        .await
        .unwrap();
    HttpResponse::Ok().json(Subscriptions { subscriptions })
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

pub async fn add_subscription(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    request: web::Form<AddRequest>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    // TODO: handle error properly
    let subscription = services
        .subscription_service
        .add_subscription_from_url(user_id, &request.quickadd)
        .await
        .unwrap();
    HttpResponse::Ok().json(AddSubscriptionResponse {
        query: request.quickadd.clone(),
        num_results: 1,
        stream_id: subscription.id,
    })
}

#[derive(Deserialize)]
pub struct SubscriptionEditRequest {
    // Action
    ac: String,
    // Subscription id
    s: String,
    // Title
    t: Option<String>,
    // Tag to add
    a: Vec<String>,
    // Tag to remove
    r: Vec<String>,
}

pub async fn edit_subscription(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    request: web::Form<SubscriptionEditRequest>,
) -> HttpResponse {
    let user_id = &auth_user.id;
    if let Some(feed_url) = request.s.strip_prefix("feed/") {
        match request.ac.deref() {
            "subscribe" => {
                services
                    .subscription_service
                    .add_subscription_from_url(user_id, feed_url)
                    .await
                    .unwrap();
            }
            "unsubscribe" => {
                services
                    .subscription_service
                    .remove_subscription(user_id, &request.s)
                    .await
                    .unwrap();
            }
            _ => {}
        };
    }
    if request.ac == "edit" {
        services
            .subscription_service
            .edit_subscription(
                user_id,
                &request.s,
                &request.t.as_deref(),
                &request.a.iter().map(|s| s.as_str()).collect(),
                &request.r.iter().map(|s| s.as_str()).collect(),
            )
            .await
            .unwrap();
    }
    HttpResponse::Ok().body("OK")
}
