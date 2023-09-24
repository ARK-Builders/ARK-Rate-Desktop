use directories::ProjectDirs;
use reqwest;
use std::io::{Read, Write};
use std::{collections::HashMap as Map, fs::create_dir_all, fs::File};

use crate::base::{CryptoRates, FiatRates, RateConstructErrors};

const FIAT_RATES_FETCH_URL: &str =
    "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json";
const CRYPTO_RATES_FETCH_URL: &str =
    "https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json";

async fn fetch_rates() -> Result<(String, String), reqwest::Error> {
    let fiat_rates = reqwest::get(FIAT_RATES_FETCH_URL).await?.text().await?;
    let crypto_rates = reqwest::get(CRYPTO_RATES_FETCH_URL).await?.text().await?;
    Ok((fiat_rates, crypto_rates))
}

fn parse_fiat_rates(unparsed_rates: String) -> Result<Map<String, f32>, serde_json::Error> {
    let fiat_rates = serde_json::from_str::<FiatRates>(&unparsed_rates)?;
    Ok(fiat_rates.rates)
}

fn parse_crypto_rates(unparsed_rates: String) -> Result<Map<String, f32>, serde_json::Error> {
    let crypto_rates = serde_json::from_str::<Vec<CryptoRates>>(&unparsed_rates)?;
    let mut rates: Map<String, f32> = Map::new();
    for crypto in crypto_rates {
        rates.insert(crypto.symbol.to_uppercase(), 1.0 / crypto.current_price);
    }
    Ok(rates)
}

pub async fn construct_rates() -> Result<Map<String, f32>, RateConstructErrors> {
    let (unparsed_fiat_rates, unparsed_crypto_rates) = fetch_rates().await?;

    let fiat_rates = parse_fiat_rates(unparsed_fiat_rates)?;
    let crypto_rates = parse_crypto_rates(unparsed_crypto_rates)?;

    let mut rates: Map<String, f32> = Map::new();
    rates.extend(fiat_rates.into_iter());
    rates.extend(crypto_rates.into_iter());

    Ok(rates)
}

fn create_or_use_rates_file(file_name: &str) -> File {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ark-builders", "ark-rate-desktop") {
        let path = proj_dirs.cache_dir();
        // must panic if failed to create/use the rates file
        let _ = create_dir_all(&path)
            .expect("Failed to create the missing directories for storing the rates");
        let path = path.join(file_name);
        File::options()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap_or_else(|_| File::create(&path).unwrap())
    } else {
        panic!("No valid home directory path could be retrieved from the operating system.");
    }
}

fn write_rates_to_file(rates: &Map<String, f32>, mut file: File) {
    let bytes = serde_json::to_string(&rates).expect("Must not fail to serialize the json into a string");
    let _ = file.set_len(0);
    file.write_all(bytes.as_bytes()).unwrap();
}

fn read_rates_from_file(mut file: File) -> Result<Map<String, f32>, RateConstructErrors> {
    let mut unparsed_rates = String::new();
    file.read_to_string(&mut unparsed_rates)?;
    let parsed_rates = serde_json::from_str::<Map<String, f32>>(&unparsed_rates)?;
    Ok(parsed_rates)
}

pub async fn get_parsed_rates() -> Map<String, f32> {
    let parsed_rates: Map<String, f32>;
    let rates_file = create_or_use_rates_file("rates");
    let result = construct_rates().await;

    match result {
        Ok(rates) => {
            parsed_rates = rates;
            write_rates_to_file(&parsed_rates, rates_file);
        }
        Err(err) => {
            dbg!(err, &rates_file);
            parsed_rates = read_rates_from_file(rates_file)
                .expect("Failed to retrieve rates which is required")
        }
    }

    parsed_rates
}