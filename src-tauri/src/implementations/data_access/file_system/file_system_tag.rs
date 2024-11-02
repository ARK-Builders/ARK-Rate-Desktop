use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemTag {
    pub id: String,
    pub name: String,
    pub assets: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for FileSystemTag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.assets == other.assets
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for FileSystemTag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
