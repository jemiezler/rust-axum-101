use std::collections::HashMap;

use super::dtos::create::CreateUserRequest;
use super::dtos::update::UpdateUserRequest;
use super::entities::User;
use super::entities::name_entity::UserNameEntity;
use super::entities::people_name::{LocalizedName, PeopleName};
use super::entities::user_entity::UserEntity;
use crate::shared::security::password::hash_password;
use crate::shared::types::result::DomainResult;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

/// =========================
/// GET ALL USERS (BONUS: FIX N+1)
/// =========================
pub async fn get_all_users(pool: &PgPool) -> DomainResult<Vec<User>, String> {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    // 1️⃣ fetch users
    let users = match sqlx::query_as::<_, UserEntity>(
        r#"
        SELECT id, email, password, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&mut *tx)
    .await
    {
        Ok(u) => u,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    if users.is_empty() {
        tx.commit().await.ok();
        return DomainResult::Ok(vec![]);
    }

    // 2️⃣ fetch ALL names in one query (NO N+1)
    let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();

    let names = match sqlx::query_as::<_, UserNameEntity>(
        r#"
        SELECT user_id, lang, first_name, middle_name, last_name
        FROM user_names
        WHERE user_id = ANY($1)
        "#,
    )
    .bind(&user_ids)
    .fetch_all(&mut *tx)
    .await
    {
        Ok(u) => u,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    // 3️⃣ group names by user_id
    let mut name_map: HashMap<Uuid, HashMap<String, LocalizedName>> = HashMap::new();

    for n in names {
        name_map.entry(n.user_id).or_default().insert(
            n.lang,
            LocalizedName {
                first: n.first_name,
                middle: n.middle_name,
                last: n.last_name,
            },
        );
    }

    // 4️⃣ assemble domain users
    let result = users
        .into_iter()
        .map(|u| User {
            id: u.id,
            name: PeopleName {
                values: name_map.remove(&u.id).unwrap_or_default(),
            },
            email: u.email,
            password: u.password,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
        .collect();

    DomainResult::Ok(result)
}

/// =========================
/// FIND ONE USER
/// =========================
pub async fn find_one_user(pool: &PgPool, id: Uuid) -> DomainResult<User, String> {
    let user = match sqlx::query_as::<_, UserEntity>(
        r#"
        SELECT id, email, password, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    {
        Ok(Some(u)) => u,
        Ok(None) => return DomainResult::NotFound,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    let names = match sqlx::query_as::<_, UserNameEntity>(
        r#"
        SELECT user_id, lang, first_name, middle_name, last_name
        FROM user_names
        WHERE user_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await
    {
        Ok(n) => n,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    DomainResult::Ok(map_to_domain(user, names))
}

/// =========================
/// CREATE USER
/// =========================
pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> DomainResult<User, String> {
    let hashed_password = match hash_password(&req.password) {
        Ok(h) => h,
        Err(e) => return DomainResult::Err(e),
    };

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    let user = match sqlx::query_as::<_, UserEntity>(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING id, email, password, created_at, updated_at
        "#,
    )
    .bind(&req.email)
    .bind(hashed_password)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(u) => u,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    // 🔥 insert ALL locales dynamically
    for (lang, name) in &req.name.values {
        let res = sqlx::query(
            r#"
            INSERT INTO user_names (user_id, lang, first_name, middle_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user.id)
        .bind(lang)
        .bind(&name.first)
        .bind(&name.middle)
        .bind(&name.last)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            return DomainResult::Err(e.to_string());
        }
    }

    if let Err(e) = tx.commit().await {
        return DomainResult::Err(e.to_string());
    }

    DomainResult::Ok(User {
        id: user.id,
        name: req.name,
        email: user.email,
        password: user.password,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}

/// =========================
/// UPDATE USER
/// =========================
pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    req: UpdateUserRequest,
) -> DomainResult<User, String> {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    let password = match req.password {
        Some(p) => match hash_password(&p) {
            Ok(h) => Some(h),
            Err(e) => return DomainResult::Err(e),
        },
        None => None,
    };

    let user_res = sqlx::query_as::<_, UserEntity>(
        r#"
        UPDATE users
        SET
            email = COALESCE($1, email),
            password = COALESCE($2, password),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        RETURNING id, email, password, created_at, updated_at
        "#,
    )
    .bind(req.email)
    .bind(password)
    .bind(id)
    .fetch_optional(&mut *tx)
    .await;

    let _user = match user_res {
        Ok(Some(u)) => u,
        Ok(None) => return DomainResult::NotFound,
        Err(e) => return DomainResult::Err(e.to_string()),
    };

    if let Some(names) = req.name {
        let del_res = sqlx::query("DELETE FROM user_names WHERE user_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await;

        if let Err(e) = del_res {
            return DomainResult::Err(e.to_string());
        }

        for (lang, name) in names.values {
            let res = sqlx::query(
                r#"
                INSERT INTO user_names (user_id, lang, first_name, middle_name, last_name)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(id)
            .bind(lang)
            .bind(name.first)
            .bind(name.middle)
            .bind(name.last)
            .execute(&mut *tx)
            .await;

            if let Err(e) = res {
                return DomainResult::Err(e.to_string());
            }
        }
    }

    if let Err(e) = tx.commit().await {
        return DomainResult::Err(e.to_string());
    }

    find_one_user(pool, id).await
}

/// =========================
/// DELETE USER
/// =========================
pub async fn delete_user(pool: &PgPool, id: Uuid) -> DomainResult<(), String> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => DomainResult::NotFound,
        Ok(_) => DomainResult::Ok(()),
        Err(e) => {
            error!("Error deleting user: {:?}", e);
            DomainResult::Err(e.to_string())
        }
    }
}

/// =========================
/// MAP DB → DOMAIN
/// =========================
fn map_to_domain(entity: UserEntity, names: Vec<UserNameEntity>) -> User {
    let mut map = HashMap::new();

    for n in names {
        map.insert(
            n.lang,
            LocalizedName {
                first: n.first_name,
                middle: n.middle_name,
                last: n.last_name,
            },
        );
    }

    User {
        id: entity.id,
        name: PeopleName { values: map },
        email: entity.email,
        password: entity.password,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    }
}
