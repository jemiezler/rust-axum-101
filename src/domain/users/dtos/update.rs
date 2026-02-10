use crate::domain::users::entities::people_name::LocalizedPersonName ;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<LocalizedPersonName >,
    pub email: Option<String>,
    pub password: Option<String>,
}
