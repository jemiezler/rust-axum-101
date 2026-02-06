use sqlx::Executor;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::str::FromStr;

pub async fn init_db_if_not_exists(database_url: &str) -> Result<(), sqlx::Error> {
    let options = PgConnectOptions::from_str(database_url)?;
    let db_name = options.get_database().unwrap_or("postgres");

    let admin_options = options.clone().database("postgres");
    let pool = PgPoolOptions::new().connect_with(admin_options).await?;

    let query = format!(r#"SELECT 1 FROM pg_database WHERE datname = '{}'"#, db_name);

    let exists = pool.fetch_optional(query.as_str()).await?.is_some();

    if !exists {
        let create_db = format!(r#"CREATE DATABASE "{}""#, db_name);
        pool.execute(create_db.as_str()).await?;
    }

    Ok(())
}
