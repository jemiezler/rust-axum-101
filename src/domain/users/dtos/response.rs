use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::users::entities::User;
use crate::domain::users::entities::people_name::PersonName;
use crate::shared::types::hash::Hash;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: Hash<String, PersonName>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
