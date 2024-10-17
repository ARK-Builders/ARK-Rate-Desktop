use super::{entity::Entity, position::Position};

#[derive(Clone, Debug)]
pub struct Portfolio {
    pub entity: Entity,
    pub positions: Vec<Position>,
}
