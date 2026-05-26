<script lang="ts">
  import { traceView } from '../stores/trace';

  function formatDuration(ns: number): string {
    if (ns >= 1_000_000_000) {
      return `${(ns / 1_000_000_000).toFixed(2)}s`;
    }
    return `${(ns / 1_000_000).toFixed(2)}ms`;
  }

  function formatStartTime(ns: number): string {
    return new Date(ns / 1_000_000).toLocaleString();
  }
</script>

{#if $traceView === null}
  <div class="placeholder">Load a trace to begin</div>
{:else}
  <div class="header" data-testid="trace-header">
    <div class="field">
      <span class="label">Service</span>
      <span class="value" data-testid="root-service">{$traceView.root_service_name}</span>
    </div>
    <div class="field">
      <span class="label">Operation</span>
      <span class="value" data-testid="root-operation">{$traceView.root_operation_name}</span>
    </div>
    <div class="field">
      <span class="label">Trace ID</span>
      <span class="value mono" data-testid="trace-id">{$traceView.trace_id}</span>
    </div>
    <div class="field">
      <span class="label">Start</span>
      <span class="value">{formatStartTime($traceView.start_time_unix_nano)}</span>
    </div>
    <div class="field">
      <span class="label">Duration</span>
      <span class="value" data-testid="duration">{formatDuration($traceView.duration_ns)}</span>
    </div>
    <div class="field">
      <span class="label">Services</span>
      <span class="value" data-testid="service-count">{$traceView.service_count}</span>
    </div>
    <div class="field">
      <span class="label">Spans</span>
      <span class="value" data-testid="span-count">{$traceView.span_count}</span>
    </div>
    <div class="field">
      <span class="label">Errors</span>
      <span
        class="value error-count"
        class:error-nonzero={$traceView.error_count > 0}
        data-testid="error-count"
      >{$traceView.error_count}</span>
    </div>
  </div>
{/if}

<style>
  .placeholder {
    padding: 0.75rem 1rem;
    font-family: var(--font-sans);
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    background-color: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
  }

  .header {
    display: flex;
    flex-wrap: wrap;
    gap: 0 2rem;
    padding: 0.75rem 1rem;
    background-color: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    font-family: var(--font-sans);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .label {
    font-size: 0.6875rem;
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .value {
    font-size: 0.875rem;
    color: var(--color-text-primary);
  }

  .value.mono {
    font-family: var(--font-mono);
    font-size: 0.8125rem;
  }

  .error-count {
    color: var(--color-text-secondary);
  }

  .error-count.error-nonzero {
    color: var(--color-error);
    font-weight: 600;
  }
</style>
