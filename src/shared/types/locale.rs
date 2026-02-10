use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    pub values: HashMap<String, String>,
}
