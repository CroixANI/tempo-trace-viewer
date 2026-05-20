# Task 01: Loki Log Parser

**Sprint:** 4 — Log Loading & Correlation  
**Branch:** `feature/sprint-4-task-01-log-parser`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/11  
**Depends on:** Sprint 2 / Task 01 (data model must exist)

---

## Objective

Implement the Rust parser that reads a Loki JSON query response file and produces
`Vec<LogEntryView>`, ready for correlation.

The Loki format uses the `data.result[]` streams structure where each stream has
a `stream` object (labels map) and a `values` array of `[timestamp_ns, log_line]` pairs.

---

## Acceptance Criteria

- [ ] `parse_log_file(path: &Path) -> Result<Vec<LogEntryView>, LogParseError>` is implemented
- [ ] Each `LogEntryView` has: `timestamp_ns`, `message`, `labels` (from stream labels), `span_id` (extracted from labels if present)
- [ ] Handles files with no log entries (returns empty `Vec` without panicking)
- [ ] `LogParseError` covers: file not found, invalid JSON, missing `data.result` field
- [ ] Unit tests pass for: valid file, empty result, missing `span_id` label
- [ ] `cargo test` passes

---

## Files to Create / Modify

- `src-tauri/src/parser/log_parser.rs` — parser implementation + unit tests
- `src-tauri/src/parser/mod.rs` — add `pub mod log_parser`

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] `cargo test` passes
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-4-task-01-log-parser`
3. Read `docs/ARCHITECTURE.md` Section 3.3 for the Loki data model.
4. Look at `docs/examples/` for a Loki JSON example file to understand the real format before implementing.
5. Implement the parser and tests.
6. Run `cargo test` — all tests must pass.
7. Create `docs/sprint-4/rust-notes/01-log-parser-notes.md` (Russian, gitignored) explaining: `HashMap<String, String>` for labels, tuple types `(String, String)` for values array, reusing `thiserror` pattern from the trace parser.
8. **Pre-commit review:** Present `git diff`, describe the Loki format and how `span_id` is extracted from labels, and wait for explicit approval.
9. **After approval:** Commit: `feat(parser): implement Loki JSON log file parser`.
10. Push and create PR linking the GitHub Issue from the header.
