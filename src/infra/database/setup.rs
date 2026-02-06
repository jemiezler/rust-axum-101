use sqlx::PgPool;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use std::time::Duration;

use super::init_db::init_db_if_not_exists;

pub async fn init(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    db_name: &str,
    max_connections: u32,
    connect_timeout_secs: u64,
) -> Result<PgPool, sqlx::Error> {
    /* Init DB if not exists */
    init_db_if_not_exists(host, port, user, password, db_name)
        .await
        .expect("failed to init db");

    use std::fmt::Write;
    let mut url = String::new();
    write!(
        &mut url,
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    )
    .unwrap();

    /* Connect to the specific DB */
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(connect_timeout_secs))
        .connect(&url)
        .await
        .expect("failed to connect to db for migration");

    /* Run migrations
    Check if migrations folder exists before running
    */
    if Path::new("./migrations").exists() {
        let migrator = Migrator::new(Path::new("./migrations")).await?;
        migrator.run(&pool).await?;
    } else {
        tracing::warn!("migrations directory not found, skipping migrations");
    }

    Ok(pool)
}
