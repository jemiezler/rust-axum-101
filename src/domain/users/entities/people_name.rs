use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonName {
    pub first: String,
    pub middle: String,
    pub last: String,
}
