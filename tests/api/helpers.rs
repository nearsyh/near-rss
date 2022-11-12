use std::str::FromStr;
use reqwest::Client;
use reqwest::redirect::Policy;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use near_rss::configuration::{Configuration, get_configuration};
use near_rss::create;

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub api_client: Client,
}

impl TestApp {
    pub async fn login(&self, email: &str, password: &str) -> reqwest::Response {
        self.api_client
            .post(format!("{}/accounts/ClientLogin", self.address))
            .form(&serde_json::json!({
                "Email": email,
                "Passwd": password
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to get configuration.");

    let rocket = create(&configuration).await;
    let _ = tokio::spawn(rocket.launch());
    // TODO: why do this this before creating rocket works?
    configure_database(&configuration).await;

    let client = Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        port: configuration.application.port,
        address: format!("http://127.0.0.1:{}", configuration.application.port),
        api_client: client,
    }
}

async fn configure_database(configuration: &Configuration) {
    let option = SqliteConnectOptions::from_str("sqlite::memory:")
        .expect("Failed to create connect options");
    let sqlite_pool = SqlitePoolOptions::new().connect_lazy_with(option);
    sqlx::migrate!("./migrations")
        .run(&sqlite_pool)
        .await
        .expect("Failed to migrate the database.");
}