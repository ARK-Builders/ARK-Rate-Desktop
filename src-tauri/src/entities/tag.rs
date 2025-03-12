use std::hash::Hash;

use super::asset::Asset;

#[derive(Clone, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub assets: Vec<Asset>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.assets == other.assets
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}
