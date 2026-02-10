use crate::shared::types::hash::Hash;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct School {
    pub id: Uuid,
    pub name: Hash<String, String>,
    pub acronym: Hash<String, String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
