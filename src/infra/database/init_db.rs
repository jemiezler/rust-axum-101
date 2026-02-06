use sqlx::Executor;
use sqlx::PgPool;
use std::fmt::Write;

pub async fn init_db_if_not_exists(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    db_name: &str,
) -> Result<(), sqlx::Error> {
    let mut url = String::new();
    write!(
        &mut url,
        "postgres://{}:{}@{}:{}/postgres",
        user, password, host, port
    )
    .unwrap();

    let pool: PgPool = PgPool::connect(&url).await?;

    let query = format!(r#"SELECT 1 FROM pg_database WHERE datname = '{}'"#, db_name);

    let exists = pool.fetch_optional(query.as_str()).await?.is_some();

    if !exists {
        let create_db = format!(r#"CREATE DATABASE "{}""#, db_name);
        pool.execute(create_db.as_str()).await?;
    }

    Ok(())
}
