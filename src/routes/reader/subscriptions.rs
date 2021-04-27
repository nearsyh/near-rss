use crate::middlewares::auth::AuthUser;
use crate::middlewares::di::Services;
use crate::services::subscriptions::Subscription;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

#[get("/api/0/subscription/list")]
pub async fn list_subscriptions(auth_user: AuthUser, services: &Services) -> Json<Subscriptions> {
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSubscriptionResponse {
    query: String,
    num_results: i64,
    stream_id: String,
}

#[get("/api/0/subscription/quickadd?<quickadd>")]
pub async fn add_subscription(
    auth_user: AuthUser,
    services: &Services,
    quickadd: &'_ str,
) -> Json<AddSubscriptionResponse> {
    let user = auth_user.user;
    // TODO: handle error properly
    let subscription = services
        .subscription_service
        .add_subscription_from_url(&user.id, quickadd)
        .await
        .unwrap();
    Json(AddSubscriptionResponse {
        query: quickadd.to_string(),
        num_results: 1,
        stream_id: subscription.id,
    })
}
