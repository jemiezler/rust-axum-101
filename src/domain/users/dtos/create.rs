use crate::domain::users::entities::people_name::PeopleName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: PeopleName,
    pub email: String,
    pub password: String,
}
