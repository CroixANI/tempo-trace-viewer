# Task 02: Span Tree with Gantt Bars

**Sprint:** 3 — Trace Visualization  
**Branch:** `feature/sprint-3-task-02-span-tree`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/8  
**Depends on:** Sprint 3 / Task 01 (stores and types must exist)

---

## Objective

Implement the span tree — the primary visualization of the trace. This is the most complex
UI component in the application.

The span tree is a virtualized list (`@tanstack/virtual`) where each row shows:
- Indented span name (depth-based indent)
- Service name badge (color-coded by service)
- Duration label
- Gantt bar (positioned by `relative_start_pct` and `duration_pct` from `SpanView`)
- Collapse/expand toggle for spans that have children

---

## Acceptance Criteria

- [ ] `@tanstack/virtual` is installed (verify 7-day age rule before installing)
- [ ] `SpanTree.svelte` renders all spans from `$traceView.spans` using `@tanstack/virtual`
- [ ] `SpanRow.svelte` renders: indent, collapse toggle (if has children), service badge, span name, duration
- [ ] `GanttBar.svelte` renders a bar positioned using `SpanView.relative_start_pct` and `SpanView.duration_pct` (CSS `left` + `width` as percentages)
- [ ] Service color is consistent per service name — same service always gets the same hue (use a deterministic hash of the service name string → hue value)
- [ ] Collapsed subtrees hide all descendant rows; expand/collapse toggle works correctly
- [ ] Selecting a span row sets `$selectedSpanId` in the store
- [ ] Selected row is visually highlighted
- [ ] Scroll is smooth at 60fps with 5,000 spans (verify manually with example file)
- [ ] All colors and spacing use CSS tokens

---

## Files to Create / Modify

- `src/lib/components/SpanTree.svelte` — virtualizer container
- `src/lib/components/SpanRow.svelte` — single span row (virtual item)
- `src/lib/components/GanttBar.svelte` — Gantt timeline bar
- `src/lib/stores/trace.ts` — add `collapsedSpans` store (`Set<string>` of collapsed span IDs)
- `src/lib/styles/tokens.css` — add span tree layout tokens if missing
- `package.json` — add `@tanstack/virtual`

---

## Dependency Age Check

Before installing `@tanstack/virtual`, run the npm check from `AGENT.md` Section 3.1.
Record the publish date in your pre-commit review summary.

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: 5,000-span trace scrolls at 60fps (check browser DevTools Performance panel)
- [ ] Manual test: collapse a parent span — all children disappear; expand — they return
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-3-task-02-span-tree`
3. Read `docs/ARCHITECTURE.md` Sections 6.1, 6.3, and the virtualization strategy before writing any code.
4. Run the dependency age check for `@tanstack/virtual` before installing it.
5. Implement `SpanTree`, `SpanRow`, and `GanttBar` components.
6. Manual test with the example trace: verify scroll fps, collapse/expand, and service color consistency.
7. Create `docs/sprint-3/rust-notes/02-span-tree-notes.md` (Russian, gitignored) explaining: virtual DOM vs real DOM, why virtualization matters at 5,000 items, CSS `position: absolute` for Gantt bars, hash functions for color derivation.
8. **Pre-commit review:** Present `git diff`, describe the virtualization approach and color hashing algorithm, confirm `@tanstack/virtual` age check passed, and wait for explicit approval.
9. **After approval:** Commit: `feat(ui): implement virtualized span tree with Gantt bars and service color coding`.
10. Push and create PR linking the GitHub Issue from the header.
