use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::trace::KeyValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiQueryResponse {
    pub data: LokiData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiData {
    pub result: Vec<LokiStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiStream {
    pub stream: HashMap<String, String>,
    pub values: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryView {
    pub timestamp_ns: u64,
    pub message: String,
    pub labels: Vec<KeyValue>,
    pub span_id: Option<String>,
}
