pub mod commands;
pub mod logging;
pub mod model;
pub mod parser;
pub mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .setup(|app| {
            logging::init_logging(&app.handle());
            Ok(())
        });

    #[cfg(not(debug_assertions))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        commands::load_trace::load_trace,
        commands::clear_session::clear_session,
    ]);

    #[cfg(debug_assertions)]
    let builder = builder.invoke_handler(tauri::generate_handler![
        commands::load_trace::load_trace,
        commands::clear_session::clear_session,
        commands::load_trace_from_path::load_trace_from_path,
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
