use crate::common::Services;
use crate::configuration::Configuration;
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

pub async fn refresh_until_stopped(configuration: Configuration) -> Result<(), anyhow::Error> {
    let sqlite_pool =
        SqlitePoolOptions::new().connect_lazy_with(configuration.database.connect_options());
    let services = Services::new(sqlite_pool.clone()).await;
    loop {
        if let Err(err) = services.stream_service.clean_up().await {
            println!("Clean up old items failed {:?}", err);
        }
        if let Err(err) = services
            .subscription_service
            .load_all_subscription_items()
            .await
        {
            println!("Load subscription items failed: {:?}", err);
        }
        tokio::time::sleep(Duration::from_secs(600)).await;
    }
}
