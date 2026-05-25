use std::sync::Mutex;

use crate::model::{LogEntryView, TraceView};

pub struct AppState {
    pub trace: Mutex<Option<TraceView>>,
    pub log_entries: Mutex<Vec<LogEntryView>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            trace: Mutex::new(None),
            log_entries: Mutex::new(Vec::new()),
        }
    }
}
