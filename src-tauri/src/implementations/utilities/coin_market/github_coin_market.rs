use std::collections::HashMap;

use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{entities::pair::Pair, utilities::coin_market::CoinMarket, Error};

pub struct GithubCoinMarket {
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

impl CoinMarket for GithubCoinMarket {
    async fn retrieve_pairs_by_base(&mut self, base: &str) -> Result<Vec<Pair>, Error> {
        let mut base_pairs: Vec<Pair> = vec![];
        let usd_fiat_pairs: Vec<Pair> =
            retrieve_updated_fiat_pairs_by_usd(&self.fiat_rates_url).await?;
        let usd_crypto_pairs: Vec<Pair> =
            retrieve_updated_crypto_pairs_by_usd(&self.crypto_rates_url).await?;
        let usd_pairs: Vec<Pair> = usd_fiat_pairs
            .iter()
            .chain(&usd_crypto_pairs)
            .cloned()
            .collect();
        if base == "USD" {
            return Ok(usd_pairs);
        }
        let maybe_usd_base_pair = find_usd_base_pair(&usd_pairs, &base);
        if maybe_usd_base_pair.is_none() {
            return Err(Error {
                message: String::from("Could not find the USD pair for the given base!"),
            });
        }
        let usd_base_pair: Pair = maybe_usd_base_pair.unwrap();
        let base_usd_value = 1.0 / usd_base_pair.value;
        for pair in &usd_pairs {
            if pair.comparison != base {
                let base_pair = Pair {
                    id: Uuid::new_v4().to_string(),
                    value: pair.value * base_usd_value,
                    base: base.to_string(),
                    comparison: pair.comparison.clone(),
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                };
                base_pairs.push(base_pair)
            }
        }
        return Ok(base_pairs);
    }
}

async fn retrieve_updated_fiat_pairs_by_usd(url: &str) -> Result<Vec<Pair>, Error> {
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
                    pairs.push(Pair {
                        id: Uuid::new_v4().to_string(),
                        value: value.clone(),
                        base: String::from("USD"),
                        comparison: code.to_string(),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    });
                }
                return Ok(pairs);
            }
            Err(_) => {
                return Err(Error {
                    message: String::from(
                        "Could not get the response text from the fiat coin market!",
                    ),
                })
            }
        },
        Err(_) => {
            return Err(Error {
                message: String::from("Could not fetch the fiat coin market!"),
            })
        }
    }
}

async fn retrieve_updated_crypto_pairs_by_usd(url: &str) -> Result<Vec<Pair>, Error> {
    match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let mut pairs: Vec<Pair> = vec![];
                let data: Vec<CryptoObject> = serde_json::from_str(&text).unwrap();
                for crypto_object in &data {
                    let code: &String = &crypto_object.symbol.to_uppercase();
                    let value = &crypto_object.current_price;
                    pairs.push(Pair {
                        id: Uuid::new_v4().to_string(),
                        value: value.clone(),
                        base: String::from("USD"),
                        comparison: code.to_string(),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    });
                }
                return Ok(pairs);
            }
            Err(_) => {
                return Err(Error {
                    message: String::from(
                        "Could not get the response text from the crypto coin market!",
                    ),
                })
            }
        },
        Err(_) => {
            return Err(Error {
                message: String::from("Could not fetch the crypto coin market!"),
            })
        }
    }
}

fn find_usd_base_pair(usd_pairs: &Vec<Pair>, comparison: &str) -> Option<Pair> {
    for pair in usd_pairs {
        if pair.comparison == comparison {
            return Some(pair.clone());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_retrieve_pairs_by_base() {
        let mut coin_market = GithubCoinMarket {
            fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
            crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
        };
        let pairs_result = coin_market.retrieve_pairs_by_base("USD").await;
        assert!(pairs_result.is_ok())
    }
}
