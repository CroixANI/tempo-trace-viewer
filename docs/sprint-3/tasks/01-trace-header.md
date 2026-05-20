# Task 01: Trace Header Component

**Sprint:** 3 — Trace Visualization  
**Branch:** `feature/sprint-3-task-01-trace-header`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 2 / Task 03 (IPC command must exist; `TraceView` type available)

---

## Objective

Build the `TraceHeader.svelte` component that displays top-level trace metadata,
and wire it to the Svelte `traceView` store.

This is the first visual component. It sets the pattern for how all components
read from stores and use CSS tokens — subsequent tasks follow the same pattern.

---

## Acceptance Criteria

- [ ] `src/lib/stores/trace.ts` defines `traceView`, `selectedSpanId`, `searchQuery`, `unmatchedLogs` stores as specified in `ARCHITECTURE.md` Section 6.2
- [ ] `src/lib/types/trace.ts` defines TypeScript interfaces mirroring the Rust `TraceView` and `SpanView` types
- [ ] `TraceHeader.svelte` displays: service name, operation name, trace ID, formatted start time, total duration, service count, span count, error count
- [ ] All colors, fonts, and spacing use CSS custom properties from `tokens.css` — no hardcoded values
- [ ] Error count is styled in `--color-error` when > 0; in `--color-text-secondary` when 0
- [ ] Component renders nothing (or a "Load a trace to begin" placeholder) when `$traceView` is null
- [ ] `App.svelte` is updated to call `invoke('load_trace')` from a toolbar button and populate the `traceView` store on success

---

## Files to Create / Modify

- `src/lib/stores/trace.ts` — Svelte writable stores
- `src/lib/types/trace.ts` — TypeScript type definitions
- `src/lib/components/TraceHeader.svelte` — header component
- `src/lib/components/Toolbar.svelte` — Load Trace button
- `src/App.svelte` — wire Toolbar and TraceHeader

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: load example trace → header shows correct metadata
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-3-task-01-trace-header`
3. Read `docs/ARCHITECTURE.md` Sections 6.1, 6.2, and 7 before writing any code.
4. Run `cargo tauri dev` and verify the IPC command from Sprint 2 Task 03 still works before starting.
5. Implement stores, types, and components.
6. Manual test: load the example trace from `docs/examples/` — verify all metadata fields show correctly.
7. Create `docs/sprint-3/rust-notes/01-trace-header-notes.md` (Russian, gitignored) explaining: Svelte stores (`writable`, `derived`), reactive declarations in Svelte (`$store` syntax), TypeScript interfaces vs Rust structs, CSS custom properties.
8. **Pre-commit review:** Present `git diff`, show a screenshot or describe the rendered header, and wait for explicit approval.
9. **After approval:** Commit: `feat(ui): add TraceHeader component and Svelte stores`.
10. Push and create PR linking the GitHub Issue from the header.
