use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileSystemAsset {
    pub id: String,
    pub coin: String,
    pub quantity: f64,
    pub usd_value: f64,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for FileSystemAsset {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.coin == other.coin
            && self.quantity == other.quantity
            && self.usd_value == other.usd_value
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for FileSystemAsset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
