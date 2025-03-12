use std::hash::Hash;

use super::pair::Pair;

#[derive(Clone, Debug)]
pub struct PairGroup {
    pub id: String,
    pub is_pinned: bool,
    pub multiplier: f64,
    pub pairs: Vec<Pair>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for PairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.is_pinned == other.is_pinned
            && self.multiplier == other.multiplier
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

impl Hash for PairGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
