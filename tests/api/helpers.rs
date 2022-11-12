use near_rss::configuration::{get_configuration, Configuration};
use near_rss::create;
use near_rss::database::users::User;
use reqwest::redirect::Policy;
use reqwest::Client;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

pub struct TestUser {
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            email: format!("{}@gmail.com", Uuid::new_v4().to_string()),
            password: "password".into(),
        }
    }

    pub async fn store(&self, pool: &SqlitePool) {
        let user = User::new(&Uuid::new_v4().to_string(), &self.email, &self.password);
        sqlx::query!(
            r#"
            INSERT INTO Users
                (id, email, password_hash, token)
            VALUES(?, ?, ?, ?)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.token
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}

pub struct TestApp {
    pub port: u16,
    pub address: String,
    pub api_client: Client,
    pub test_user: TestUser,
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
    let configuration = {
        let mut c = get_configuration().expect("Failed to get configuration.");
        c.application.port = portpicker::pick_unused_port().expect("No free ports");
        c
    };
    println!("Port: {}", configuration.application.port);

    let app = create(&configuration).await;
    let _ = tokio::spawn(app.rocket.launch());
    // TODO: why do this this before creating rocket works?
    configure_database(&app.pool).await;

    let test_user = TestUser::generate();
    test_user.store(&app.pool).await;

    let client = Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        port: configuration.application.port,
        address: format!("http://127.0.0.1:{}", configuration.application.port),
        api_client: client,
        test_user,
    }
}

async fn configure_database(pool: &SqlitePool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to migrate the database.");
}
