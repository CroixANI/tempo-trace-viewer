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
    use crate::model::trace::AnyValue;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn one_span(
        trace_id: &str,
        span_id: &str,
        parent_span_id: Option<&str>,
        name: &str,
        service: &str,
        start: u64,
        end: u64,
    ) -> String {
        let parent_field = match parent_span_id {
            Some(p) => format!(r#","parentSpanId": "{p}""#),
            None => String::new(),
        };
        format!(
            r#"{{
                "batches": [{{
                    "resource": {{"attributes": [{{"key": "service.name", "value": {{"stringValue": "{service}"}}}}]}},
                    "instrumentationLibrarySpans": [{{"spans": [
                        {{"traceId": "{trace_id}", "spanId": "{span_id}"{parent_field},
                          "name": "{name}",
                          "startTimeUnixNano": {start}, "endTimeUnixNano": {end}}}
                    ]}}]
                }}]
            }}"#
        )
    }

    // ── integration test against the real example file ────────────────────

    #[test]
    fn test_parse_example_trace() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../docs/examples/traces-and-logs/otel-trace-01003a80-5371-49a3-93fa-cb471d6e2c6d.json",
        );
        let result = parse_trace_file(&path).unwrap();
        assert_eq!(result.span_count, 578);
        assert_eq!(result.error_count, 17);
        assert_eq!(result.service_count, 21);
        assert_eq!(result.root_service_name, "nimbusteamsdriver");
        assert_eq!(result.root_operation_name, "New Conversation");
        assert!(!result.trace_id.is_empty());
    }

    // ── TraceView top-level fields ────────────────────────────────────────

    #[test]
    fn test_parse_empty_batches() {
        let result = parse_trace_str(r#"{"batches": []}"#).unwrap();
        assert_eq!(result.span_count, 0);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.service_count, 0);
        assert_eq!(result.duration_ns, 0);
    }

    #[test]
    fn test_single_span_all_trace_view_fields() {
        let json = one_span("tid01", "sid01", None, "HTTP GET", "api-gateway", 1_000, 5_000);
        let result = parse_trace_str(&json).unwrap();
        assert_eq!(result.trace_id, "tid01");
        assert_eq!(result.root_service_name, "api-gateway");
        assert_eq!(result.root_operation_name, "HTTP GET");
        assert_eq!(result.start_time_unix_nano, 1_000);
        assert_eq!(result.duration_ns, 4_000);
        assert_eq!(result.span_count, 1);
        assert_eq!(result.service_count, 1);
        assert_eq!(result.error_count, 0);
    }

    // ── root span detection ───────────────────────────────────────────────

    #[test]
    fn test_root_when_parent_id_absent() {
        let json = one_span("t", "s1", None, "op", "svc", 0, 1000);
        let result = parse_trace_str(&json).unwrap();
        assert_eq!(result.spans[0].depth, 0);
        assert!(result.spans[0].parent_span_id.is_none());
    }

    #[test]
    fn test_root_when_parent_id_is_empty_string() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s1", "parentSpanId": "",
                     "name": "op", "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.spans[0].depth, 0);
    }

    #[test]
    fn test_root_when_parent_not_in_trace() {
        // parentSpanId points to a span that does not exist in this trace.
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s1", "parentSpanId": "ghost99",
                     "name": "op", "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.spans[0].depth, 0);
        assert!(result.spans[0].parent_span_id.is_none());
    }

    // ── depth and DFS ordering ────────────────────────────────────────────

    #[test]
    fn test_depth_three_levels() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "root", "name": "root",
                     "startTimeUnixNano": 1000, "endTimeUnixNano": 5000},
                    {"traceId": "t", "spanId": "child", "parentSpanId": "root",
                     "name": "child", "startTimeUnixNano": 1100, "endTimeUnixNano": 4900},
                    {"traceId": "t", "spanId": "grand", "parentSpanId": "child",
                     "name": "grand", "startTimeUnixNano": 1200, "endTimeUnixNano": 4800}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        let s = |id: &str| result.spans.iter().find(|s| s.span_id == id).unwrap();
        assert_eq!(s("root").depth, 0);
        assert_eq!(s("child").depth, 1);
        assert_eq!(s("grand").depth, 2);
    }

    #[test]
    fn test_dfs_output_order() {
        // Tree: root → child_a → grandchild; root → child_b
        // child_a starts before child_b, so DFS order: root, child_a, grandchild, child_b
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "root",       "name": "root",
                     "startTimeUnixNano": 1000, "endTimeUnixNano": 5000},
                    {"traceId": "t", "spanId": "child_a",    "parentSpanId": "root",
                     "name": "child_a", "startTimeUnixNano": 1100, "endTimeUnixNano": 2500},
                    {"traceId": "t", "spanId": "grandchild", "parentSpanId": "child_a",
                     "name": "grandchild", "startTimeUnixNano": 1200, "endTimeUnixNano": 2400},
                    {"traceId": "t", "spanId": "child_b",    "parentSpanId": "root",
                     "name": "child_b", "startTimeUnixNano": 2600, "endTimeUnixNano": 4900}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        let ids: Vec<&str> = result.spans.iter().map(|s| s.span_id.as_str()).collect();
        assert_eq!(ids, ["root", "child_a", "grandchild", "child_b"]);
    }

    #[test]
    fn test_multiple_roots_sorted_by_start_time() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "late_root",  "name": "late",
                     "startTimeUnixNano": 2000, "endTimeUnixNano": 3000},
                    {"traceId": "t", "spanId": "early_root", "name": "early",
                     "startTimeUnixNano": 1000, "endTimeUnixNano": 1500}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.spans[0].span_id, "early_root");
        assert_eq!(result.spans[1].span_id, "late_root");
        // root_service derives from the first DFS span
        assert_eq!(result.root_operation_name, "early");
    }

    // ── duration and timeline math ────────────────────────────────────────

    #[test]
    fn test_trace_duration_uses_global_min_max() {
        // child starts before root and ends after — trace duration must span all.
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "root",  "name": "root",
                     "startTimeUnixNano": 2000, "endTimeUnixNano": 3000},
                    {"traceId": "t", "spanId": "child", "parentSpanId": "root",
                     "name": "child", "startTimeUnixNano": 1000, "endTimeUnixNano": 4000}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.start_time_unix_nano, 1_000);
        assert_eq!(result.duration_ns, 3_000); // 4000 - 1000
    }

    #[test]
    fn test_relative_start_and_gantt_percentages() {
        // root: [1000, 5000], child: [2000, 3000]
        // trace_duration = 4000
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "root",  "name": "root",
                     "startTimeUnixNano": 1000, "endTimeUnixNano": 5000},
                    {"traceId": "t", "spanId": "child", "parentSpanId": "root",
                     "name": "child", "startTimeUnixNano": 2000, "endTimeUnixNano": 3000}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        let root  = result.spans.iter().find(|s| s.span_id == "root").unwrap();
        let child = result.spans.iter().find(|s| s.span_id == "child").unwrap();

        assert_eq!(root.relative_start_ns, 0);
        assert_eq!(root.duration_ns, 4_000);
        assert!((root.relative_start_pct - 0.0).abs() < f64::EPSILON);
        assert!((root.duration_pct - 1.0).abs() < f64::EPSILON);

        assert_eq!(child.relative_start_ns, 1_000);
        assert_eq!(child.duration_ns, 1_000);
        assert!((child.relative_start_pct - 0.25).abs() < f64::EPSILON);
        assert!((child.duration_pct - 0.25).abs() < f64::EPSILON);
    }

    // ── service name ──────────────────────────────────────────────────────

    #[test]
    fn test_service_name_from_resource_attribute() {
        let json = r#"{
            "batches": [{"resource": {"attributes": [
                {"key": "service.name", "value": {"stringValue": "payment-service"}}
            ]},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s1", "name": "charge",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.spans[0].service_name, "payment-service");
        assert_eq!(result.root_service_name, "payment-service");
    }

    #[test]
    fn test_missing_service_name_defaults_to_unknown() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s1", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.spans[0].service_name, "unknown");
    }

    // ── error counting ────────────────────────────────────────────────────

    #[test]
    fn test_error_count_from_status_code() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "ok01", "name": "ok",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100,
                     "status": {"code": "STATUS_CODE_OK"}},
                    {"traceId": "t", "spanId": "err1", "name": "fail",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100,
                     "status": {"code": "STATUS_CODE_ERROR"}},
                    {"traceId": "t", "spanId": "err2", "name": "fail2",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100,
                     "status": {"code": "STATUS_CODE_ERROR"}}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.error_count, 2);
        assert!(!result.spans.iter().find(|s| s.span_id == "ok01").unwrap().has_error);
        assert!(result.spans.iter().find(|s| s.span_id == "err1").unwrap().has_error);
    }

    // ── span format variants ──────────────────────────────────────────────

    #[test]
    fn test_scope_spans_key_accepted() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "scopeSpans": [{"spans": [
                    {"traceId": "t", "spanId": "s1", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.span_count, 1);
    }

    #[test]
    fn test_both_span_keys_merged_in_same_batch() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "ils", "name": "ils",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}],
                "scopeSpans": [{"spans": [
                    {"traceId": "t", "spanId": "ss", "name": "ss",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.span_count, 2);
    }

    // ── service count ─────────────────────────────────────────────────────

    #[test]
    fn test_service_count_is_distinct_services() {
        let json = r#"{
            "batches": [
                {"resource": {"attributes": [{"key": "service.name", "value": {"stringValue": "svc-a"}}]},
                 "instrumentationLibrarySpans": [{"spans": [
                     {"traceId": "t", "spanId": "a1", "name": "op", "startTimeUnixNano": 0, "endTimeUnixNano": 100},
                     {"traceId": "t", "spanId": "a2", "name": "op", "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                 ]}]},
                {"resource": {"attributes": [{"key": "service.name", "value": {"stringValue": "svc-b"}}]},
                 "instrumentationLibrarySpans": [{"spans": [
                     {"traceId": "t", "spanId": "b1", "name": "op", "startTimeUnixNano": 0, "endTimeUnixNano": 100}
                 ]}]}
            ]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert_eq!(result.service_count, 2);
        assert_eq!(result.span_count, 3);
    }

    // ── attribute value types ─────────────────────────────────────────────

    #[test]
    fn test_attribute_string_value() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 1,
                     "attributes": [{"key": "http.method", "value": {"stringValue": "GET"}}]}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        let attr = &result.spans[0].attributes[0];
        assert_eq!(attr.key, "http.method");
        assert!(matches!(&attr.value, AnyValue::StringValue { string_value } if string_value == "GET"));
    }

    #[test]
    fn test_attribute_bool_value() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 1,
                     "attributes": [{"key": "cache.hit", "value": {"boolValue": true}}]}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert!(matches!(&result.spans[0].attributes[0].value, AnyValue::BoolValue { bool_value } if *bool_value));
    }

    #[test]
    fn test_attribute_int_value_as_number() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 1,
                     "attributes": [{"key": "http.status_code", "value": {"intValue": 200}}]}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert!(matches!(&result.spans[0].attributes[0].value, AnyValue::IntValue { int_value } if *int_value == 200));
    }

    #[test]
    fn test_attribute_int_value_as_string_encoded() {
        // Tempo exports intValue as a quoted string: {"intValue": "443"}
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 1,
                     "attributes": [{"key": "net.peer.port", "value": {"intValue": "443"}}]}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert!(matches!(&result.spans[0].attributes[0].value, AnyValue::IntValue { int_value } if *int_value == 443));
    }

    #[test]
    fn test_attribute_double_value() {
        let json = r#"{
            "batches": [{"resource": {"attributes": []},
                "instrumentationLibrarySpans": [{"spans": [
                    {"traceId": "t", "spanId": "s", "name": "op",
                     "startTimeUnixNano": 0, "endTimeUnixNano": 1,
                     "attributes": [{"key": "score", "value": {"doubleValue": 0.95}}]}
                ]}]
            }]
        }"#;
        let result = parse_trace_str(json).unwrap();
        assert!(matches!(&result.spans[0].attributes[0].value, AnyValue::DoubleValue { double_value } if (*double_value - 0.95).abs() < f64::EPSILON));
    }
}
