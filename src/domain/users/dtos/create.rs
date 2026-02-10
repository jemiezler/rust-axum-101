use crate::{domain::users::entities::people_name::PersonName, shared::types::hash::Hash};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: Hash<String, PersonName>,
    pub email: String,
    pub password: String,
}
