# Task 04: Performance Validation

**Sprint:** 5 — Search & Polish  
**Branch:** `feature/sprint-5-task-04-performance`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/17  
**Depends on:** All Sprint 3, 4, and 5 tasks complete

---

## Objective

Validate that all four performance targets from PRD Section 8 are met.
This is a measurement and optimization task — implement fixes only if a target is missed.

| Target | Measurement method |
|---|---|
| Trace parse + render < 2s (5,000 spans) | `console.time` around `invoke('load_trace')` call |
| Search keystroke < 100ms | DevTools Performance panel, record while typing |
| 60fps scroll | DevTools Performance panel, record while scrolling |
| Memory < 200MB (5,000 spans + logs) | DevTools Memory panel, heap snapshot after loading |

---

## Acceptance Criteria

- [ ] All four targets from PRD Section 8 are met on macOS (Apple Silicon)
- [ ] Measurement results are documented in `docs/sprint-5/performance-report.md`
- [ ] If any target is missed: the bottleneck is identified (profiler screenshot) and a fix is implemented before this task is marked done
- [ ] No regressions introduced — existing features still work after any optimization changes

---

## Files to Create / Modify

- `docs/sprint-5/performance-report.md` — measurement results (git tracked)
- Any file where a bottleneck is found — optimization changes

---

## Definition of Done

- [ ] All four performance targets confirmed met
- [ ] `performance-report.md` written with measurements and methodology
- [ ] Code reviewed and approved by user before commit (if any code changes made)
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-5-task-04-performance`
3. Run `cargo tauri build` to get a release binary for measurements (debug builds are not representative).
4. Open the app, open DevTools (if Tauri config allows in release build — if not, test in `tauri dev` with `--release` flag).
5. Measure each of the four targets using the methods above. Record raw numbers.
6. If a target is missed:
   - Trace parse > 2s: profile the Rust parser with `cargo flamegraph` or add `tracing` timing spans
   - Search > 100ms: profile `computeSearchResults` — likely an O(n²) string match; consider pre-indexing
   - Scroll < 60fps: check if `@tanstack/virtual` `overscan` is too high; check for layout thrashing in `SpanRow`
   - Memory > 200MB: take a heap snapshot; check for duplicate data between Rust state and JS objects
7. Implement fixes, re-measure, confirm targets met.
8. Write `docs/sprint-5/performance-report.md` with methodology, raw numbers, and any fixes applied.
9. Create `docs/sprint-5/rust-notes/04-performance-notes.md` (Russian, gitignored) explaining: Rust release vs debug builds, `cargo flamegraph`, why memory appears to be 3× the file size, browser DevTools memory profiling.
10. **Pre-commit review:** Present `git diff` (code changes if any) + the performance report, and wait for explicit approval.
11. **After approval:** Commit: `chore(perf): validate all performance targets — see docs/sprint-5/performance-report.md`.
12. Push and create PR linking the GitHub Issue from the header.
