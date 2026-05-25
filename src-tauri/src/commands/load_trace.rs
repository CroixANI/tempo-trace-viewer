use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

use crate::model::TraceView;
use crate::parser::trace_parser::parse_trace_file;
use crate::state::AppState;

#[tauri::command]
pub async fn load_trace(app: AppHandle) -> Result<Option<TraceView>, String> {
    tracing::info!("load_trace invoked");

    let file_path = app
        .dialog()
        .file()
        .add_filter("JSON Trace", &["json"])
        .blocking_pick_file();

    let Some(path) = file_path else {
        tracing::info!("file dialog cancelled");
        return Ok(None);
    };

    let path = path.into_path().map_err(|e| e.to_string())?;
    tracing::info!("parsing trace from {:?}", path);

    let trace_view = parse_trace_file(&path).map_err(|e| e.to_string())?;
    tracing::info!(
        "trace loaded: {} spans, {} services",
        trace_view.span_count,
        trace_view.service_count
    );

    *app.state::<AppState>().trace.lock().unwrap() = Some(trace_view.clone());

    Ok(Some(trace_view))
}
