use crate::domain::users::entities::people_name::LocalizedPersonName ;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: LocalizedPersonName ,
    pub email: String,
    pub password: String,
}
