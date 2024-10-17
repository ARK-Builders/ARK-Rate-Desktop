use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
