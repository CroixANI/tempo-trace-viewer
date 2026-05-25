<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  async function loadTrace() {
    const result = await invoke('load_trace');
    console.log('TraceView:', result);
  }
</script>

<main>
  <h1>Tempo Trace Viewer</h1>
  <p>Load a Tempo trace file to get started.</p>
  <button onclick={loadTrace}>Load Trace</button>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-family: var(--font-sans);
    color: var(--color-text-primary);
    background-color: var(--color-bg);
  }

  h1 {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  p {
    color: var(--color-text-secondary);
  }
</style>
