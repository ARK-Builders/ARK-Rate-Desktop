use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemPairGroup {
    pub id: String,
    pub is_pinned: bool,
    pub multiplier: f64,
    pub pairs: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for FileSystemPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.is_pinned == other.is_pinned
            && self.multiplier == other.multiplier
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for FileSystemPairGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.id.hash(state);
    }
}
