use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::model::TraceView;
use crate::parser::trace_parser::parse_trace_file;
use crate::state::AppState;

/// Loads a trace from an absolute path without opening a file dialog.
/// Only compiled in debug builds; used exclusively by the E2E test suite.
#[tauri::command]
pub async fn load_trace_from_path(path: String, app: AppHandle) -> Result<TraceView, String> {
    tracing::debug!("load_trace_from_path: {}", path);
    let trace_view = parse_trace_file(&PathBuf::from(&path)).map_err(|e| e.to_string())?;
    *app.state::<AppState>().trace.lock().unwrap() = Some(trace_view.clone());
    Ok(trace_view)
}
