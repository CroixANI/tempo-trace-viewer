<script lang="ts">
  import { traceView, selectedSpanId } from '../stores/trace';
  import TagsSection from './TagsSection.svelte';
  import ResourceSection from './ResourceSection.svelte';
  import LogsSection from './LogsSection.svelte';
  import type { SpanView } from '../types/trace';

  $: selectedSpan = $selectedSpanId && $traceView
    ? ($traceView.spans.find(s => s.span_id === $selectedSpanId) ?? null)
    : null;

  function formatDuration(ns: number): string {
    if (ns >= 1_000_000_000) return (ns / 1_000_000_000).toFixed(2) + 's';
    if (ns >= 1_000_000) return (ns / 1_000_000).toFixed(2) + 'ms';
    return (ns / 1_000).toFixed(2) + 'µs';
  }
</script>

{#if selectedSpan}
  <div class="span-detail">
    <div class="detail-header">
      <div class="detail-service" title={selectedSpan.service_name}>{selectedSpan.service_name}</div>
      <div class="detail-operation">{selectedSpan.operation_name}</div>
      <div class="detail-meta">
        <span class="meta-item">
          <span class="meta-label">Span ID</span>
          <span class="meta-value mono">{selectedSpan.span_id}</span>
        </span>
        <span class="meta-item">
          <span class="meta-label">Duration</span>
          <span class="meta-value mono">{formatDuration(selectedSpan.duration_ns)}</span>
        </span>
        {#if selectedSpan.has_error}
          <span class="error-badge">error</span>
        {/if}
      </div>
    </div>

    <div class="sections">
      <TagsSection attributes={selectedSpan.attributes} />
      <ResourceSection attributes={selectedSpan.resource_attributes} />
      <LogsSection logs={selectedSpan.logs} />
    </div>
  </div>
{/if}

<style>
  .span-detail {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background-color: var(--color-bg);
    border-left: 1px solid var(--color-border);
    font-family: var(--font-sans);
  }

  .detail-header {
    padding: 12px;
    border-bottom: 2px solid var(--color-border);
    background-color: var(--color-surface);
    flex-shrink: 0;
  }

  .detail-service {
    font-size: 11px;
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-operation {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 4px 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 6px;
    align-items: center;
  }

  .meta-item {
    display: flex;
    gap: 4px;
    align-items: baseline;
  }

  .meta-label {
    font-size: 10px;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .meta-value {
    font-size: 11px;
    color: var(--color-text-primary);
  }

  .mono {
    font-family: var(--font-mono);
  }

  .error-badge {
    font-size: 10px;
    font-weight: 600;
    color: #fff;
    background-color: var(--color-error);
    padding: 1px 6px;
    border-radius: 3px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .sections {
    flex: 1;
    overflow-y: auto;
  }
</style>
