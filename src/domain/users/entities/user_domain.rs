use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::users::entities::people_name::PeopleName;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: PeopleName,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
