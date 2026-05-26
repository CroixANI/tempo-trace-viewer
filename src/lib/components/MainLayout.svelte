<script lang="ts">
  import SpanTree from './SpanTree.svelte';
  import SpanDetail from './SpanDetail.svelte';
  import ResizableDivider from './ResizableDivider.svelte';
  import { selectedSpanId } from '../stores/trace';

  const MIN_WIDTH = 200;

  let containerEl: HTMLDivElement;
  let leftWidth = 60; // percent

  function onDrag(e: CustomEvent<number>) {
    if (!containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    const totalWidth = rect.width;
    const rawPx = e.detail - rect.left;
    const minPct = (MIN_WIDTH / totalWidth) * 100;
    const maxPct = 100 - (MIN_WIDTH / totalWidth) * 100 - (6 / totalWidth) * 100;
    leftWidth = Math.min(Math.max((rawPx / totalWidth) * 100, minPct), maxPct);
  }
</script>

<div class="main-layout" bind:this={containerEl}>
  <div class="left-panel" style="width: {leftWidth}%;">
    <SpanTree />
  </div>

  <ResizableDivider on:drag={onDrag} />

  {#if $selectedSpanId}
    <div class="right-panel">
      <SpanDetail />
    </div>
  {:else}
    <div class="right-panel right-empty">
      <span>Select a span to inspect</span>
    </div>
  {/if}
</div>

<style>
  .main-layout {
    flex: 1;
    display: flex;
    flex-direction: row;
    overflow: hidden;
    min-height: 0;
  }

  .left-panel {
    min-width: 200px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .right-panel {
    flex: 1;
    min-width: 200px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .right-empty {
    align-items: center;
    justify-content: center;
    color: var(--color-text-secondary);
    font-family: var(--font-sans);
    font-size: 13px;
    background-color: var(--color-surface);
  }
</style>
