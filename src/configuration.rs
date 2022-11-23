use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationConfiguration {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
    path: String,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    settings.try_into()
}

impl DatabaseConfiguration {
    pub fn connect_options(&self) -> SqliteConnectOptions {
        let db = format!("sqlite:{}", self.path);
        SqliteConnectOptions::from_str(&db)
            .expect(&format!(
                "Failed to create Sqlite connect options with {db}"
            ))
            .create_if_missing(true)
    }
}

enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
