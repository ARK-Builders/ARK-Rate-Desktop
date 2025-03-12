use chrono::Utc;
use serde::Deserialize;

use crate::{
    entities::{asset::Asset, tag::Tag},
    Error,
};

use super::interactor::Interactor;

pub trait UpdatePortfolioDataAccess {
    async fn retrieve_tags_by_asset(&mut self, id: &str) -> Result<Vec<Tag>, Error>;
    async fn find_tag(&mut self, id: &str) -> Result<Option<Tag>, Error>;
    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error>;
    async fn find_asset(&mut self, id: &str) -> Result<Option<Asset>, Error>;
    async fn update_asset(&mut self, asset: &Asset) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestAsset {
    pub id: String,
    pub coin: String,
    pub quantity: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePortfolioRequest {
    pub tag_ids: Vec<String>,
    pub asset: RequestAsset,
}

// TODO: should it be called `UpdateAsset` instead?
pub struct UpdatePortfolio<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<UpdatePortfolioRequest, ()> for UpdatePortfolio<DA>
where
    DA: UpdatePortfolioDataAccess,
{
    async fn perform(&mut self, request: UpdatePortfolioRequest) -> Result<(), Error> {
        let maybe_asset = self.data_access.find_asset(&request.asset.id).await?;
        if maybe_asset.is_none() {
            return Err(Error {
                message: String::from("Asset does not exist!"),
            });
        }
        let asset = maybe_asset.unwrap();
        let updated_asset = Asset {
            id: asset.id.clone(),
            coin: request.asset.coin.clone(),
            quantity: request.asset.quantity.clone(),
            usd_value: asset.usd_value.clone(),
            created_at: asset.created_at.clone(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.data_access.update_asset(&updated_asset).await?;

        let mut request_tags: Vec<Tag> = vec![];
        for tag_id in &request.tag_ids {
            let maybe_tag = self.data_access.find_tag(tag_id).await?;
            if maybe_tag.is_none() {
                return Err(Error {
                    message: String::from("One of the tags was not found!"),
                });
            }
            let tag = maybe_tag.unwrap();
            request_tags.push(tag);
        }

        let mut updated_tags: Vec<Tag> = vec![];
        let current_tags = self.data_access.retrieve_tags_by_asset(&asset.id).await?;

        for current_tag in &current_tags {
            let mut is_removed = true;
            for request_tag in &request_tags {
                if request_tag.id == current_tag.id {
                    is_removed = false;
                    break;
                }
            }
            if is_removed {
                if let Some(asset_idx) = current_tag.assets.iter().position(|a| a.id == asset.id) {
                    let mut updated_tag = current_tag.clone();
                    updated_tag.assets.remove(asset_idx);
                    updated_tags.push(updated_tag);
                }
            }
        }

        for request_tag in &request_tags {
            let mut is_added = true;
            for current_tag in &current_tags {
                if current_tag.id == request_tag.id {
                    is_added = false;
                    break;
                }
            }
            if is_added {
                let mut updated_tag = request_tag.clone();
                updated_tag.assets.push(updated_asset.clone());
                updated_tags.push(updated_tag);
            }
        }

        for updated_tag in &updated_tags {
            self.data_access.update_tag(&updated_tag).await?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
