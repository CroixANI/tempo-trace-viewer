pub mod commands;
pub mod logging;
pub mod model;
pub mod parser;
pub mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .setup(|app| {
            logging::init_logging(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_trace::load_trace,
            commands::clear_session::clear_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
