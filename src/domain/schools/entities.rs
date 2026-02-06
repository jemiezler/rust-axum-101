use crate::shared::types::locale::Locale;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct School {
    pub id: Uuid,
    pub name: Locale,
    pub acronym: Locale,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
