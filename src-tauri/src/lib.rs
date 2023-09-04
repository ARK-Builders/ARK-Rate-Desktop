use std::collections::HashMap as Map;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Debug, serde::Deserialize)]
struct Currency {
    conversionRate: f32,
    existingAmount: f32,
}

#[tauri::command]
fn calculate_currency_total(selected_currencies: Map<String, Currency>) -> Map<String, f32> {
    let currencies: Vec<(String, Currency)> = selected_currencies.into_iter().collect();
    let mut total: Map<String, f32> = Map::new();

    for (key, currency) in &currencies {
        total.insert(key.clone(), currency.existingAmount);
    }

    for (key1, currency) in &currencies {
        for (key2, other_currency) in &currencies {
            if key1 != key2 {
                let total_value = (total[key1])
                    + (other_currency.existingAmount
                        * (currency.conversionRate / other_currency.conversionRate));
                total.insert(key1.clone(), total_value);
            }
        }
    }

    return total;
}

#[tauri::command]
fn calculate_exchange_rates(selected_currencies: Map<String, Currency>) -> Map<String, f32> {
    let currencies: Vec<(String, Currency)> = selected_currencies.into_iter().collect();
    let mut exchange: Map<String, f32> = Map::new();

    for (key1, currency) in &currencies {
        for (key2, other_currency) in &currencies {
            if key1 != key2 {
                exchange.insert(
                    format!("{key1}/{key2}"),
                    other_currency.conversionRate / currency.conversionRate,
                );
            }
        }
    }

    return exchange;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            calculate_currency_total,
            calculate_exchange_rates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
