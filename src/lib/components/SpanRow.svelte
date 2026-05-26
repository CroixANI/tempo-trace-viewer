<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import GanttBar from './GanttBar.svelte';
  import type { SpanView } from '../types/trace';

  export let span: SpanView;
  export let isSelected: boolean = false;
  export let isCollapsed: boolean = false;
  export let hasChildren: boolean = false;

  const dispatch = createEventDispatcher<{ select: string; toggle: string }>();

  // djb2 hash of service name → hue (0–359)
  function serviceColor(name: string): string {
    let h = 5381;
    for (let i = 0; i < name.length; i++) {
      h = ((h << 5) + h + name.charCodeAt(i)) | 0;
    }
    const hue = Math.abs(h) % 360;
    return `hsl(${hue}, 60%, 50%)`;
  }

  function formatDuration(ns: number): string {
    if (ns >= 1_000_000_000) return (ns / 1_000_000_000).toFixed(2) + 's';
    if (ns >= 1_000_000) return (ns / 1_000_000).toFixed(2) + 'ms';
    return (ns / 1_000).toFixed(2) + 'µs';
  }

  $: color = serviceColor(span.service_name);
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div
  class="span-row"
  class:selected={isSelected}
  class:has-error={span.has_error}
  style="padding-left: calc({span.depth} * var(--span-indent-width) + 4px);"
  on:click={() => dispatch('select', span.span_id)}
  data-testid="span-row"
>
  <button
    class="collapse-btn"
    class:invisible={!hasChildren}
    tabindex="-1"
    on:click|stopPropagation={() => dispatch('toggle', span.span_id)}
    aria-label={isCollapsed ? 'Expand' : 'Collapse'}
  >
    {isCollapsed ? '▶' : '▼'}
  </button>

  <span class="service-badge" style="background-color: {color};" title={span.service_name}>
    {span.service_name}
  </span>

  <span class="span-name">{span.operation_name}</span>

  <span class="duration">{formatDuration(span.duration_ns)}</span>

  <div class="gantt-area">
    <GanttBar startPct={span.relative_start_pct} durationPct={span.duration_pct} {color} />
  </div>
</div>

<style>
  .span-row {
    display: flex;
    align-items: center;
    height: var(--span-row-height-base);
    gap: 6px;
    cursor: pointer;
    font-family: var(--font-sans);
    font-size: 12px;
    color: var(--color-text-primary);
    border-bottom: 1px solid var(--color-border);
    box-sizing: border-box;
    padding-right: 8px;
  }

  .span-row:hover {
    background-color: var(--color-row-hover);
  }

  .span-row.selected {
    background-color: var(--color-selected-bg);
  }

  .collapse-btn {
    flex-shrink: 0;
    width: 16px;
    height: 16px;
    padding: 0;
    border: none;
    background: none;
    cursor: pointer;
    color: var(--color-text-secondary);
    font-size: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .collapse-btn.invisible {
    visibility: hidden;
  }

  .service-badge {
    flex-shrink: 0;
    padding: 1px 5px;
    border-radius: 3px;
    font-size: 10px;
    font-weight: 600;
    color: #fff;
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .span-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--color-text-primary);
  }

  .has-error .span-name {
    color: var(--color-error);
  }

  .duration {
    flex-shrink: 0;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--color-text-secondary);
    min-width: 60px;
    text-align: right;
  }

  .gantt-area {
    flex-shrink: 0;
    width: var(--gantt-area-width);
  }
</style>
