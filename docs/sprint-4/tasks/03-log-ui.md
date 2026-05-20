# Task 03: Log UI — Logs Section & Unmatched Logs Panel

**Sprint:** 4 — Log Loading & Correlation  
**Branch:** `feature/sprint-4-task-04-log-ui`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 4 / Task 02 (correlation command must exist); Sprint 3 / Task 03 (LogsSection placeholder must exist)

---

## Objective

Wire the log correlation result into the UI:
1. Populate the `LogsSection.svelte` placeholder (built in Sprint 3 Task 03) with actual log entries for the selected span
2. Build the `UnmatchedLogsPanel.svelte` accessible from the trace header area

---

## Acceptance Criteria

- [ ] `Toolbar.svelte` has a "Load Logs" button that calls `invoke('load_logs')` and merges the result into stores
- [ ] `src/lib/stores/trace.ts` adds `unmatchedLogs` store update logic after `load_logs` returns
- [ ] After loading logs, `$traceView.spans` are updated in place with correlated log entries
- [ ] `LogsSection.svelte` (in `SpanDetail`) shows logs for the selected span, sorted by timestamp ascending
- [ ] Each log entry shows: formatted timestamp, log message text, and a collapsed label list (expandable)
- [ ] `LogsSection.svelte` shows "No logs for this span" when `span.logs` is empty
- [ ] `UnmatchedLogsPanel.svelte` is toggled from a button in `TraceHeader.svelte`
- [ ] Unmatched logs panel shows all logs in `$unmatchedLogs`, sorted by timestamp
- [ ] All colors and spacing use CSS tokens

---

## Files to Create / Modify

- `src/lib/components/LogsSection.svelte` — replace placeholder with real log list
- `src/lib/components/UnmatchedLogsPanel.svelte` — new panel component
- `src/lib/components/TraceHeader.svelte` — add "Unmatched Logs (N)" toggle button
- `src/lib/components/Toolbar.svelte` — add "Load Logs" button
- `src/lib/stores/trace.ts` — handle `LogCorrelationResult` returned from `load_logs`
- `src/lib/types/log.ts` — TypeScript type for `LogEntryView`

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: load trace + load log file → select a span with matching logs → logs appear in detail panel
- [ ] Manual test: unmatched logs panel opens and shows unmatched entries
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-4-task-04-log-ui`
3. Read `docs/ARCHITECTURE.md` Sections 5 and 6.1.
4. Implement the components and store updates.
5. Manual test: load the example trace and log files from `docs/examples/` — verify the full correlation flow in the UI.
6. Create `docs/sprint-4/rust-notes/03-log-ui-notes.md` (Russian, gitignored) explaining: Svelte store updates with `update()` vs `set()`, merging arrays in TypeScript, sorting by timestamp.
7. **Pre-commit review:** Present `git diff`, walk through the UI flow from button click to log display, and wait for explicit approval.
8. **After approval:** Commit: `feat(ui): implement log display in SpanDetail and UnmatchedLogsPanel`.
9. Push and create PR linking the GitHub Issue from the header.
