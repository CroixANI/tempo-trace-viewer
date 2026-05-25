use tauri::{AppHandle, Manager};
use tracing_appender::rolling::Rotation;

pub fn init_logging(app: &AppHandle) {
    let log_dir = app.path().app_log_dir().expect("log dir unavailable");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(Rotation::NEVER)
        .max_log_files(1)
        .filename_prefix("tempo-trace-viewer")
        .filename_suffix("log")
        .build(log_dir)
        .expect("failed to create log appender");

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");
}
