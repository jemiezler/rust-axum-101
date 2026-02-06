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
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout_secs: u64,
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
                url: get("DATABASE_URL")?,
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
