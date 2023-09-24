// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod base;
pub mod commands;
pub mod rates;

use base::Storage;
use commands::{calculate_currency_total, calculate_exchange_rates, get_rates};
use rates::get_parsed_rates;

use std::collections::HashMap as Map;
use tokio;

#[tokio::main]
async fn main() {
    let rates: Map<String, f32> = get_parsed_rates().await;

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
