use serde::Deserialize;

use crate::{entities::pair_group::PairGroup, Error};

use super::interactor::Interactor;

pub trait DeletePairGroupDataAccess {
    async fn find_pair_group(&mut self, id: &str) -> Result<Option<PairGroup>, Error>;
    async fn delete_pair(&mut self, id: &str) -> Result<(), Error>;
    async fn delete_pair_group(&mut self, id: &str) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPairGroup {
    pub id: String,
}

impl PartialEq for RequestPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePairGroupRequest {
    pub pair_group: RequestPairGroup,
}

pub struct DeletePairGroup<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<DeletePairGroupRequest, ()> for DeletePairGroup<DA>
where
    DA: DeletePairGroupDataAccess,
{
    async fn perform(&mut self, request: DeletePairGroupRequest) -> Result<(), Error> {
        let maybe_pair_group = self
            .data_access
            .find_pair_group(&request.pair_group.id)
            .await?;
        if maybe_pair_group.is_none() {
            return Err(Error {
                message: String::from("Pair group to delete does not exist!"),
            });
        }
        let pair_group = maybe_pair_group.unwrap();
        for pair in &pair_group.pairs {
            self.data_access.delete_pair(&pair.id).await?;
        }
        self.data_access.delete_pair_group(&pair_group.id).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
