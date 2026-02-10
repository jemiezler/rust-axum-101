use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(bound(
    deserialize = "K: std::cmp::Eq + std::hash::Hash + Deserialize<'de>, V: Deserialize<'de>"
))]
pub struct Hash<K, V> {
    pub values: HashMap<K, V>,
}

impl<K, V> Hash<K, V> {
    pub fn new(values: HashMap<K, V>) -> Self {
        Self { values }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: std::hash::Hash + Eq,
    {
        self.values.get(key)
    }

    pub fn get_or_default(&self, key: &K) -> V
    where
        K: std::hash::Hash + Eq,
        V: Clone + Default,
    {
        self.values.get(key).cloned().unwrap_or_default()
    }
}
