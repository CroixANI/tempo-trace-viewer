use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use serde::Deserialize;

use crate::model::trace::{AnyValue, KeyValue, SpanEvent};
use crate::model::view::{SpanView, TraceView};

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum TraceParseError {
    #[error("file not found: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing required field: {0}")]
    MissingField(&'static str),
}

// ── Raw deserialization types matching the actual Tempo JSON format ──────────
// These are private — the public API deals only with TraceView.

#[derive(Deserialize)]
struct TempoFile {
    batches: Vec<TempoBatch>,
}

#[derive(Deserialize)]
struct TempoBatch {
    resource: TempoResource,
    // Tempo uses the older "instrumentationLibrarySpans" key;
    // newer exporters use "scopeSpans".
    #[serde(rename = "instrumentationLibrarySpans", default)]
    instrumentation_library_spans: Vec<TempoScopeSpans>,
    #[serde(rename = "scopeSpans", default)]
    scope_spans: Vec<TempoScopeSpans>,
}

#[derive(Deserialize)]
struct TempoResource {
    #[serde(default)]
    attributes: Vec<RawKeyValue>,
}

#[derive(Deserialize)]
struct TempoScopeSpans {
    spans: Vec<RawSpan>,
}

#[derive(Deserialize)]
struct RawSpan {
    #[serde(rename = "traceId")]
    trace_id: String,
    #[serde(rename = "spanId")]
    span_id: String,
    #[serde(rename = "parentSpanId", default)]
    parent_span_id: Option<String>,
    name: String,
    #[serde(rename = "startTimeUnixNano")]
    start_time_unix_nano: u64,
    #[serde(rename = "endTimeUnixNano")]
    end_time_unix_nano: u64,
    #[serde(default)]
    attributes: Vec<RawKeyValue>,
    #[serde(default)]
    status: RawSpanStatus,
    #[serde(default)]
    events: Vec<RawSpanEvent>,
}

#[derive(Deserialize)]
struct RawKeyValue {
    key: String,
    value: RawAnyValue,
}

// intValue is string-encoded in Tempo exports (e.g. {"intValue": "443"}).
// Using serde_json::Value for that variant handles both string and integer forms.
#[derive(Deserialize)]
#[serde(untagged)]
enum RawAnyValue {
    StringValue {
        #[serde(rename = "stringValue")]
        string_value: String,
    },
    BoolValue {
        #[serde(rename = "boolValue")]
        bool_value: bool,
    },
    IntValue {
        #[serde(rename = "intValue")]
        int_value: serde_json::Value,
    },
    DoubleValue {
        #[serde(rename = "doubleValue")]
        double_value: f64,
    },
    ArrayValue {
        #[serde(rename = "arrayValue")]
        array_value: serde_json::Value,
    },
    KvlistValue {
        #[serde(rename = "kvlistValue")]
        kvlist_value: serde_json::Value,
    },
    // Catch-all for unknown shapes (e.g. empty {}) — must be last in untagged enum.
    #[allow(dead_code)]
    Unknown(serde_json::Value),
}

#[derive(Deserialize, Default)]
struct RawSpanStatus {
    #[serde(default)]
    code: String,
}

#[derive(Deserialize)]
struct RawSpanEvent {
    #[serde(rename = "timeUnixNano")]
    time_unix_nano: String,
    name: String,
    #[serde(default)]
    attributes: Vec<RawKeyValue>,
}

// ── Public entry points ──────────────────────────────────────────────────────

pub fn parse_trace_file(path: &Path) -> Result<TraceView, TraceParseError> {
    let content = std::fs::read_to_string(path)?;
    parse_trace_str(&content)
}

pub fn parse_trace_str(content: &str) -> Result<TraceView, TraceParseError> {
    let file: TempoFile = serde_json::from_str(content)?;
    build_trace_view(file)
}

// ── Core transformation ──────────────────────────────────────────────────────

fn build_trace_view(file: TempoFile) -> Result<TraceView, TraceParseError> {
    // Flatten batches into (resource_batch_idx, raw_span) pairs, keeping resource
    // attributes separate so they can be referenced by index without cloning per span.
    let mut resources: Vec<Vec<RawKeyValue>> = Vec::new();
    let mut flat: Vec<(usize, RawSpan)> = Vec::new();

    for batch in file.batches {
        let resource_idx = resources.len();
        resources.push(batch.resource.attributes);
        for scope in batch
            .instrumentation_library_spans
            .into_iter()
            .chain(batch.scope_spans)
        {
            for span in scope.spans {
                flat.push((resource_idx, span));
            }
        }
    }

    if flat.is_empty() {
        return Ok(TraceView {
            trace_id: String::new(),
            root_service_name: String::new(),
            root_operation_name: String::new(),
            start_time_unix_nano: 0,
            duration_ns: 0,
            service_count: 0,
            span_count: 0,
            error_count: 0,
            spans: Vec::new(),
        });
    }

    let trace_id = flat[0].1.trace_id.clone();

    let trace_start = flat
        .iter()
        .map(|(_, s)| s.start_time_unix_nano)
        .min()
        .unwrap();
    let trace_end = flat
        .iter()
        .map(|(_, s)| s.end_time_unix_nano)
        .max()
        .unwrap();
    let trace_duration = trace_end.saturating_sub(trace_start);

    // span_id_set for root detection: parent not found in file → treat as root.
    let span_id_set: HashSet<String> = flat.iter().map(|(_, s)| s.span_id.clone()).collect();

    // children_map: parent_span_id → Vec<child_span_id>
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    for (_, span) in &flat {
        let effective_parent = span
            .parent_span_id
            .as_deref()
            .filter(|p| !p.is_empty() && span_id_set.contains(*p));
        if let Some(pid) = effective_parent {
            children_map
                .entry(pid.to_owned())
                .or_default()
                .push(span.span_id.clone());
        }
    }

    // BFS depth assignment starting from root spans.
    let mut depth_map: HashMap<String, u32> = HashMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    for (_, span) in &flat {
        let is_root = span
            .parent_span_id
            .as_deref()
            .map(|p| p.is_empty() || !span_id_set.contains(p))
            .unwrap_or(true);
        if is_root {
            depth_map.insert(span.span_id.clone(), 0);
            queue.push_back(span.span_id.clone());
        }
    }
    while let Some(id) = queue.pop_front() {
        let depth = depth_map[&id];
        if let Some(children) = children_map.get(&id) {
            for child_id in children {
                depth_map.entry(child_id.clone()).or_insert(depth + 1);
                queue.push_back(child_id.clone());
            }
        }
    }

    // Build SpanView for each raw span.
    let mut span_view_map: HashMap<String, SpanView> = HashMap::with_capacity(flat.len());
    for (resource_idx, raw) in flat {
        let resource_attrs = &resources[resource_idx];
        let service_name = extract_service_name(resource_attrs);
        let has_error = raw.status.code == "STATUS_CODE_ERROR";
        let relative_start_ns = raw.start_time_unix_nano.saturating_sub(trace_start);
        let duration_ns = raw
            .end_time_unix_nano
            .saturating_sub(raw.start_time_unix_nano);
        let depth = depth_map.get(&raw.span_id).copied().unwrap_or(0);
        let (relative_start_pct, duration_pct) = if trace_duration > 0 {
            (
                relative_start_ns as f64 / trace_duration as f64,
                duration_ns as f64 / trace_duration as f64,
            )
        } else {
            (0.0, 1.0)
        };

        let events: Vec<SpanEvent> = raw
            .events
            .into_iter()
            .map(|e| SpanEvent {
                time_unix_nano: e.time_unix_nano,
                name: e.name,
                attributes: convert_attributes(e.attributes),
            })
            .collect();

        let parent_span_id = raw
            .parent_span_id
            .filter(|p| !p.is_empty() && span_id_set.contains(p.as_str()));

        let view = SpanView {
            span_id: raw.span_id.clone(),
            parent_span_id,
            service_name,
            operation_name: raw.name,
            start_time_unix_nano: raw.start_time_unix_nano,
            duration_ns,
            relative_start_ns,
            relative_start_pct,
            duration_pct,
            depth,
            has_error,
            attributes: convert_attributes(raw.attributes),
            resource_attributes: convert_attributes_from_ref(resource_attrs),
            events,
            logs: Vec::new(),
        };
        span_view_map.insert(raw.span_id, view);
    }

    // DFS collect — roots sorted by start time, children sorted by start time.
    let mut root_ids: Vec<String> = depth_map
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(id, _)| id.clone())
        .collect();
    root_ids.sort_by_key(|id| span_view_map[id].start_time_unix_nano);

    let mut spans: Vec<SpanView> = Vec::with_capacity(span_view_map.len());
    for root_id in &root_ids {
        dfs_collect(root_id, &children_map, &span_view_map, &mut spans);
    }

    let root = spans.first().ok_or(TraceParseError::MissingField("root span"))?;
    let root_service_name = root.service_name.clone();
    let root_operation_name = root.operation_name.clone();

    let service_count = spans
        .iter()
        .map(|s| s.service_name.as_str())
        .collect::<HashSet<_>>()
        .len();
    let error_count = spans.iter().filter(|s| s.has_error).count();
    let span_count = spans.len();

    Ok(TraceView {
        trace_id,
        root_service_name,
        root_operation_name,
        start_time_unix_nano: trace_start,
        duration_ns: trace_duration,
        service_count,
        span_count,
        error_count,
        spans,
    })
}

