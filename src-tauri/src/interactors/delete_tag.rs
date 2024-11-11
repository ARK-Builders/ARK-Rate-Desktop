use serde::Deserialize;

use crate::Error;

use super::interactor::Interactor;

pub trait DeleteTagDataAccess {
    async fn delete_tag(&mut self, id: &str) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestTag {
    pub id: String,
}

impl PartialEq for RequestTag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteTagRequest {
    pub tag: RequestTag,
}

pub struct DeleteTag<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<DeleteTagRequest, ()> for DeleteTag<DA>
where
    DA: DeleteTagDataAccess,
{
    async fn perform(&mut self, request: DeleteTagRequest) -> Result<(), Error> {
        self.data_access.delete_tag(&request.tag.id).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
