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
    async fn fetch_usd_pairs(&mut self) -> Result<Vec<Pair>, Error> {
        let usd_fiat_pairs: Vec<Pair> = fetch_usd_fiat_pairs(&self.fiat_rates_url).await?;
        let usd_crypto_pairs: Vec<Pair> = fetch_usd_crypto_pairs(&self.crypto_rates_url).await?;
        return Ok(usd_fiat_pairs
            .iter()
            .chain(&usd_crypto_pairs)
            .cloned()
            .collect());
    }
}

async fn fetch_usd_fiat_pairs(url: &str) -> Result<Vec<Pair>, Error> {
    match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let mut pairs: Vec<Pair> = vec![];
                let data: FiatResponse = serde_json::from_str(&text).unwrap();
                for rate in &data.rates {
                    let code = rate.0;
                    let value = rate.1;
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

async fn fetch_usd_crypto_pairs(url: &str) -> Result<Vec<Pair>, Error> {
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

#[cfg(test)]
mod tests {
    // TODO: create unit tests
}
