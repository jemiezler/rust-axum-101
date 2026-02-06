use crate::domain::users::entities::people_name::PeopleName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<PeopleName>,
    pub email: Option<String>,
    pub password: Option<String>,
}
