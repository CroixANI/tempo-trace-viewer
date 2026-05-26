<script lang="ts">
  import Toolbar from './lib/components/Toolbar.svelte';
  import TraceHeader from './lib/components/TraceHeader.svelte';
</script>

<div class="app">
  <Toolbar />
  <TraceHeader />
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    font-family: var(--font-sans);
    background-color: var(--color-bg);
    color: var(--color-text-primary);
  }
</style>
