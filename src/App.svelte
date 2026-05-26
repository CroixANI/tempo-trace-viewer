<script lang="ts">
  import Toolbar from './lib/components/Toolbar.svelte';
  import TraceHeader from './lib/components/TraceHeader.svelte';
  import MainLayout from './lib/components/MainLayout.svelte';
</script>

<div class="app">
  <Toolbar />
  <TraceHeader />
  <MainLayout />
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    font-family: var(--font-sans);
    background-color: var(--color-bg);
    color: var(--color-text-primary);
  }
</style>
