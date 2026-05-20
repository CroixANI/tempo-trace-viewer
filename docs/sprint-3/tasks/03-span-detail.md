# Task 03: Span Detail Panel

**Sprint:** 3 — Trace Visualization  
**Branch:** `feature/sprint-3-task-03-span-detail`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/9  
**Depends on:** Sprint 3 / Task 02 (`selectedSpanId` store must exist)

---

## Objective

Build the `SpanDetail.svelte` panel shown on the right side when a span is selected.

The panel has three collapsible sections:
1. **Tags** — span attributes as a key/value table
2. **Process / Resource** — resource attributes from the span's parent resource
3. **Logs** — span-correlated logs (`SpanView.logs`). Empty in this sprint; populated in Sprint 4.

---

## Acceptance Criteria

- [ ] `SpanDetail.svelte` is shown when `$selectedSpanId` is not null; hidden otherwise
- [ ] Derives the selected `SpanView` from `$traceView.spans` using `$selectedSpanId`
- [ ] **Tags section** renders `SpanView.attributes` as a two-column table (key | value)
- [ ] **Resource section** renders `SpanView.resource_attributes` as a two-column table
- [ ] **Logs section** renders a placeholder "No logs loaded" message (Sprint 4 will populate this)
- [ ] Each section is collapsible (toggle open/close)
- [ ] Long string values truncate with a "show more" toggle rather than overflowing the panel
- [ ] All colors, spacing, fonts use CSS tokens

---

## Files to Create / Modify

- `src/lib/components/SpanDetail.svelte` — detail panel container
- `src/lib/components/TagsSection.svelte` — tags table
- `src/lib/components/ResourceSection.svelte` — resource attributes table
- `src/lib/components/LogsSection.svelte` — logs placeholder (extended in Sprint 4)

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: select a span with multiple tags — tags render in table form; resource section renders resource attributes
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-3-task-03-span-detail`
3. Read `docs/ARCHITECTURE.md` Section 6.1 for the component hierarchy.
4. Implement `SpanDetail`, `TagsSection`, `ResourceSection`, and `LogsSection` (placeholder).
5. Manual test: select different spans — verify tags and resource sections populate correctly.
6. Create `docs/sprint-3/rust-notes/03-span-detail-notes.md` (Russian, gitignored) explaining: Svelte derived stores, reactive `$:` declarations, conditional rendering in Svelte (`{#if}`), Svelte `{#each}` blocks.
7. **Pre-commit review:** Present `git diff`, describe how the selected span is derived from the store, and wait for explicit approval.
8. **After approval:** Commit: `feat(ui): add SpanDetail panel with Tags and Resource sections`.
9. Push and create PR linking the GitHub Issue from the header.
