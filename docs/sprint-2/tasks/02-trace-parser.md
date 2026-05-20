# Task 02: Implement Trace Parser

**Sprint:** 2 — Data Model & Trace Parsing  
**Branch:** `feature/sprint-2-task-02-trace-parser`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 2 / Task 01 (data model must exist)

---

## Objective

Implement the Rust parser that reads a Tempo JSON file from disk and produces a `TraceView`.

The parser lives in `src-tauri/src/parser/trace_parser.rs` and performs:
1. Read file bytes from a given path
2. Deserialize the raw `batches[]` structure via `serde_json`
3. Build the flattened `TraceView` + `Vec<SpanView>` including:
   - Tree depth calculation (walk parent_span_id chains)
   - Relative start offset per span (span.start - trace.start)
   - Service name extraction from resource attributes (`service.name` key)
   - Error detection (`span.status.code == ERROR`)
   - Gantt percentage fields: `relative_start_pct`, `duration_pct` (0.0–1.0 of total trace duration)

---

## Acceptance Criteria

- [ ] `parse_trace_file(path: &Path) -> Result<TraceView, TraceParseError>` is implemented
- [ ] Handles a file with 0 spans (returns an empty `TraceView` without panicking)
- [ ] Handles a file with 5,000 spans without exceeding 2 seconds parse time (verify manually with the example file from `docs/examples/`)
- [ ] `TraceView.error_count` correctly counts spans with status code `ERROR`
- [ ] `SpanView.depth` is correct for a 3-level nested trace (verify with example file)
- [ ] `TraceParseError` uses `thiserror` and covers: file not found, invalid JSON, missing required fields
- [ ] `cargo test` passes for the parser unit tests

---

## Files to Create / Modify

- `src-tauri/src/parser/mod.rs` — module declarations
- `src-tauri/src/parser/trace_parser.rs` — parser implementation + unit tests
- `src-tauri/src/lib.rs` — add `mod parser;`

---

## Test Cases to Include

Write unit tests in `trace_parser.rs` (inline `#[cfg(test)]` module):
1. Parse the example trace file from `docs/examples/` — assert span count matches expected
2. Parse an empty `batches: []` — assert `span_count == 0`
3. Parse a minimal 2-span parent/child trace inline — assert `child.depth == 1`, `parent.depth == 0`

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
2. **Create branch:** `git checkout -b feature/sprint-2-task-02-trace-parser`
3. Read `docs/ARCHITECTURE.md` Sections 3 and 4 before writing any code.
4. Look at `docs/examples/` for a real Tempo JSON file to understand the actual shape of the data before implementing deserialization.
5. Implement the parser and tests.
6. Run `cargo test` — all tests must pass before proceeding to review.
7. Create `docs/sprint-2/rust-notes/02-trace-parser-notes.md` (Russian, gitignored) explaining: `serde_json::from_str` vs `from_reader`, `Result<T, E>` and the `?` operator, `thiserror` derive macro, recursive tree traversal in Rust, iterator methods (`map`, `filter`, `collect`).
8. **Pre-commit review:** Present `git diff`, summarize the parsing algorithm and each test case, and wait for explicit approval.
9. **After approval:** Commit: `feat(parser): implement Tempo JSON trace parser with tree depth and Gantt calculations`.
10. Push and create PR linking the GitHub Issue from the header.
