use crate::{domain::users::entities::people_name::PersonName, shared::types::hash::Hash};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Hash<String, PersonName>,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
