use super::{coin::Coin, entity::Entity};

#[derive(Clone, Debug)]
pub struct Position {
    pub entity: Entity,
    pub coin: Coin,
    pub quantity: f64,
}
