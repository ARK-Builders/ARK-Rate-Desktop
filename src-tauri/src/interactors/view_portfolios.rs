use serde::Serialize;

use crate::{
    entities::{asset::Asset, pair::Pair, tag::Tag},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait ViewPortfoliosDataAccess {
    async fn fetch_tags(&mut self) -> Result<Vec<Tag>, Error>;
    async fn fetch_assets(&mut self) -> Result<Vec<Asset>, Error>;
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseTag {
    pub id: String,
    pub name: String,
}

impl PartialEq for ResponseTag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.name == other.name;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseAsset {
    pub id: String,
    pub coin: String,
    pub quantity: f64,
}

impl PartialEq for ResponseAsset {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.coin == other.coin && self.quantity == other.quantity;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePortfolio {
    pub usd_value: f64,
    pub tag: ResponseTag,
    pub asset: ResponseAsset,
}

impl PartialEq for ResponsePortfolio {
    fn eq(&self, other: &Self) -> bool {
        return self.usd_value == other.usd_value
            && self.tag == other.tag
            && self.asset == other.asset;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewPortfoliosResponse {
    portfolios: Vec<ResponsePortfolio>,
}

pub struct ViewPortfolios<DA, CM> {
    pub data_access: DA,
    pub coin_market: CM,
}

impl<DA, CM> Interactor<(), ViewPortfoliosResponse> for ViewPortfolios<DA, CM>
where
    DA: ViewPortfoliosDataAccess,
    CM: CoinMarket,
{
    async fn perform(&mut self, _request: ()) -> Result<ViewPortfoliosResponse, Error> {
        let tags = self.data_access.fetch_tags().await?;
        let assets = self.data_access.fetch_assets().await?;
        let usd_pairs = self.coin_market.retrieve_usd_pairs().await?;
        let portfolios = create_portfolios(&tags, &assets, &usd_pairs);
        return Ok(ViewPortfoliosResponse { portfolios });
    }
}

fn create_portfolios(
    tags: &Vec<Tag>,
    assets: &Vec<Asset>,
    usd_pairs: &Vec<Pair>,
) -> Vec<ResponsePortfolio> {
    return vec![];
}

#[cfg(test)]
mod test {
    // TODO: create tests for the interactor
}