fn dfs_collect(
    span_id: &str,
    children_map: &HashMap<String, Vec<String>>,
    span_view_map: &HashMap<String, SpanView>,
    result: &mut Vec<SpanView>,
) {
    if let Some(span) = span_view_map.get(span_id) {
        result.push(span.clone());
        if let Some(children) = children_map.get(span_id) {
            let mut sorted = children.clone();
            sorted.sort_by_key(|id| span_view_map[id].start_time_unix_nano);
            for child_id in sorted {
                dfs_collect(&child_id, children_map, span_view_map, result);
            }
        }
    }
}

// ── Attribute conversion helpers ─────────────────────────────────────────────

fn extract_service_name(attributes: &[RawKeyValue]) -> String {
    attributes
        .iter()
        .find(|kv| kv.key == "service.name")
        .and_then(|kv| {
            if let RawAnyValue::StringValue { string_value } = &kv.value {
                Some(string_value.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_owned())
}

fn convert_attributes(raw: Vec<RawKeyValue>) -> Vec<KeyValue> {
    raw.into_iter()
        .map(|kv| KeyValue {
            key: kv.key,
            value: convert_value(kv.value),
        })
        .collect()
}

fn convert_attributes_from_ref(raw: &[RawKeyValue]) -> Vec<KeyValue> {
    raw.iter()
        .map(|kv| KeyValue {
            key: kv.key.clone(),
            value: convert_value_from_ref(&kv.value),
        })
        .collect()
}

fn convert_value(raw: RawAnyValue) -> AnyValue {
    match raw {
        RawAnyValue::StringValue { string_value } => AnyValue::StringValue { string_value },
        RawAnyValue::BoolValue { bool_value } => AnyValue::BoolValue { bool_value },
        RawAnyValue::IntValue { int_value } => AnyValue::IntValue {
            int_value: parse_int_value(&int_value),
        },
        RawAnyValue::DoubleValue { double_value } => AnyValue::DoubleValue { double_value },
        // ArrayValue and KvlistValue are preserved as opaque JSON; frontend treats them as strings.
        RawAnyValue::ArrayValue { array_value } => AnyValue::StringValue {
            string_value: array_value.to_string(),
        },
        RawAnyValue::KvlistValue { kvlist_value } => AnyValue::StringValue {
            string_value: kvlist_value.to_string(),
        },
        RawAnyValue::Unknown(_) => AnyValue::StringValue {
            string_value: String::new(),
        },
    }
}

fn convert_value_from_ref(raw: &RawAnyValue) -> AnyValue {
    match raw {
        RawAnyValue::StringValue { string_value } => AnyValue::StringValue {
            string_value: string_value.clone(),
        },
        RawAnyValue::BoolValue { bool_value } => AnyValue::BoolValue {
            bool_value: *bool_value,
        },
        RawAnyValue::IntValue { int_value } => AnyValue::IntValue {
            int_value: parse_int_value(int_value),
        },
        RawAnyValue::DoubleValue { double_value } => AnyValue::DoubleValue {
            double_value: *double_value,
        },
        RawAnyValue::ArrayValue { array_value } => AnyValue::StringValue {
            string_value: array_value.to_string(),
        },
        RawAnyValue::KvlistValue { kvlist_value } => AnyValue::StringValue {
            string_value: kvlist_value.to_string(),
        },
        RawAnyValue::Unknown(_) => AnyValue::StringValue {
            string_value: String::new(),
        },
    }
}

fn parse_int_value(v: &serde_json::Value) -> i64 {
    match v {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or(0),
        serde_json::Value::String(s) => s.parse().unwrap_or(0),
        _ => 0,
    }
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_trace() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../docs/examples/traces-and-logs/otel-trace-01003a80-5371-49a3-93fa-cb471d6e2c6d.json",
        );
        let result = parse_trace_file(&path).unwrap();
        assert_eq!(result.span_count, 578);
        assert_eq!(result.error_count, 17);
        assert!(!result.trace_id.is_empty());
    }

    #[test]
    fn test_parse_empty_trace() {
        let result = parse_trace_str(r#"{"batches": []}"#).unwrap();
        assert_eq!(result.span_count, 0);
    }

    #[test]
    fn test_depth_calculation() {
        let json = r#"{
            "batches": [{
                "resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "abc", "spanId": "parent01", "name": "parent",
                     "startTimeUnixNano": 1000, "endTimeUnixNano": 2000},
                    {"traceId": "abc", "spanId": "child001", "parentSpanId": "parent01",
                     "name": "child", "startTimeUnixNano": 1100, "endTimeUnixNano": 1900}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        let parent = result.spans.iter().find(|s| s.span_id == "parent01").unwrap();
        let child = result.spans.iter().find(|s| s.span_id == "child001").unwrap();
        assert_eq!(parent.depth, 0);
        assert_eq!(child.depth, 1);
    }
}
