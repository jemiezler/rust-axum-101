use super::entities::{CreateUserRequest, UpdateUserRequest, User};
use crate::shared::{security::password::hash_password, types::result::DomainResult};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

pub async fn get_all_users(pool: &PgPool) -> DomainResult<Vec<User>, String> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            name,
            email,
            password,
            created_at,
            updated_at
        FROM users
        "#,
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

pub async fn find_one_user(pool: &PgPool, id: Uuid) -> DomainResult<User, String> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            name,
            email,
            password,
            created_at,
            updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
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
    let hashed_password = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => return DomainResult::Err(e),
    };
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            name,
            email,
            password,
            created_at,
            updated_at
        "#,
    )
    .bind(req.name)
    .bind(req.email)
    .bind(hashed_password)
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
    id: Uuid,
    req: UpdateUserRequest,
) -> DomainResult<User, String> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET
            name = COALESCE($1, name),
            email = COALESCE($2, email),
            password = COALESCE($3, password),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING
            id,
            name,
            email,
            password,
            created_at,
            updated_at
        "#,
    )
    .bind(req.name)
    .bind(req.email)
    .bind(req.password)
    .bind(id)
    .fetch_optional(pool)
    .await;

    match user {
        Ok(Some(user)) => DomainResult::Ok(user),
        Ok(None) => DomainResult::NotFound,
        Err(e) => {
            error!("Error updating user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> DomainResult<(), String> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
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
