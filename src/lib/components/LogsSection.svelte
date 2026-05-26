<script lang="ts">
  import type { LogEntryView } from '../types/trace';

  export let logs: LogEntryView[] = [];

  let open = true;
</script>

<section class="detail-section">
  <button class="section-header" on:click={() => (open = !open)}>
    <span class="chevron">{open ? '▼' : '▶'}</span>
    <span class="section-title">Logs</span>
    <span class="count">({logs.length})</span>
  </button>

  {#if open}
    <p class="placeholder">No logs loaded — log correlation coming in Sprint 4.</p>
  {/if}
</section>

<style>
  .detail-section {
    border-bottom: 1px solid var(--color-border);
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 8px 12px;
    background: none;
    border: none;
    cursor: pointer;
    text-align: left;
    font-family: var(--font-sans);
    font-size: 12px;
    font-weight: 600;
    color: var(--color-text-primary);
    background-color: var(--color-surface);
  }

  .section-header:hover {
    background-color: var(--color-border);
  }

  .chevron {
    font-size: 8px;
    color: var(--color-text-secondary);
  }

  .section-title {
    flex: 1;
  }

  .count {
    color: var(--color-text-secondary);
    font-weight: normal;
  }

  .placeholder {
    padding: 8px 12px;
    font-size: 12px;
    color: var(--color-text-secondary);
    font-family: var(--font-sans);
    font-style: italic;
    margin: 0;
  }
</style>
