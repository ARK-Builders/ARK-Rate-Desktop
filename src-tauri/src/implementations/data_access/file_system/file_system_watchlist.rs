use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemWatchlist {
    pub id: String,
    pub pairs: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for FileSystemWatchlist {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for FileSystemWatchlist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.id.hash(state);
    }
}
