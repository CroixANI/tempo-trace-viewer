use serde::{Deserialize, Serialize};

use crate::model::log::LogEntryView;
use crate::model::trace::{KeyValue, SpanEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceView {
    pub trace_id: String,
    pub root_service_name: String,
    pub root_operation_name: String,
    pub start_time_unix_nano: u64,
    pub duration_ns: u64,
    pub service_count: usize,
    pub span_count: usize,
    pub error_count: usize,
    pub spans: Vec<SpanView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanView {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub service_name: String,
    pub operation_name: String,
    pub start_time_unix_nano: u64,
    pub duration_ns: u64,
    pub relative_start_ns: u64,
    pub relative_start_pct: f64,
    pub duration_pct: f64,
    pub depth: u32,
    pub has_error: bool,
    pub attributes: Vec<KeyValue>,
    pub resource_attributes: Vec<KeyValue>,
    pub events: Vec<SpanEvent>,
    pub logs: Vec<LogEntryView>,
}
