use log::error;

use crate::common::Services;

pub mod accounts;
pub mod api;
pub mod reader;

#[catch(403)]
pub fn unauthorized() -> &'static str {
    "Unauthorized"
}

#[get("/refresh")]
pub async fn refresh(services: &Services) -> &'static str {
    if let Err(err) = services.stream_service.clean_up().await {
        error!("Clean up old items failed {:?}", err);
    }
    match services
        .subscription_service
        .load_all_subscription_items()
        .await
    {
        Ok(_) => "OK",
        Err(err) => {
            error!("Load subscription items failed: {:?}", err);
            "ERR"
        }
    }
}
