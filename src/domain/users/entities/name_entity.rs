use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserNameEntity {
    pub user_id: Uuid,
    pub lang: String, // "th", "en"
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}
