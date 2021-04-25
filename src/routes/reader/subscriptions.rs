use crate::middlewares::auth::AuthToken;
use crate::services::subscriptions::{new_subscription_service, Subscription};
use crate::services::users::new_user_service;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Subscriptions {
    subscriptions: Vec<Subscription>,
}

#[get("/api/0/subscription/list")]
pub async fn list_subscriptions(token: AuthToken<'_>) -> Json<Subscriptions> {
    let user = new_user_service().get_user(token.0).await;
    let subscriptions = new_subscription_service()
        .list_subscriptions(&user.id, 0, 100)
        .await;
    Json(Subscriptions {
        subscriptions: subscriptions,
    })
}
