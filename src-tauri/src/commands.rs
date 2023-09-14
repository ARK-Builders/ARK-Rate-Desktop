use std::collections::HashMap as Map;
use crate::base::{Coin, Storage};

#[tauri::command]
pub fn get_rates(storage: tauri::State<Storage>) -> Map<String, f32> {
    storage.rates.clone()
}

#[tauri::command]
pub async fn calculate_currency_total(selected_currencies: Map<String, Coin>) -> Map<String, f32> {
    let currencies: Vec<(String, Coin)> = selected_currencies.into_iter().collect();
    let mut total: Map<String, f32> = Map::new();

    for (key, currency) in &currencies {
        total.insert(key.clone(), currency.existing_amount);
    }

    for (key1, currency) in &currencies {
        for (key2, other_currency) in &currencies {
            if key1 != key2 {
                let total_value = (total[key1])
                    + (other_currency.existing_amount
                        * (currency.conversion_rate / other_currency.conversion_rate));
                total.insert(key1.clone(), total_value);
            }
        }
    }

    return total;
}

#[tauri::command]
pub fn calculate_exchange_rates(selected_currencies: Map<String, Coin>) -> Map<String, f32> {
    let currencies: Vec<(String, Coin)> = selected_currencies.into_iter().collect();
    let mut exchange: Map<String, f32> = Map::new();

    for (key1, currency) in &currencies {
        for (key2, other_currency) in &currencies {
            if key1 != key2 {
                exchange.insert(
                    format!("{key1}/{key2}"),
                    other_currency.conversion_rate / currency.conversion_rate,
                );
            }
        }
    }

    return exchange;
}