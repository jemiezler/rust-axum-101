use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pg_pool(
    database_url: &str,
    max_connections: u32,
    connect_timeout_secs: u64,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(connect_timeout_secs))
        .connect(database_url)
        .await
}
