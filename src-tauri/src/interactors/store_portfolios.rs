use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{asset::Asset, pair::Pair, tag::Tag},
    utilities::coin_market::CoinMarket,
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

// TODO: should it be called `StoreAssets` instead?
pub struct StorePortfolios<DA, CM> {
    pub data_access: DA,
    pub coin_market: CM,
}

impl<DA, CM> Interactor<StorePortfoliosRequest, ()> for StorePortfolios<DA, CM>
where
    DA: StorePortfoliosDataAccess,
    CM: CoinMarket,
{
    async fn perform(&mut self, request: StorePortfoliosRequest) -> Result<(), Error> {
        if request.tag.is_none() {
            return store_assets(&mut self.data_access, &mut self.coin_market, &request).await;
        } else {
            return store_tagged_assets(&mut self.data_access, &mut self.coin_market, &request)
                .await;
        }
    }
}

async fn store_assets(
    data_access: &mut impl StorePortfoliosDataAccess,
    coin_market: &mut impl CoinMarket,
    request: &StorePortfoliosRequest,
) -> Result<(), Error> {
    let usd_pairs = coin_market.retrieve_usd_pairs().await?;
    for request_asset in &request.assets {
        let usd_value = get_equivalent_usd_value(&usd_pairs, &request_asset.coin)?;
        let asset = Asset {
            id: Uuid::new_v4().to_string(),
            usd_value,
            coin: request_asset.coin.clone(),
            quantity: request_asset.quantity.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        data_access.save_asset(&asset).await?;
    }
    return Ok(());
}

fn get_equivalent_usd_value(usd_pairs: &Vec<Pair>, target_base: &str) -> Result<f64, Error> {
    for usd_pair in usd_pairs {
        if usd_pair.comparison == target_base {
            return Ok(1.0 / usd_pair.value);
        }
    }
    return Err(Error {
        message: String::from("Could not find the equivalent USD value for the target base!"),
    });
}

async fn store_tagged_assets(
    data_access: &mut impl StorePortfoliosDataAccess,
    coin_market: &mut impl CoinMarket,
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
    let usd_pairs = coin_market.retrieve_usd_pairs().await?;
    for request_asset in &request.assets {
        let usd_value = get_equivalent_usd_value(&usd_pairs, &request_asset.coin)?;
        let asset = Asset {
            id: Uuid::new_v4().to_string(),
            usd_value,
            coin: request_asset.coin.clone(),
            quantity: request_asset.quantity.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        data_access.save_asset(&asset).await?;
        tag.assets.push(asset);
    }
    return data_access.update_tag(&tag).await;
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
