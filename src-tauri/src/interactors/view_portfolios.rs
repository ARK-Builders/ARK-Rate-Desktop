use serde::Serialize;

use crate::{
    entities::{asset::Asset, tag::Tag},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait ViewPortfoliosDataAccess {
    async fn fetch_tags(&mut self) -> Result<Vec<Tag>, Error>;
    async fn fetch_assets(&mut self) -> Result<Vec<Asset>, Error>;
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePair {
    pub id: String,
    pub value: f64,
    pub base: String,
    pub comparison: String,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponsePair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseTag {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponseTag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseAsset {
    pub id: String,
    pub coin: String,
    pub quantity: f64,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponseAsset {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.coin == other.coin
            && self.quantity == other.quantity
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePortfolio {
    pub tags: Vec<ResponseTag>,
    pub asset: ResponseAsset,
}

impl PartialEq for ResponsePortfolio {
    fn eq(&self, other: &Self) -> bool {
        return self.tags == other.tags && self.asset == other.asset;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewPortfoliosResponse {
    pub usd_pairs: Vec<ResponsePair>,
    pub portfolios: Vec<ResponsePortfolio>,
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
        let portfolios = create_portfolios(&tags, &assets)?;
        return Ok(ViewPortfoliosResponse {
            portfolios,
            usd_pairs: usd_pairs
                .iter()
                .map(|p| ResponsePair {
                    id: p.id.clone(),
                    base: p.base.clone(),
                    value: p.value.clone(),
                    comparison: p.comparison.clone(),
                    created_at: p.created_at.clone(),
                    updated_at: p.updated_at.clone(),
                })
                .collect(),
        });
    }
}

fn create_portfolios(
    tags: &Vec<Tag>,
    assets: &Vec<Asset>,
) -> Result<Vec<ResponsePortfolio>, Error> {
    let mut portfolios: Vec<ResponsePortfolio> = assets
        .iter()
        .map(|a| ResponsePortfolio {
            tags: vec![],
            asset: ResponseAsset {
                id: a.id.clone(),
                coin: a.coin.clone(),
                quantity: a.quantity.clone(),
                created_at: a.created_at.clone(),
                updated_at: a.updated_at.clone(),
            },
        })
        .collect();
    for tag in tags {
        let response_tag = ResponseTag {
            id: tag.id.clone(),
            name: tag.name.clone(),
            created_at: tag.created_at.clone(),
            updated_at: tag.updated_at.clone(),
        };
        for portfolio in &mut portfolios {
            for tag_asset in &tag.assets {
                if tag_asset.id == portfolio.asset.id {
                    portfolio.tags.push(response_tag.clone());
                    break;
                }
            }
        }
    }
    return Ok(portfolios);
}

#[cfg(test)]
mod test {
    // TODO: create unit tests for this impl
}
