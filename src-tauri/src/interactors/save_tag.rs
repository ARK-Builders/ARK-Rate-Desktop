use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{entities::tag::Tag, Error};

use super::interactor::Interactor;

pub trait SaveTagDataAccess {
    async fn save_tag(&mut self, tag: &Tag) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestTag {
    pub name: String,
}

impl PartialEq for RequestTag {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SaveTagRequest {
    tag: RequestTag,
}

pub struct SaveTag<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<SaveTagRequest, ()> for SaveTag<DA>
where
    DA: SaveTagDataAccess,
{
    async fn perform(&mut self, request: SaveTagRequest) -> Result<(), Error> {
        let tag = Tag {
            id: Uuid::new_v4().to_string(),
            assets: vec![],
            name: request.tag.name,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.data_access.save_tag(&tag).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
