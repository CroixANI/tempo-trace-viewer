use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub async fn clear_session(state: State<'_, AppState>) -> Result<(), String> {
    *state.trace.lock().unwrap() = None;
    state.log_entries.lock().unwrap().clear();
    tracing::info!("session cleared");
    Ok(())
}
