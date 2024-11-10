use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{asset::Asset, tag::Tag},
    Error,
};

use super::interactor::Interactor;

pub trait StorePortfoliosDataAccess {
    async fn find_tag(&mut self, id: &str) -> Result<Option<Tag>, Error>;
    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error>;
    async fn save_asset(&mut self, asset: &Asset) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestTag {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestAsset {
    pub coin: String,
    pub quantity: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StorePortfoliosRequest {
    pub tag: Option<RequestTag>,
    pub assets: Vec<RequestAsset>,
}

pub struct StorePortfolios<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<StorePortfoliosRequest, ()> for StorePortfolios<DA>
where
    DA: StorePortfoliosDataAccess,
{
    async fn perform(&mut self, request: StorePortfoliosRequest) -> Result<(), Error> {
        if request.tag.is_none() {
            return store_asset(&mut self.data_access, &request).await;
        } else {
            return store_tagged_asset(&mut self.data_access, &request).await;
        }
    }
}

async fn store_asset(
    data_access: &mut impl StorePortfoliosDataAccess,
    request: &StorePortfoliosRequest,
) -> Result<(), Error> {
    for request_asset in &request.assets {
        let asset = Asset {
            id: Uuid::new_v4().to_string(),
            coin: request_asset.coin.clone(),
            quantity: request_asset.quantity.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        data_access.save_asset(&asset).await?;
    }
    return Ok(());
}

async fn store_tagged_asset(
    data_access: &mut impl StorePortfoliosDataAccess,
    request: &StorePortfoliosRequest,
) -> Result<(), Error> {
    let request_tag = request.tag.clone().unwrap();
    let maybe_tag = data_access.find_tag(&request_tag.id).await?;
    if maybe_tag.is_none() {
        return Err(Error {
            message: String::from("Tag not found!"),
        });
    }
    let mut tag = maybe_tag.unwrap();
    for request_asset in &request.assets {
        let asset = Asset {
            id: Uuid::new_v4().to_string(),
            coin: request_asset.coin.clone(),
            quantity: request_asset.quantity.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        tag.assets.push(asset);
    }
    return data_access.update_tag(&tag).await;
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
