use super::{coin::Coin, entity::Entity};

#[derive(Clone, Debug)]
pub struct Pair {
    pub entity: Entity,
    pub base: Coin,
    pub comparison: Coin,
    pub value: f64, // this represents how much `comparison` values compared to `base`
}
