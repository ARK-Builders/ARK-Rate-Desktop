use std::collections::HashMap as Map;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::io::Error as StdIOError;
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Coin {
    pub conversion_rate: f32,
    pub existing_amount: f32,
    pub is_selected: bool,
}

pub struct Storage {
    pub rates: Map<String, f32>
}

#[derive(serde::Deserialize)]
pub struct FiatRates {
    pub rates: Map<String, f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CryptoRates {
    pub symbol: String,
    pub current_price: f32,
}

#[derive(Debug)]
pub enum RateConstructErrors {
    ParseError(serde_json::Error),
    FetchError(reqwest::Error),
    IOError(std::io::Error),
}

impl From<ReqwestError> for RateConstructErrors {
    fn from(error: ReqwestError) -> Self {
        RateConstructErrors::FetchError(error)
    }
}

impl From<SerdeJsonError> for RateConstructErrors {
    fn from(error: SerdeJsonError) -> Self {
        RateConstructErrors::ParseError(error)
    }
}

impl From<StdIOError> for RateConstructErrors {
    fn from(error: StdIOError) -> Self {
        RateConstructErrors::IOError(error)
    }
}