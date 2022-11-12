use near_rss::create;

pub struct TestApp {
    pub port: u16,
    pub address: String
}

pub async fn spawn_app() -> TestApp {
    let rocket = create().await;
    let _ = tokio::spawn(rocket.launch());
    TestApp {
        port: 8000,
        address: "http://127.0.0.1:8000".into()
    }
}