use super::entities::{CreateUserRequest, UpdateUserRequest, User};
use crate::shared::types::result::DomainResult;
use sqlx::PgPool;
use tracing::error;

pub async fn get_all_users(pool: &PgPool) -> DomainResult<Vec<User>, String> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password, 
               created_at::TEXT as "created_at!", 
               updated_at::TEXT as "updated_at!" 
        FROM users
        "#
    )
    .fetch_all(pool)
    .await;

    match users {
        Ok(users) => DomainResult::Ok(users),
        Err(e) => {
            error!("Error getting users: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

pub async fn find_one_user(pool: &PgPool, id: i32) -> DomainResult<User, String> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, password, 
               created_at::TEXT as "created_at!", 
               updated_at::TEXT as "updated_at!" 
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await;

    match user {
        Ok(Some(user)) => DomainResult::Ok(user),
        Ok(None) => DomainResult::NotFound,
        Err(e) => {
            error!("Error getting user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> DomainResult<User, String> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, password, 
                  created_at::TEXT as "created_at!", 
                  updated_at::TEXT as "updated_at!"
        "#,
        req.name,
        req.email,
        req.password
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => DomainResult::Ok(user),
        Err(e) => {
            error!("Error creating user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

pub async fn update_user(
    pool: &PgPool,
    id: i32,
    req: UpdateUserRequest,
) -> DomainResult<User, String> {
    // Check if user exists first
    let exists = find_one_user(pool, id).await;
    if let DomainResult::NotFound = exists {
        return DomainResult::NotFound;
    }

    // This is a simple update, in a real app you might handle partial updates more dynamically
    // or fetch the user first to merge. For now assuming all fields are updated if present
    // or we use COALESCE in SQL. Here we simply use COALESCE logic in SQL or just update what we have.
    // simpler to just do dynamic query building or strict updates.
    // Let's assume for this template we just update name if provided, etc.
    // But sqlx macros don't like dynamic queries easily.
    // Let's do a simple implementation where we update all fields or specific ones.

    // For simplicity in this template, let's assume we fetch, merge in Rust (or just use simple query).

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET name = COALESCE($1, name),
            email = COALESCE($2, email),
            password = COALESCE($3, password),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING id, name, email, password, 
                  created_at::TEXT as "created_at!", 
                  updated_at::TEXT as "updated_at!"
        "#,
        req.name,
        req.email,
        req.password,
        id
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => DomainResult::Ok(user),
        Err(sqlx::Error::RowNotFound) => DomainResult::NotFound,
        Err(e) => {
            error!("Error updating user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

pub async fn delete_user(pool: &PgPool, id: i32) -> DomainResult<(), String> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                DomainResult::NotFound
            } else {
                DomainResult::Ok(())
            }
        }
        Err(e) => {
            error!("Error deleting user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}
