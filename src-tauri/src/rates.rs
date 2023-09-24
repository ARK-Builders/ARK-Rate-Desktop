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

fn create_or_use_rates_file(file_name: &str, empty: bool) -> File {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ark-builders", "ark-rate-desktop") {
        let path = proj_dirs.cache_dir();
        // must panic if failed to create/use the rates file
        let _ = create_dir_all(&path)
            .expect("Failed to create the missing directories for storing the rates");
        let path = path.join(file_name);
        File::options()
            .read(true)
            .write(true)
            .truncate(empty)
            .open(&path)
            .unwrap_or_else(|_| File::create(&path).unwrap())
    } else {
        panic!("No valid home directory path could be retrieved from the operating system.");
    }
}

fn write_rates_to_file(rates: &Map<String, f32>, mut file: &File) {
    let bytes = serde_json::to_string(rates).expect("Must not fail to serialize the json into a string");
    file.set_len(0).unwrap();
    file.write_all(bytes.as_bytes()).unwrap();
    file.flush().unwrap();
}

fn read_rates_from_file(mut file: &File) -> Result<Map<String, f32>, RateConstructErrors> {
    let mut unparsed_rates = String::new();
    file.read_to_string(&mut unparsed_rates)?;
    let parsed_rates = serde_json::from_str::<Map<String, f32>>(&unparsed_rates)?;
    Ok(parsed_rates)
}

pub async fn get_parsed_rates() -> Map<String, f32> {
    let parsed_rates: Map<String, f32>;
    let result = construct_rates().await;

    match result {
        Ok(rates) => {
            parsed_rates = rates;
            let rates_file = create_or_use_rates_file("rates", true);
            write_rates_to_file(&parsed_rates, &rates_file);
        }
        Err(_) => {
            let rates_file = create_or_use_rates_file("rates", false);
            parsed_rates = read_rates_from_file(&rates_file)
                .expect("Failed to retrieve rates which is required")
        }
    }

    parsed_rates
}

#[cfg(test)]
mod tests {
    use std::io::Seek;

    use super::*;

    fn create_or_use_file(path: &str, empty: bool) -> File {
        File::options()
            .read(true)
            .write(true)
            .truncate(empty)
            .open(path)
            .unwrap_or_else(|_| File::create(path).unwrap())
    }

    fn add_string_to_file(data: &str, file: &mut File) {
        file.set_len(0).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn test_write_rates_to_file() {
        let rates: Map<String, f32> = Map::from([(String::from("USD"), 1.0), (String::from("INR"), 0.01)]);
        let mut file = create_or_use_file("write-rates", true);

        write_rates_to_file(&rates, &file);

        file.rewind().unwrap();
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();
        assert_eq!(file_data.is_empty(), false);
    }

    #[test]
    fn test_read_rates_from_file() {
        let mut file = create_or_use_file("read-rates", true);
        let rates_string = "{ \"USD\": 1.0, \"INR\": 0.01 }";
        let rates = serde_json::from_str::<Map<String, f32>>(rates_string).unwrap();
        add_string_to_file(rates_string, &mut file);
        file.rewind().unwrap();

        let parsed_rates = read_rates_from_file(&file).unwrap();
        assert_eq!(rates, parsed_rates);
    }

    #[test]
    fn test_parse_fiat_rates() {
        let unparsed_rates = "{ \"rates\": {\"USD\": 1.0, \"INR\": 0.01} }";
        let parsed_rates: Map<String, f32> = Map::from([(String::from("USD"), 1.0), (String::from("INR"), 0.01)]);

        let result = parse_fiat_rates(String::from(unparsed_rates)).unwrap();
        assert_eq!(parsed_rates, result);
    }

    #[test]
    fn test_parse_crypto_rates() {
        let unparsed_rates = "[ { \"symbol\": \"USD\", \"current_price\": 1 }, {\"symbol\": \"INR\", \"current_price\": 100} ]";
        let parsed_rates: Map<String, f32> = Map::from([(String::from("USD"), 1.0), (String::from("INR"), 0.01)]);

        let result = parse_crypto_rates(String::from(unparsed_rates)).unwrap();
        assert_eq!(parsed_rates, result);    
    }
}
