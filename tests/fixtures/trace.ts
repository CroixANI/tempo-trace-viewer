import type { TraceView } from '../../src/lib/types/trace';

export const traceViewNoErrors: TraceView = {
  trace_id: 'abc123def456abc123def456abc123de',
  root_service_name: 'frontend-service',
  root_operation_name: '/api/v1/checkout',
  start_time_unix_nano: 1_716_192_000_000_000_000,
  duration_ns: 452_000_000,
  service_count: 3,
  span_count: 12,
  error_count: 0,
  spans: [],
};

export const traceViewWithErrors: TraceView = {
  ...traceViewNoErrors,
  error_count: 4,
};
