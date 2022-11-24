use near_rss::configuration::{get_configuration, Configuration};
use near_rss::database::users::User;
use near_rss::Application;
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
        assert_eq!(response.status().as_u16(), 200);
        self.token = Some(
            response
                .text()
                .await
                .unwrap()
                .split('\n')
                .collect::<Vec<&str>>()[2]
                .split('=')
                .collect::<Vec<&str>>()[1]
                .into(),
        );
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
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
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

    pub async fn quick_add_subscription(&self, link: &str) -> reqwest::Response {
        self.api_client
            .post(format!(
                "{}/reader/api/0/subscription/quickadd",
                self.address
            ))
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
            .form(&serde_json::json!({ "quickadd": link }))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn list_subscriptions(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/reader/api/0/subscription/list", self.address))
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn mark_as_read(&self, ids: &[&str]) -> reqwest::Response {
        self.api_client
            .post(format!("{}/api/markAsRead", self.address))
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
            .json(&serde_json::json!({ "ids": ids }))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_unread_items(
        &self,
        offset: Option<String>,
        limit: Option<usize>,
    ) -> reqwest::Response {
        let mut builder = self.api_client.get(format!("{}/api/unread", self.address));
        if let Some(offset) = offset.as_deref() {
            builder = builder.query(&[("offset", offset)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)]);
        }

        builder
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
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

    pub async fn get_user_info(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/reader/api/0/user-info", self.address))
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_user_token(&self) -> reqwest::Response {
        self.api_client
            .get(format!("{}/reader/api/0/token", self.address))
            .header(
                "Authorization",
                format!("GoogleLogin auth={}", self.token.as_deref().unwrap_or("")),
            )
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to get configuration.");
        c.application.port = 0;
        c
    };

    let app = Application::create(&configuration)
        .await
        .expect("Failed to create actix server");
    let port = app.port;

    let pool = app.pool.clone();
    let _ = tokio::spawn(app.run_until_stopped());

    configure_database(&pool).await;
    let test_user = TestUser::generate();
    test_user.store(&pool).await;

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
