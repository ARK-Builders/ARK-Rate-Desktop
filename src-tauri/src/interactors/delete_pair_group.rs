use serde::{Deserialize, Serialize};

use crate::Error;

use super::interactor::Interactor;

pub trait DeletePairGroupDataAccess {
    async fn delete_pair_group(&mut self, id: &str) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestPairGroup {
    pub id: String,
}

impl PartialEq for RequestPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletePairGroupRequest {
    pair_group: RequestPairGroup,
}

pub struct DeletePairGroup<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<DeletePairGroupRequest, ()> for DeletePairGroup<DA>
where
    DA: DeletePairGroupDataAccess,
{
    async fn perform(&mut self, request: DeletePairGroupRequest) -> Result<(), Error> {
        /*
           TODO (NOT SURE):
               - Make sure the pair group exists.
        */
        self.data_access
            .delete_pair_group(&request.pair_group.id)
            .await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
