# Task 04: Main Layout with Resizable Divider

**Sprint:** 3 — Trace Visualization  
**Branch:** `feature/sprint-3-task-03-layout`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 3 / Task 03 (SpanTree and SpanDetail must exist)

---

## Objective

Assemble the full application layout: Toolbar → TraceHeader → MainLayout (SpanTree | Divider | SpanDetail).

The `ResizableDivider.svelte` component allows the user to drag the boundary between the span tree
and the detail panel, adjusting the width ratio of the two panels.

---

## Acceptance Criteria

- [ ] `MainLayout.svelte` places `SpanTree` and `SpanDetail` side by side in a flex/grid layout
- [ ] `ResizableDivider.svelte` is a draggable vertical bar between the two panels
- [ ] Dragging the divider resizes both panels (CSS `flex-basis` or `grid-template-columns` update live)
- [ ] Minimum width enforced: neither panel collapses below 200px
- [ ] Divider drag works on both mouse and trackpad (pointer events, not mouse-only)
- [ ] Full layout assembles in `App.svelte`: Toolbar (top) → TraceHeader (below toolbar) → MainLayout (fills remaining height)
- [ ] Layout fills the full window height; no scroll on the outer page

---

## Files to Create / Modify

- `src/lib/components/MainLayout.svelte` — two-panel layout container
- `src/lib/components/ResizableDivider.svelte` — draggable divider
- `src/App.svelte` — full layout assembly

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: drag divider — both panels resize; neither goes below 200px
- [ ] Manual test: load a trace — full layout renders correctly end-to-end
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-3-task-03-layout`
3. Read `docs/ARCHITECTURE.md` Section 6.1 for the full component hierarchy.
4. Implement `MainLayout`, `ResizableDivider`, and update `App.svelte`.
5. Manual test: drag the divider, load a trace, verify the full layout end-to-end.
6. Create `docs/sprint-3/rust-notes/04-layout-notes.md` (Russian, gitignored) explaining: CSS Flexbox vs Grid, pointer events in the browser, Svelte event handling, CSS `min-width`.
7. **Pre-commit review:** Present `git diff`, describe the divider drag implementation, and wait for explicit approval.
8. **After approval:** Commit: `feat(ui): assemble main layout with resizable divider`.
9. Push and create PR linking the GitHub Issue from the header.
