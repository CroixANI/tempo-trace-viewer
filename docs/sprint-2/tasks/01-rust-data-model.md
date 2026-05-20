# Task 01: Define Rust Data Model

**Sprint:** 2 — Data Model & Trace Parsing  
**Branch:** `feature/sprint-2-task-01-rust-data-model`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 1 / Task 01 (project scaffold must exist)

---

## Objective

Define all Rust data model types used throughout the application:
- Raw deserialization types for OTEL/Tempo JSON (`model/trace.rs`, `model/log.rs`)
- Frontend view model types that cross the IPC boundary (`model/view.rs`)

No parsing logic in this task — types only. The parser tasks (02, 03) depend on these types existing first.

---

## Acceptance Criteria

- [ ] `src-tauri/src/model/trace.rs` contains all raw OTEL types from `ARCHITECTURE.md` Section 3.1
- [ ] `src-tauri/src/model/log.rs` contains all Loki types from `ARCHITECTURE.md` Section 3.3
- [ ] `src-tauri/src/model/view.rs` contains `TraceView`, `SpanView`, `LogEntryView` from `ARCHITECTURE.md` Section 3.2
- [ ] All structs derive `Debug`, `Serialize`, `Deserialize`, and `Clone`
- [ ] `src-tauri/src/model/mod.rs` re-exports all public types
- [ ] `cargo build` compiles without errors or warnings

---

## Files to Create / Modify

- `src-tauri/src/model/mod.rs` — module declarations and re-exports
- `src-tauri/src/model/trace.rs` — raw OTEL/Tempo types
- `src-tauri/src/model/log.rs` — raw Loki types
- `src-tauri/src/model/view.rs` — frontend view model types
- `src-tauri/src/lib.rs` — add `mod model;`

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] `cargo build` clean (no warnings)
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-2-task-01-rust-data-model`
3. Read `docs/ARCHITECTURE.md` Sections 3.1, 3.2, 3.3 before writing any code. Implement exactly what is specified — do not add fields or types not listed there.
4. Implement the acceptance criteria above.
5. Create `docs/sprint-2/rust-notes/01-rust-data-model-notes.md` (Russian, gitignored) explaining: structs vs enums in Rust, `derive` macros, `serde` Serialize/Deserialize, `Option<T>`, `Vec<T>`, `HashMap`, ownership and why `Clone` is needed for IPC.
6. **Pre-commit review:** Run `git diff`, present the full diff, summarize what each type represents in the OTEL data model, and wait for explicit approval.
7. **After approval:** Stage specific files, commit: `feat(model): define Rust data model types for OTEL trace, Loki logs, and frontend view`.
8. Push and create PR: `gh pr create` with title `feat(sprint-2): define Rust data model` linking the GitHub Issue from the header.
