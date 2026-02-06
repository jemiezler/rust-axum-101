use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pg_pool(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    db_name: &str,
    max_connections: u32,
    connect_timeout_secs: u64,
) -> Result<PgPool, sqlx::Error> {
    use std::fmt::Write;

    let mut url = String::new();
    write!(
        &mut url,
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    )
    .unwrap();

    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(connect_timeout_secs))
        .connect(&url)
        .await
}
