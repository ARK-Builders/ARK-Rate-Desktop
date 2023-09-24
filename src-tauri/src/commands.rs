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
pub async fn calculate_exchange_rates(selected_currencies: Map<String, Coin>) -> Map<String, f32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exchange_rate_calculation() {
        let rates: Map<String, Coin> = Map::from([(String::from("USD"), Coin {
            conversion_rate: 1.0,
            existing_amount: 2.0,
            is_selected: true
        }), (String::from("INR"), Coin {
            conversion_rate: 0.01,
            existing_amount: 10.0,
            is_selected: true
        })]);

        let expected_result = Map::from([(String::from("INR/USD"), 100.0), (String::from("USD/INR"), 0.01)]);
        let result = calculate_exchange_rates(rates).await;
        assert_eq!(result, expected_result);
    }

    #[tokio::test]
    async fn test_total_calculation() {
        let rates: Map<String, Coin> = Map::from([(String::from("USD"), Coin {
            conversion_rate: 1.0,
            existing_amount: 2.0,
            is_selected: true
        }), (String::from("INR"), Coin {
            conversion_rate: 0.01,
            existing_amount: 10.0,
            is_selected: true
        })]);

        let expected_result = Map::from([(String::from("INR"), 10.02), (String::from("USD"), 1002.0)]);
        let result = calculate_currency_total(rates).await;
        assert_eq!(result, expected_result);
    }
}