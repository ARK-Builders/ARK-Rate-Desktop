use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemPair {
    pub id: String,
    pub value: f64,
    pub base: String,
    pub comparison: String,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for FileSystemPair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for FileSystemPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.id.hash(state);
    }
}
