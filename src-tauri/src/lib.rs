// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod osu_db_parser;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![osu_db_parser::load_osu_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
