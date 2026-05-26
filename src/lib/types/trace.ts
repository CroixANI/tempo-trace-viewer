export interface KeyValue {
  key: string;
  value: AnyValue;
}

export type AnyValue =
  | { string_value: string }
  | { bool_value: boolean }
  | { int_value: number }
  | { double_value: number }
  | { array_value: AnyValue[] }
  | { kvlist_value: KeyValue[] };

export interface SpanEvent {
  time_unix_nano: number;
  name: string;
  attributes: KeyValue[];
}

export interface LogEntryView {
  timestamp_ns: number;
  message: string;
  labels: KeyValue[];
  span_id: string | null;
}

export interface SpanView {
  span_id: string;
  parent_span_id: string | null;
  service_name: string;
  operation_name: string;
  start_time_unix_nano: number;
  duration_ns: number;
  relative_start_ns: number;
  relative_start_pct: number;
  duration_pct: number;
  depth: number;
  has_error: boolean;
  attributes: KeyValue[];
  resource_attributes: KeyValue[];
  events: SpanEvent[];
  logs: LogEntryView[];
}

export interface TraceView {
  trace_id: string;
  root_service_name: string;
  root_operation_name: string;
  start_time_unix_nano: number;
  duration_ns: number;
  service_count: number;
  span_count: number;
  error_count: number;
  spans: SpanView[];
}
