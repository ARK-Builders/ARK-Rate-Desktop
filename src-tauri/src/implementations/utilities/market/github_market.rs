use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    entities::{coin::Coin, create_entity::create_entity, pair::Pair},
    interactors::interactor::Error,
    utilities::market::Market,
};

pub trait GithubMarketDataAccess {
    async fn fetch_coins(&self) -> Result<Vec<Coin>, Error>;
}

pub struct GithubMarket<T> {
    pub data_access: T,
    pub fiat_rates_url: String,
    pub crypto_rates_url: String,
}

#[derive(Deserialize)]
struct FiatResponse {
    pub rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct CryptoObject {
    pub symbol: String,
    pub current_price: f64,
}

impl<T> Market for GithubMarket<T>
where
    T: GithubMarketDataAccess,
{
    async fn retrieve_updated_pairs_by_base_coin_code(&self, code: &str) -> Result<Vec<Pair>, Error> {
        let mut base_pairs: Vec<Pair> = vec![];
        let coins: Vec<Coin> = self.data_access.fetch_coins().await?;
        let maybe_base = find_coin_by_code(&coins, code);
        if maybe_base.is_none() {
            return Err(Error {
                message: String::from("Could not find base coin!"),
            });
        }
        let base: Coin = maybe_base.unwrap();
        let usd_fiat_pairs: Vec<Pair> =
            retrieve_updated_fiat_pairs_by_usd(&coins, &self.fiat_rates_url).await?;
        let usd_crypto_pairs: Vec<Pair> =
            retrieve_updated_crypto_pairs_by_usd(&coins, &self.crypto_rates_url).await?;
        let usd_pairs: Vec<Pair> = usd_fiat_pairs
            .iter()
            .chain(&usd_crypto_pairs)
            .cloned()
            .collect();
        if base.code == "USD" {
            return Ok(usd_pairs);
        }
        let maybe_usd_base_pair = find_usd_base_pair(&usd_pairs, &base.code);
        if maybe_usd_base_pair.is_none() {
            return Err(Error {
                message: String::from("Could not find the USD pair for the given base!"),
            });
        }
        let usd_base_pair: Pair = maybe_usd_base_pair.unwrap();
        let base_usd_value = 1.0 / usd_base_pair.value;
        for pair in &usd_pairs {
            if pair.comparison.code != base.code {
                let base_pair = Pair {
                    entity: create_entity(),
                    base: base.clone(),
                    comparison: pair.comparison.clone(),
                    value: pair.value * base_usd_value,
                };
                base_pairs.push(base_pair)
            }
        }
        return Ok(base_pairs);
    }
}

fn find_coin_by_code(coins: &Vec<Coin>, code: &str) -> Option<Coin> {
    for coin in coins {
        if coin.code == code {
            return Some(coin.clone());
        }
    }
    return None;
}

async fn retrieve_updated_fiat_pairs_by_usd(
    coins: &Vec<Coin>,
    url: &str,
) -> Result<Vec<Pair>, Error> {
    let maybe_base: Option<Coin> = find_coin_by_code(coins, "USD");
    if maybe_base.is_none() {
        return Err(Error {
            message: String::from("Could not find USD coin!"),
        });
    }
    let base: Coin = maybe_base.unwrap();
    match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let mut pairs: Vec<Pair> = vec![];
                let data: FiatResponse = serde_json::from_str(&text).unwrap();
                for rate in &data.rates {
                    let code = rate.0;
                    let value = rate.1;
                    if code == "USD" {
                        continue;
                    }
                    let maybe_comparison = find_coin_by_code(coins, code);
                    if maybe_comparison.is_none() {
                        return Err(Error {
                            message: String::from("Could not find comparison coin!"),
                        });
                    };
                    pairs.push(Pair {
                        entity: create_entity(),
                        base: base.clone(),
                        comparison: maybe_comparison.unwrap(),
                        value: value.clone(),
                    });
                }
                return Ok(pairs);
            }
            Err(_) => {}
        },
        Err(_) => {}
    }
    return Err(Error {
        message: String::from("Unexpected error while fetching fiat pairs!"),
    });
}

async fn retrieve_updated_crypto_pairs_by_usd(
    coins: &Vec<Coin>,
    url: &str,
) -> Result<Vec<Pair>, Error> {
    let maybe_base: Option<Coin> = find_coin_by_code(coins, "USD");
    if maybe_base.is_none() {
        return Err(Error {
            message: String::from("Could not find USD coin!"),
        });
    }
    let base: Coin = maybe_base.unwrap();
    match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let mut pairs: Vec<Pair> = vec![];
                let data: Vec<CryptoObject> = serde_json::from_str(&text).unwrap();
                for crypto_object in &data {
                    let code: &String = &crypto_object.symbol.to_uppercase();
                    let value = &crypto_object.current_price;
                    let maybe_comparison = find_coin_by_code(coins, code);
                    if maybe_comparison.is_none() {
                        return Err(Error {
                            message: String::from("Could not find comparison coin!"),
                        });
                    };
                    pairs.push(Pair {
                        entity: create_entity(),
                        base: base.clone(),
                        comparison: maybe_comparison.unwrap(),
                        value: value.clone(),
                    });
                }
                return Ok(pairs);
            }
            Err(_) => {}
        },
        Err(_) => {}
    }
    return Err(Error {
        message: String::from("Unexpected error while fetching crypto pairs!"),
    });
}

fn find_usd_base_pair(usd_pairs: &Vec<Pair>, comparison_code: &str) -> Option<Pair> {
    for pair in usd_pairs {
        if pair.comparison.code == comparison_code {
            return Some(pair.clone());
        }
    }
    return None;
}

