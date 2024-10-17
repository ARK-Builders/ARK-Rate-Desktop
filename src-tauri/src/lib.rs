use entities::pair_group::PairGroup;
use interactors::view_pair_groups::ViewPairGroupsDataAccess;

mod entities;
mod implementations;
mod interactors;
mod utilities;

#[tauri::command]
fn view_pair_groups() -> String {
    return "".to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
