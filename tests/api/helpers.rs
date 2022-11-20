use near_rss::configuration::{get_configuration, Configuration};
use near_rss::database::users::User;
use near_rss::Application;
use reqwest::header::AUTHORIZATION;
use reqwest::redirect::Policy;
use reqwest::Client;
use sqlx::SqlitePool;
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
    pub token: Option<String>,
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

    pub async fn test_user_login(&mut self) {
        let response = self
            .login(&self.test_user.email, &self.test_user.password)
            .await;
        self.token = Some(response.text().await.unwrap());
    }

    pub fn test_user_logout(&mut self) {
        self.token.take();
    }

    pub async fn add_subscription(
        &self,
        link: &str,
        title: Option<&str>,
        folder: Option<&str>,
    ) -> reqwest::Response {
        self.api_client
            .post(format!("{}/api/addSubscription", self.address))
            .header(
                AUTHORIZATION,
                self.token
                    .as_ref()
                    .map(|t| format!("GoogleLogin auth={}", t))
                    .unwrap_or("".into()),
            )
            .json(&serde_json::json!({
                "link": link,
                "title": title,
                "folder": folder
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_index(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/index.html", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    spawn_app_by_type(true).await
}

pub async fn spawn_app_by_type(use_actix: bool) -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to get configuration.");
        c.application.port = 0;
        c
    };

    let app = if use_actix {
        Application::create_actix_server(&configuration)
            .await
            .expect("Failed to create actix server")
    } else {
        Application::create_rocket_server(&configuration)
            .await
            .expect("Failed to create rocket server")
    };
    let port = app.port;

    configure_database(&app.pool).await;
    let test_user = TestUser::generate();
    test_user.store(&app.pool).await;

    let _ = tokio::spawn(app.run_until_stopped());

    let client = Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        port,
        address: format!("http://127.0.0.1:{}", port),
        api_client: client,
        test_user,
        token: None,
    }
}

async fn configure_database(pool: &SqlitePool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to migrate the database.");
}
