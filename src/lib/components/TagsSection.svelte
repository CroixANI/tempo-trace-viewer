<script lang="ts">
  import type { KeyValue, AnyValue } from '../types/trace';

  export let attributes: KeyValue[];
  export let title: string = 'Tags';

  let open = true;

  function formatValue(v: AnyValue): string {
    if ('string_value' in v) return v.string_value;
    if ('bool_value' in v) return String(v.bool_value);
    if ('int_value' in v) return String(v.int_value);
    if ('double_value' in v) return String(v.double_value);
    if ('array_value' in v) return v.array_value.map(formatValue).join(', ');
    if ('kvlist_value' in v) return v.kvlist_value.map(kv => `${kv.key}=${formatValue(kv.value)}`).join(', ');
    return '';
  }

  const TRUNCATE_AT = 120;
  let expanded = new Set<number>();

  function toggle(i: number) {
    expanded = expanded.has(i) ? (expanded.delete(i), new Set(expanded)) : new Set(expanded).add(i);
  }
</script>

<section class="detail-section">
  <button class="section-header" on:click={() => (open = !open)}>
    <span class="chevron">{open ? '▼' : '▶'}</span>
    <span class="section-title">{title}</span>
    <span class="count">({attributes.length})</span>
  </button>

  {#if open}
    {#if attributes.length === 0}
      <p class="empty">No attributes</p>
    {:else}
      <table class="kv-table">
        <tbody>
          {#each attributes as kv, i}
            {@const raw = formatValue(kv.value)}
            {@const long = raw.length > TRUNCATE_AT}
            {@const show = expanded.has(i)}
            <tr>
              <td class="key">{kv.key}</td>
              <td class="value">
                {show || !long ? raw : raw.slice(0, TRUNCATE_AT) + '…'}
                {#if long}
                  <button class="toggle-more" on:click={() => toggle(i)}>
                    {show ? 'show less' : 'show more'}
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
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

  .empty {
    padding: 8px 12px;
    font-size: 12px;
    color: var(--color-text-secondary);
    font-family: var(--font-sans);
    margin: 0;
  }

  .kv-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
    font-family: var(--font-sans);
  }

  .kv-table tr:hover {
    background-color: var(--color-surface);
  }

  td {
    padding: 4px 12px;
    vertical-align: top;
    border-bottom: 1px solid var(--color-border);
  }

  .key {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--color-text-secondary);
    white-space: nowrap;
    width: 40%;
    word-break: break-all;
  }

  .value {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--color-text-primary);
    word-break: break-all;
  }

  .toggle-more {
    display: inline;
    margin-left: 4px;
    padding: 0;
    border: none;
    background: none;
    color: var(--color-accent);
    font-size: 11px;
    cursor: pointer;
    font-family: var(--font-sans);
  }
</style>
