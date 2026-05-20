# Task 03: Tauri IPC — load_trace Command

**Sprint:** 2 — Data Model & Trace Parsing  
**Branch:** `feature/sprint-2-task-03-ipc-trace-command`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/6  
**Depends on:** Sprint 2 / Task 02 (trace parser must exist)

---

## Objective

Wire the trace parser to a Tauri command so the frontend can trigger file loading.

The `load_trace` command:
1. Opens a native file picker (filtered to `.json` files)
2. Calls `parse_trace_file()` on the selected path
3. Stores the result in `AppState`
4. Returns the `TraceView` to the frontend as a single JSON payload

This task also initializes `AppState` and the operational logging system.

---

## Acceptance Criteria

- [ ] `src-tauri/src/state.rs` defines `AppState` as specified in `ARCHITECTURE.md` Section 4.2
- [ ] `src-tauri/src/logging.rs` initializes `tracing` + `tracing-appender` as specified in `ARCHITECTURE.md` Section 8
- [ ] `load_trace` command is registered in `lib.rs` and callable from the frontend via `invoke('load_trace')`
- [ ] File picker opens filtered to `.json` files only
- [ ] On success: `AppState.trace` is updated and `TraceView` is returned to the frontend
- [ ] On parse error: command returns a descriptive error string (not a panic)
- [ ] If the user cancels the file dialog (no file selected): command returns `Ok(null)` — not an error
- [ ] `clear_session` command is implemented and clears `AppState.trace` and `AppState.log_entries`
- [ ] A simple frontend test page calls `invoke('load_trace')` and `console.log`s the result — verified manually in `cargo tauri dev`

---

## Files to Create / Modify

- `src-tauri/src/state.rs` — `AppState` definition
- `src-tauri/src/logging.rs` — logging initialization
- `src-tauri/src/commands/mod.rs` — module declarations
- `src-tauri/src/commands/load_trace.rs` — `load_trace` command
- `src-tauri/src/commands/clear_session.rs` — `clear_session` command
- `src-tauri/src/lib.rs` — register commands and `AppState`, call `init_logging`
- `src/App.svelte` — temporary test button to invoke `load_trace` and log result

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: load the example trace from `docs/examples/` via the test button — `TraceView` appears in browser console
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-2-task-03-ipc-trace-command`
3. Read `docs/ARCHITECTURE.md` Sections 4 and 8 before writing any code.
4. Implement `AppState`, logging init, commands, and frontend test.
5. Run `cargo tauri dev` and manually test loading the example trace file. Verify the full `TraceView` JSON appears in the browser console.
6. Create `docs/sprint-2/rust-notes/03-ipc-trace-command-notes.md` (Russian, gitignored) explaining: Tauri `#[tauri::command]` macro, `State<T>` managed state, `Mutex` and why it's needed for shared state, `async fn` in Tauri commands, `AppHandle` and what it provides.
7. **Pre-commit review:** Present `git diff`, describe the IPC flow end-to-end (user click → file dialog → parser → state → frontend), and wait for explicit approval.
8. **After approval:** Commit: `feat(ipc): implement load_trace command with AppState and operational logging`.
9. Push and create PR linking the GitHub Issue from the header.
