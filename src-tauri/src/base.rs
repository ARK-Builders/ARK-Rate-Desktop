use std::collections::HashMap as Map;

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