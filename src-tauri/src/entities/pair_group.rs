use super::{entity::Entity, pair::Pair};

#[derive(Clone, Debug)]
pub struct PairGroup {
    pub entity: Entity,
    pub pairs: Vec<Pair>,
    pub is_pinned: bool,
}
