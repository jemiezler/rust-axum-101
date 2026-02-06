use dotenvy::dotenv;
use serde::Deserialize;

use super::error::ConfigError;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub database: Database,
}

#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
    pub env: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub max_connections: u32,
    pub connect_timeout_secs: u64,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv().ok();

        let config = AppConfig {
            app: App {
                host: get("APP_HOST")?,
                port: get("APP_PORT")?.parse()?,
                env: get("APP_ENV")?,
            },
            database: Database {
                host: get("DB_HOST")?,
                port: get("DB_PORT")?.parse()?,
                user: get("DB_USER")?,
                password: get("DB_PASSWORD")?,
                db_name: get("DB_NAME")?,
                max_connections: get("DB_MAX_CONNECTIONS").unwrap_or("10".into()).parse()?,
                connect_timeout_secs: get("DB_CONNECT_TIMEOUT").unwrap_or("5".into()).parse()?,
            },
        };

        Ok(config)
    }
}

fn get(key: &str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::MissingVar(key.to_string()))
}
