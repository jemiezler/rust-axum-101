use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonName {
    pub first: String,
    pub middle: String,
    pub last: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocalizedPersonName {
    pub values: HashMap<String, PersonName>,
}
