# Task 02: Log Correlation & IPC Command

**Sprint:** 4 — Log Loading & Correlation  
**Branch:** `feature/sprint-4-task-02-log-correlation`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/12  
**Depends on:** Sprint 4 / Task 01 (log parser must exist); Sprint 2 / Task 03 (AppState must exist)

---

## Objective

Implement the log correlation algorithm in Rust and expose it via the `load_logs` Tauri command.

Correlation algorithm (from `ARCHITECTURE.md` Section 5):
- For each `LogEntryView`: if `span_id` is present, find the matching span and attach the log to it
- If no matching span exists, add to `unmatched_logs`
- If no `span_id` label, add to `unmatched_logs`

The `load_logs` command:
1. Opens a file dialog (filtered to `.json`)
2. Parses the selected file
3. Merges new log entries into `AppState.log_entries` (additive — does not replace existing logs)
4. Runs correlation against the current `AppState.trace`
5. Returns `LogCorrelationResult` to the frontend

---

## Acceptance Criteria

- [ ] `src-tauri/src/correlator.rs` implements `correlate(spans: &mut Vec<SpanView>, logs: &[LogEntryView]) -> Vec<LogEntryView>` (returns unmatched logs)
- [ ] Correlation is additive: loading a second log file adds to existing span logs, not replaces them
- [ ] `load_logs` command returns `LogCorrelationResult { updated_spans: Vec<SpanView>, unmatched_logs: Vec<LogEntryView> }`
- [ ] If no trace is loaded when `load_logs` is called: command returns an error "No trace loaded — load a trace first"
- [ ] `clear_session` command (from Sprint 2 Task 03) also clears `AppState.log_entries`
- [ ] Unit tests for correlator: all logs matched, no logs matched, partial match, empty log list

---

## Files to Create / Modify

- `src-tauri/src/correlator.rs` — correlation algorithm + unit tests
- `src-tauri/src/commands/load_logs.rs` — `load_logs` Tauri command
- `src-tauri/src/commands/mod.rs` — add `pub mod load_logs`
- `src-tauri/src/lib.rs` — register `load_logs` command
- `src-tauri/src/lib.rs` — update `clear_session` to clear log entries

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] `cargo test` passes
- [ ] Manual test: load trace, load log file, verify `updated_spans` are returned with logs attached
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-4-task-02-log-correlation`
3. Read `docs/ARCHITECTURE.md` Section 5 for the correlation algorithm.
4. Implement `correlator.rs` with unit tests first, then the `load_logs` command.
5. Run `cargo test` — all tests must pass.
6. Create `docs/sprint-4/rust-notes/02-log-correlation-notes.md` (Russian, gitignored) explaining: mutable references (`&mut`), borrowing rules, why we pass `&mut Vec<SpanView>` vs returning a new Vec, `HashMap` for O(1) span lookup by ID.
7. **Pre-commit review:** Present `git diff`, walk through the correlation algorithm with a concrete example (3 logs: 1 matched, 1 unmatched by missing span, 1 unmatched by no span_id), and wait for explicit approval.
8. **After approval:** Commit: `feat(correlator): implement log-to-span correlation and load_logs command`.
9. Push and create PR linking the GitHub Issue from the header.
