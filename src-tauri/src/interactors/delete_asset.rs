use serde::Deserialize;

use crate::{entities::tag::Tag, Error};

use super::interactor::Interactor;

pub trait DeleteAssetDataAccess {
    async fn retrieve_tags_by_asset(&mut self, id: &str) -> Result<Vec<Tag>, Error>;
    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error>;
    async fn delete_asset(&mut self, id: &str) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestAsset {
    pub id: String,
}

impl PartialEq for RequestAsset {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteAssetRequest {
    pub asset: RequestAsset,
}

// TODO: should it be called `DeletePortfolio` instead?
pub struct DeleteAsset<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<DeleteAssetRequest, ()> for DeleteAsset<DA>
where
    DA: DeleteAssetDataAccess,
{
    async fn perform(&mut self, request: DeleteAssetRequest) -> Result<(), Error> {
        let mut tags = self
            .data_access
            .retrieve_tags_by_asset(&request.asset.id)
            .await?;
        for tag in &mut tags {
            if let Some(asset_idx) = tag.assets.iter().position(|a| a.id == request.asset.id) {
                tag.assets.remove(asset_idx);
                self.data_access.update_tag(tag).await?;
            }
        }
        self.data_access.delete_asset(&request.asset.id).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
