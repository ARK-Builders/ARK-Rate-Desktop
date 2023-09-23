// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod base;
pub mod commands;

use base::{CryptoRates, FiatRates, Storage};
use commands::{calculate_currency_total, calculate_exchange_rates, get_rates};
use directories::ProjectDirs;
use reqwest;
use std::io::{Read, Write};
use std::{collections::HashMap as Map, fs::create_dir_all, fs::File};
use tokio;

const FIAT_RATES_FETCH_URL: &str =
    "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json";
const CRYPTO_RATES_FETCH_URL: &str =
    "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json";

async fn fetch_rates() -> Result<(Map<String, f32>, Vec<CryptoRates>), reqwest::Error> {
    let fiat_rates = reqwest::get(FIAT_RATES_FETCH_URL).await?.text().await?;

    if fiat_rates.is_empty() {
        panic!("No fiat rates found.");
    }

    let fiat_rates: FiatRates = serde_json::from_str(&fiat_rates).unwrap();

    let crypto_rates = reqwest::get(CRYPTO_RATES_FETCH_URL).await?.text().await?;

    if crypto_rates.is_empty() {
        panic!("No crypto rates found.");
    }

    let crypto_rates: Vec<CryptoRates> = serde_json::from_str(&crypto_rates).unwrap();

    Ok((fiat_rates.rates, crypto_rates))
}

#[tokio::main]
async fn main() {
    let mut rates: Map<String, f32>;
    if let Some(proj_dirs) = ProjectDirs::from("com", "ark-builders", "ark-rate-desktop") {
        let path = proj_dirs.cache_dir();
        let _ = create_dir_all(&path)
            .expect("Failed to create the missing directories for storing the rates");
        let path = path.join("rates");
        let mut rates_file = File::options()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap_or_else(|_| File::create(&path).unwrap());

        let result = fetch_rates().await;

        match result {
            Ok((fiat_rates, crypto_rates)) => {
                rates = fiat_rates;
                for crypto in crypto_rates {
                    rates.insert(crypto.symbol.to_uppercase(), 1.0 / crypto.current_price);
                }
                let bytes = serde_json::to_string(&rates).unwrap_or(String::new());
                rates_file.write_all(bytes.as_bytes()).unwrap();
            }
            Err(_) => {
                let mut unparsed_rates = String::new();
                rates_file.read_to_string(&mut unparsed_rates).unwrap_or(0);
                rates = serde_json::from_str(&unparsed_rates).unwrap_or_default();
            }
        }
    } else {
        panic!("No valid home directory path could be retrieved from the operating system.")
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            calculate_currency_total,
            calculate_exchange_rates,
            get_rates
        ])
        .manage(Storage { rates })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
