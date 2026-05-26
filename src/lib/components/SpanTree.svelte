<script lang="ts">
  import { createVirtualizer } from '@tanstack/svelte-virtual';
  import { traceView, selectedSpanId, collapsedSpans } from '../stores/trace';
  import SpanRow from './SpanRow.svelte';
  import type { SpanView } from '../types/trace';

  let containerEl: HTMLDivElement;

  // Precompute which span IDs have at least one child
  function buildChildSet(spans: SpanView[]): Set<string> {
    const s = new Set<string>();
    for (const span of spans) {
      if (span.parent_span_id) s.add(span.parent_span_id);
    }
    return s;
  }

  // Filter to only visible spans (DFS order + collapsed subtree pruning)
  function visibleRows(spans: SpanView[], collapsed: Set<string>): SpanView[] {
    const out: SpanView[] = [];
    let skipDepth: number | null = null;
    for (const span of spans) {
      if (skipDepth !== null) {
        if (span.depth > skipDepth) continue;
        skipDepth = null;
      }
      out.push(span);
      if (collapsed.has(span.span_id)) skipDepth = span.depth;
    }
    return out;
  }

  $: spans = $traceView?.spans ?? [];
  $: childSet = buildChildSet(spans);
  $: visible = visibleRows(spans, $collapsedSpans);

  // Recreate virtualizer whenever the visible count changes so row addresses stay correct
  $: virtualizer = createVirtualizer<HTMLDivElement, HTMLDivElement>({
    count: visible.length,
    getScrollElement: () => containerEl,
    estimateSize: () => 32,
    overscan: 10,
  });

  function toggle(spanId: string) {
    collapsedSpans.update(prev => {
      const next = new Set(prev);
      if (next.has(spanId)) next.delete(spanId);
      else next.add(spanId);
      return next;
    });
  }

  function select(spanId: string) {
    selectedSpanId.set(spanId);
  }
</script>

{#if !$traceView}
  <div class="empty">Load a trace to begin</div>
{:else}
  <div class="span-tree" bind:this={containerEl}>
    <div style="height: {$virtualizer.getTotalSize()}px; position: relative;">
      {#each $virtualizer.getVirtualItems() as item (item.key)}
        {@const span = visible[item.index]}
        <div
          style="position: absolute; top: 0; left: 0; width: 100%; transform: translateY({item.start}px);"
        >
          <SpanRow
            {span}
            isSelected={$selectedSpanId === span.span_id}
            isCollapsed={$collapsedSpans.has(span.span_id)}
            hasChildren={childSet.has(span.span_id)}
            on:select={(e) => select(e.detail)}
            on:toggle={(e) => toggle(e.detail)}
          />
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .span-tree {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: var(--color-bg);
    font-family: var(--font-sans);
  }

  .empty {
    padding: 24px;
    color: var(--color-text-secondary);
    font-family: var(--font-sans);
    font-size: 14px;
  }
</style>
