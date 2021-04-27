use crate::middlewares::auth::AuthUser;
use crate::services::subscriptions::{new_subscription_service, Subscription};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

#[get("/api/0/subscription/list")]
pub async fn list_subscriptions(auth_user: AuthUser) -> Json<Subscriptions> {
    let user = auth_user.user;
    // TODO: handle error properly
    let subscriptions = new_subscription_service()
        .list_subscriptions(&user.id)
        .await
        .unwrap();
    Json(Subscriptions {
        subscriptions: subscriptions,
    })
}
