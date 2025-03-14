use directories::ProjectDirs;
use error::Error;
use implementations::{
    data_access::file_system::file_system_data_access::FileSystemDataAccess,
    utilities::coin_market::github_coin_market::GithubCoinMarket,
};
use interactors::{
    delete_asset::{DeleteAsset, DeleteAssetRequest},
    delete_pair_group::{DeletePairGroup, DeletePairGroupRequest},
    delete_tag::{DeleteTag, DeleteTagRequest},
    delete_watchlist_pair::{DeleteWatchlistPair, DeleteWatchlistPairRequest},
    interactor::Interactor,
    save_pair_group::{SavePairGroup, SavePairGroupRequest},
    save_tag::{SaveTag, SaveTagRequest},
    store_portfolios::{StorePortfolios, StorePortfoliosRequest},
    store_watchlist_coins::{StoreWatchlistCoins, StoreWatchlistCoinsRequest},
    update_pair_group::{UpdatePairGroup, UpdatePairGroupRequest},
    update_portfolio::{UpdatePortfolio, UpdatePortfolioRequest},
    view_pair_groups::ViewPairGroups,
    view_portfolios::ViewPortfolios,
    view_watchlist::ViewWatchlist,
};

mod entities;
mod error;
mod implementations;
mod interactors;
mod utilities;

/*
    TODO (NOT SURE):
        - Try to reuse the 'coin_market' instance for all commands
        - Try to reuse the 'data_access' instance for all commands
*/

#[tauri::command]
async fn view_pair_groups() -> Result<String, String> {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = ViewPairGroups {
        coin_market,
        data_access,
    };
    let result = interactor.perform(()).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

fn create_fs_data_access() -> FileSystemDataAccess {
    let dirs = get_project_dirs().unwrap();
    let data_access = FileSystemDataAccess {
        root: dirs.cache_dir().to_path_buf(),
    };
    return data_access;
}

fn get_project_dirs() -> Result<ProjectDirs, Error> {
    let maybe_dirs = ProjectDirs::from("com", "ark-builders", "ark-rate-desktop");
    if maybe_dirs.is_none() {
        return Err(Error {
            message: String::from("Project directory was not found!"),
        });
    }
    return Ok(maybe_dirs.unwrap());
}

#[tauri::command]
async fn save_pair_group(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = SavePairGroup { data_access };
    let parsed_request = serde_json::from_str::<SavePairGroupRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn update_pair_group(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = UpdatePairGroup { data_access };
    let parsed_request = serde_json::from_str::<UpdatePairGroupRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn delete_pair_group(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = DeletePairGroup { data_access };
    let parsed_request = serde_json::from_str::<DeletePairGroupRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn view_portfolios() -> Result<String, String> {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = ViewPortfolios {
        coin_market,
        data_access,
    };
    let result = interactor.perform(()).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn store_portfolios(request: String) -> Result<String, String> {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = StorePortfolios {
        coin_market,
        data_access,
    };
    let parsed_request = serde_json::from_str::<StorePortfoliosRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn save_tag(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = SaveTag { data_access };
    let parsed_request = serde_json::from_str::<SaveTagRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn delete_tag(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = DeleteTag { data_access };
    let parsed_request = serde_json::from_str::<DeleteTagRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn update_portfolio(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = UpdatePortfolio { data_access };
    let parsed_request = serde_json::from_str::<UpdatePortfolioRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn delete_asset(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = DeleteAsset { data_access };
    let parsed_request = serde_json::from_str::<DeleteAssetRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn view_watchlist() -> Result<String, String> {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = ViewWatchlist {
        coin_market,
        data_access,
    };
    let result = interactor.perform(()).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn store_watchlist_coins(request: String) -> Result<String, String> {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = StoreWatchlistCoins {
        coin_market,
        data_access,
    };
    let parsed_request = serde_json::from_str::<StoreWatchlistCoinsRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[tauri::command]
async fn delete_watchlist_pair(request: String) -> Result<String, String> {
    let data_access = create_fs_data_access();
    let mut interactor = DeleteWatchlistPair { data_access };
    let parsed_request = serde_json::from_str::<DeleteWatchlistPairRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await;
    if result.is_err() {
        return Err(serde_json::to_string(&result.unwrap_err()).unwrap());
    }
    return Ok(serde_json::to_string(&result.unwrap()).unwrap());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            view_pair_groups,
            save_pair_group,
            update_pair_group,
            delete_pair_group,
            view_portfolios,
            store_portfolios,
            save_tag,
            delete_tag,
            update_portfolio,
            delete_asset,
            view_watchlist,
            store_watchlist_coins,
            delete_watchlist_pair,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
