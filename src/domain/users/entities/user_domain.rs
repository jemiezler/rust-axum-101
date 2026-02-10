use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::users::entities::people_name::LocalizedPersonName;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: LocalizedPersonName,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
