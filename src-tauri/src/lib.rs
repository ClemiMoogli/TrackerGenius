mod analysis;
mod commands;
mod loader;
mod parser;
mod types;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let hands = parser::parsing().unwrap_or_else(|e| {
                println!("Erreur: {}", e);
                Vec::new()
            });
            app.manage(hands);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::get_profit_curve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
