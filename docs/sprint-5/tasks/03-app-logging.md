# Task 03: Application Operational Logging

**Sprint:** 5 — Search & Polish  
**Branch:** `feature/sprint-5-task-03-app-logging`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 2 / Task 03 (logging.rs stub must exist)

---

## Objective

Finalize the operational logging system (not trace data — the app's own log output).

`logging.rs` was stubbed in Sprint 2 Task 03. This task wires it fully:
- Rolling file appender with 10MB cap
- `INFO` level in release, `DEBUG` in dev
- `tracing::instrument` on key command functions
- Log file location per platform as specified in PRD Section 7

---

## Acceptance Criteria

- [ ] `init_logging()` is called at app startup (in `lib.rs`)
- [ ] Log file is written to the platform-correct location:
  - macOS: `~/Library/Logs/{bundle-id}/tempo-trace-viewer.log`
  - Windows: `%APPDATA%\{bundle-id}\logs\tempo-trace-viewer.log`
- [ ] Log file is created on first run if it does not exist
- [ ] Log entries include: timestamp, level, module path, message
- [ ] `load_trace` command logs: start of load (INFO), file path (DEBUG), parse duration (INFO), error (ERROR)
- [ ] `load_logs` command logs: start of load (INFO), number of entries parsed (INFO), correlation result counts (INFO)
- [ ] Log level is `DEBUG` in debug builds, `INFO` in release builds (compile-time `cfg(debug_assertions)`)
- [ ] Manual test: run app in debug mode, load a trace, verify log file is created and contains expected entries

---

## Files to Create / Modify

- `src-tauri/src/logging.rs` — finalize implementation per `ARCHITECTURE.md` Section 8
- `src-tauri/src/commands/load_trace.rs` — add `tracing::info!` / `tracing::debug!` calls
- `src-tauri/src/commands/load_logs.rs` — add tracing calls

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: log file present at correct path with expected content after loading a trace
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-5-task-03-app-logging`
3. Read `docs/ARCHITECTURE.md` Section 8 and PRD Section 7 before writing any code.
4. Implement `logging.rs` per the architecture spec.
5. Add tracing calls to `load_trace` and `load_logs` commands.
6. Run `cargo tauri dev`, load the example trace, then `cat ~/Library/Logs/{bundle-id}/tempo-trace-viewer.log` to verify output.
7. Create `docs/sprint-5/rust-notes/03-app-logging-notes.md` (Russian, gitignored) explaining: `tracing` vs `log` crate, spans vs events in tracing, `tracing-appender` rolling files, `cfg(debug_assertions)` compile-time flags.
8. **Pre-commit review:** Present `git diff`, include the actual log file output from the manual test, and wait for explicit approval.
9. **After approval:** Commit: `feat(logging): finalize operational logging with rolling file appender and tracing instrumentation`.
10. Push and create PR linking the GitHub Issue from the header.
