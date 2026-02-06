use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocalizedName {
    pub first: String,
    pub middle: String,
    pub last: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PeopleName {
    pub values: HashMap<String, LocalizedName>,
}
