use chrono::Local;
use uuid::Uuid;

use super::entity::Entity;

pub fn create_entity() -> Entity {
    return Entity {
        id: Uuid::new_v4().to_string(),
        created_at: Local::now().to_utc(),
        updated_at: Local::now().to_utc(),
    }
}
