use crate::{domain::users::entities::people_name::PersonName, shared::types::hash::Hash};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<Hash<String, PersonName>>,
    pub email: Option<String>,
    pub password: Option<String>,
}
