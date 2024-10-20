use directories::ProjectDirs;
use implementations::{
    data_access::{self, file_system::file_system_data_access::FileSystemDataAccess},
    utilities::coin_market::github_coin_market::GithubCoinMarket,
};
use interactors::{
    interactor::Interactor,
    save_pair_group::{SavePairGroup, SavePairGroupRequest},
    view_pair_groups::ViewPairGroups,
};

mod entities;
mod implementations;
mod interactors;
mod utilities;

#[derive(Clone, Debug)]
pub struct Error {
    pub message: String,
}

#[tauri::command]
async fn view_pair_groups() -> String {
    let coin_market = GithubCoinMarket {
        fiat_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/fiat-rates.json"),
        crypto_rates_url: String::from("https://raw.githubusercontent.com/ARK-Builders/ark-exchange-rates/main/crypto-rates.json")
    };
    let data_access = create_fs_data_access();
    let mut interactor = ViewPairGroups {
        coin_market,
        data_access,
    };
    let result = interactor.perform(()).await.unwrap();
    let result_json = serde_json::to_string(&result).unwrap();
    return result_json;
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
async fn save_pair_group(request: String) -> String {
    let data_access = create_fs_data_access();
    let mut interactor = SavePairGroup { data_access };
    let parsed_request = serde_json::from_str::<SavePairGroupRequest>(&request).unwrap();
    let result = interactor.perform(parsed_request).await.unwrap();
    let result_json = serde_json::to_string(&result).unwrap();
    return result_json;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![view_pair_groups, save_pair_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
