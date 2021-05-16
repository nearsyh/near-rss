use crate::common::Services;

pub mod accounts;
pub mod reader;

#[catch(403)]
pub fn unauthorized() -> &'static str {
    "Unauthorized"
}

#[get("/refresh")]
pub async fn refresh(services: &Services) -> &'static str {
    services
        .subscription_service
        .load_all_subscription_items()
        .await
        .unwrap();
    "OK"
}
