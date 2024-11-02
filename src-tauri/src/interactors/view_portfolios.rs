use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

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
        return self.id == other.id && self.name == other.name;
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
        let portfolios = create_portfolios(&tags, &assets, &usd_pairs)?;
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
    usd_pairs: &Vec<Pair>,
) -> Result<Vec<ResponsePortfolio>, Error> {
    let mut portfolios: Vec<ResponsePortfolio> = vec![];
    for tag in tags {
        for asset in &tag.assets {
            let portfolio = create_portfolio(tag, asset, usd_pairs)?;
            portfolios.push(portfolio);
        }
    }
    let standalone_assets = retrieve_standalone_assets(tags, assets);
    if standalone_assets.len() > 0 {
        let default_tag = ResponseTag {
            id: Uuid::new_v4().to_string(),
            name: String::from("Untagged"),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        for asset in &standalone_assets {
            let equivalent_usd_value = get_equivalent_usd_value(usd_pairs, &asset.coin)?;
            portfolios.push(ResponsePortfolio {
                asset: asset.clone(),
                tag: default_tag.clone(),
                usd_value: equivalent_usd_value * asset.quantity,
            });
        }
    }
    return Ok(portfolios);
}

fn create_portfolio(
    tag: &Tag,
    asset: &Asset,
    usd_pairs: &Vec<Pair>,
) -> Result<ResponsePortfolio, Error> {
    let equivalent_usd_value = get_equivalent_usd_value(usd_pairs, &asset.coin)?;
    return Ok(ResponsePortfolio {
        usd_value: equivalent_usd_value * asset.quantity,
        tag: ResponseTag {
            id: tag.id.clone(),
            name: tag.name.clone(),
            created_at: tag.created_at.clone(),
            updated_at: tag.updated_at.clone(),
        },
        asset: ResponseAsset {
            id: asset.id.clone(),
            coin: asset.coin.clone(),
            quantity: asset.quantity.clone(),
            created_at: asset.created_at.clone(),
            updated_at: asset.updated_at.clone(),
        },
    });
}

// TODO: create an utils module containing this function
fn get_equivalent_usd_value(usd_pairs: &Vec<Pair>, target_base: &str) -> Result<f64, Error> {
    if target_base == "USD" {
        return Ok(1.0);
    }
    for usd_pair in usd_pairs {
        if usd_pair.comparison == target_base {
            return Ok(1.0 / usd_pair.value);
        }
    }
    return Err(Error {
        message: String::from("Could not find the equivalent USD value for the target base!"),
    });
}

fn retrieve_standalone_assets(tags: &Vec<Tag>, assets: &Vec<Asset>) -> Vec<ResponseAsset> {
    let mut standalone_assets: Vec<ResponseAsset> = vec![];
    for asset in assets {
        let mut is_standalone = true;
        for tag in tags {
            for asset_tag in &tag.assets {
                if asset_tag.id == asset.id {
                    is_standalone = false;
                    break;
                }
            }
            if !is_standalone {
                break;
            }
        }
        if is_standalone {
            standalone_assets.push(ResponseAsset {
                id: asset.id.clone(),
                coin: asset.coin.clone(),
                quantity: asset.quantity.clone(),
                created_at: asset.created_at.clone(),
                updated_at: asset.updated_at.clone(),
            });
        }
    }
    return standalone_assets;
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct ViewPortfoliosDataAccessMock {
        fetch_tags_result: Result<Vec<Tag>, Error>,
        fetch_assets_result: Result<Vec<Asset>, Error>,
    }

    impl ViewPortfoliosDataAccess for ViewPortfoliosDataAccessMock {
        async fn fetch_tags(&mut self) -> Result<Vec<Tag>, Error> {
            return self.fetch_tags_result.clone();
        }

        async fn fetch_assets(&mut self) -> Result<Vec<Asset>, Error> {
            return self.fetch_assets_result.clone();
        }
    }

    pub struct CoinMarketMock {
        retrieve_pairs_by_base_result: Result<Vec<Pair>, Error>,
    }

    impl CoinMarket for CoinMarketMock {
        async fn retrieve_usd_pairs(&mut self) -> Result<Vec<Pair>, Error> {
            return self.retrieve_pairs_by_base_result.clone();
        }
    }

    #[tokio::test]
    async fn test_tagged_assets() {
        let example_assets: Vec<Asset> = vec![
            Asset {
                id: "a1".to_string(),
                quantity: 10.0,
                coin: "USD".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a2".to_string(),
                quantity: 20.0,
                coin: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a3".to_string(),
                quantity: 30.0,
                coin: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let example_tags: Vec<Tag> = vec![
            Tag {
                id: "t1".to_string(),
                name: "Tag One".to_string(),
                assets: vec![example_assets[0].clone(), example_assets[1].clone()],
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Tag {
                id: "t2".to_string(),
                name: "Tag Two".to_string(),
                assets: vec![example_assets[1].clone(), example_assets[2].clone()],
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let data_access = ViewPortfoliosDataAccessMock {
            fetch_tags_result: Ok(example_tags.clone()),
            fetch_assets_result: Ok(example_assets.clone()),
        };

        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };

        let mut interactor = ViewPortfolios {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.portfolios.len(), 4);

        assert!(response.portfolios.contains(&ResponsePortfolio {
            usd_value: 10.0,
            tag: ResponseTag {
                id: "t1".to_string(),
                name: "Tag One".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            asset: ResponseAsset {
                id: "a1".to_string(),
                quantity: 10.0,
                coin: "USD".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        }));
        assert!(response.portfolios.contains(&ResponsePortfolio {
            usd_value: 20.0 / 5.0,
            tag: ResponseTag {
                id: "t1".to_string(),
                name: "Tag One".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            asset: ResponseAsset {
                id: "a2".to_string(),
                quantity: 20.0,
                coin: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        }));
        assert!(response.portfolios.contains(&ResponsePortfolio {
            usd_value: 20.0 / 5.0,
            tag: ResponseTag {
                id: "t2".to_string(),
                name: "Tag Two".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            asset: ResponseAsset {
                id: "a2".to_string(),
                quantity: 20.0,
                coin: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        }));
        assert!(response.portfolios.contains(&ResponsePortfolio {
            usd_value: 30.0 / 6.0,
            tag: ResponseTag {
                id: "t2".to_string(),
                name: "Tag Two".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            asset: ResponseAsset {
                id: "a3".to_string(),
                quantity: 30.0,
                coin: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        }));
    }

    #[tokio::test]
    async fn test_untagged_assets() {
        let example_assets: Vec<Asset> = vec![
            Asset {
                id: "a1".to_string(),
                quantity: 10.0,
                coin: "USD".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a2".to_string(),
                quantity: 20.0,
                coin: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a3".to_string(),
                quantity: 30.0,
                coin: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let data_access = ViewPortfoliosDataAccessMock {
            fetch_tags_result: Ok(vec![]),
            fetch_assets_result: Ok(example_assets.clone()),
        };

        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };

        let mut interactor = ViewPortfolios {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.portfolios.len(), 3);

        // TODO: assert the response portfolios correctness
    }

    // TODO: create an "untagged and tagged" unit test
    #[tokio::test]
    async fn test_untagged_and_tagged_assets() {
        let example_assets: Vec<Asset> = vec![
            Asset {
                id: "a1".to_string(),
                quantity: 10.0,
                coin: "USD".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a2".to_string(),
                quantity: 20.0,
                coin: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Asset {
                id: "a3".to_string(),
                quantity: 30.0,
                coin: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let example_tags: Vec<Tag> = vec![Tag {
            id: "t1".to_string(),
            name: "Tag One".to_string(),
            assets: vec![example_assets[0].clone(), example_assets[1].clone()],
            created_at: String::from("2024-01-01T10:00:00+00:00"),
            updated_at: String::from("2024-01-01T10:00:00+00:00"),
        }];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "EUR".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T10:00:00+00:00"),
            },
        ];

        let data_access = ViewPortfoliosDataAccessMock {
            fetch_tags_result: Ok(example_tags.clone()),
            fetch_assets_result: Ok(example_assets.clone()),
        };

        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };

        let mut interactor = ViewPortfolios {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.portfolios.len(), 3);

        // TODO: assert the response portfolios correctness
    }
}
