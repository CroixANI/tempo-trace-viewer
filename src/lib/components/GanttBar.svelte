<script lang="ts">
  export let startPct: number;  // 0..1 fraction from SpanView.relative_start_pct
  export let durationPct: number;  // 0..1 fraction from SpanView.duration_pct
  export let color: string;
</script>

<div class="gantt-track">
  <div
    class="gantt-bar"
    style="left: {startPct * 100}%; width: {Math.max(durationPct * 100, 0.3)}%; background-color: {color};"
  ></div>
</div>

<style>
  .gantt-track {
    position: relative;
    width: 100%;
    height: var(--gantt-bar-height);
  }

  .gantt-bar {
    position: absolute;
    top: 0;
    height: 100%;
    border-radius: 2px;
    min-width: 2px;
  }
</style>
