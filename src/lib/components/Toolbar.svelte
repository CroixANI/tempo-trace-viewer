<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { TraceView } from '../types/trace';
  import { traceView } from '../stores/trace';

  async function loadTrace() {
    try {
      const result = await invoke<TraceView>('load_trace');
      traceView.set(result);
    } catch (err) {
      console.error('Failed to load trace:', err);
    }
  }
</script>

<div class="toolbar">
  <span class="app-title">Tempo Trace Viewer</span>
  <button class="load-btn" onclick={loadTrace}>Load Trace</button>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0 1rem;
    height: 48px;
    background-color: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    font-family: var(--font-sans);
    flex-shrink: 0;
  }

  .app-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .load-btn {
    padding: 0.375rem 1rem;
    background-color: var(--color-accent);
    color: #ffffff;
    border: none;
    font-family: var(--font-sans);
    font-size: 0.875rem;
    cursor: pointer;
  }

  .load-btn:hover {
    filter: brightness(1.1);
  }
</style>
