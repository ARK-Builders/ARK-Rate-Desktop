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
    async fn update_asset(&mut self, asset: &Asset) -> Result<(), Error>;
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
    pub usd_value: f64,
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
    pub fluctuation: f64,
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
    pub tags: Vec<ResponseTag>,
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

        let fresh_assets = refresh_assets(&usd_pairs, &assets)?;
        for fresh_assets in &fresh_assets {
            self.data_access.update_asset(&fresh_assets).await?;
        }

        let portfolios = create_portfolios(&tags, &assets, &fresh_assets)?;
        return Ok(ViewPortfoliosResponse {
            portfolios,
            tags: tags
                .iter()
                .map(|t| ResponseTag {
                    id: t.id.clone(),
                    name: t.name.clone(),
                    created_at: t.created_at.clone(),
                    updated_at: t.updated_at.clone(),
                })
                .collect(),
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

fn refresh_assets(usd_pairs: &Vec<Pair>, assets: &Vec<Asset>) -> Result<Vec<Asset>, Error> {
    let mut fresh_assets: Vec<Asset> = vec![];
    for asset in assets {
        let usd_value = get_equivalent_usd_value(usd_pairs, &asset.coin)?;
        fresh_assets.push(Asset {
            id: asset.id.clone(),
            usd_value,
            coin: asset.coin.clone(),
            quantity: asset.quantity.clone(),
            created_at: asset.created_at.clone(),
            updated_at: asset.updated_at.clone(),
        });
    }
    return Ok(fresh_assets);
}

// TODO: maybe extract it to a utility function, since it seems to be used in several code parts
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

fn create_portfolios(
    tags: &Vec<Tag>,
    assets: &Vec<Asset>,
    fresh_assets: &Vec<Asset>,
) -> Result<Vec<ResponsePortfolio>, Error> {
    let mut portfolios: Vec<ResponsePortfolio> = fresh_assets
        .iter()
        .map(|fa| {
            let mut fluctuation = 0.0;
            if let Some(asset) = assets.iter().find(|a| a.id == fa.id) {
                let usd_value = asset.usd_value;
                let fresh_usd_value = fa.usd_value;
                let usd_difference = fresh_usd_value - usd_value;
                fluctuation = usd_difference / usd_value;
            }
            return ResponsePortfolio {
                fluctuation,
                tags: vec![],
                asset: ResponseAsset {
                    id: fa.id.clone(),
                    coin: fa.coin.clone(),
                    quantity: fa.quantity.clone(),
                    usd_value: fa.usd_value.clone(),
                    created_at: fa.created_at.clone(),
                    updated_at: fa.updated_at.clone(),
                },
            };
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
