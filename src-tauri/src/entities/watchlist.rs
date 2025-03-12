use std::hash::Hash;

use super::pair::Pair;

#[derive(Clone, Debug)]
pub struct Watchlist {
    pub id: String,
    pub pairs: Vec<Pair>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for Watchlist {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for Watchlist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
