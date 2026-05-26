<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{ drag: number }>();

  let dragging = false;

  function onPointerDown(e: PointerEvent) {
    dragging = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    e.preventDefault();
  }

  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    dispatch('drag', e.clientX);
  }

  function onPointerUp(e: PointerEvent) {
    dragging = false;
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="divider"
  class:dragging
  on:pointerdown={onPointerDown}
  on:pointermove={onPointerMove}
  on:pointerup={onPointerUp}
  on:pointercancel={onPointerUp}
>
  <div class="handle"></div>
</div>

<style>
  .divider {
    flex-shrink: 0;
    width: 6px;
    cursor: col-resize;
    background-color: var(--color-border);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.1s;
    user-select: none;
  }

  .divider:hover,
  .divider.dragging {
    background-color: var(--color-accent);
  }

  .handle {
    width: 2px;
    height: 32px;
    border-radius: 1px;
    background-color: var(--color-text-secondary);
    opacity: 0.4;
  }

  .divider:hover .handle,
  .divider.dragging .handle {
    opacity: 0;
  }
</style>
