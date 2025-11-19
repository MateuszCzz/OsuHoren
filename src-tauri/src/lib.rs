// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod osu_data_loader;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![osu_data_loader::setup_osu_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
