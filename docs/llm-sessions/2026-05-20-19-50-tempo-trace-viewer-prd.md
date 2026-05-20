# Session: Tempo Trace Viewer — PRD Requirements Review

**Date:** 2026-05-20  
**Topic:** Requirements stress-test and PRD creation for Tempo Trace Viewer  
**Output:** `docs/PRD.md`, branch `feature/add-prd`, PR #1
**Resumt:** `claude --resume f789b251-5225-4b26-bff4-0e33ea3d5da8`

---

## Context

The user presented a plan to build a cross-platform desktop application called **Tempo Trace Viewer** using Rust and Tauri. The app is intended to fill a gap in the Grafana Tempo ecosystem: Tempo can export traces but cannot import and visualize them offline, and has no per-trace search capability.

The session ran as a structured requirements interview (grill-me), resolving one decision at a time before writing the PRD.

**Initial requirements provided:**

- Build with Rust + Tauri, target macOS and Windows
- Replace Jaeger UI workflow for OTEL/Tempo traces
- Load a trace file + additional Loki log files; correlate logs to spans (like Jaeger)
- Search within a single trace (spans, logs, tags)
- Design reference files in `docs/design/`, example files in `docs/examples/`
- Restrict dependencies to those at least 7 days old (supply-chain safety)
- Best practices captured in `AGENT.md`
- Write app log file using platform-appropriate location
- White theme only for v1.0; dark theme deferred
- Each implementation task to include Russian-language Rust learning notes

---

## Pre-interview Codebase Analysis

Before questioning, the assistant read the example files and found:

- **OTEL trace:** 51 batches, 578 spans, ~82 seconds total duration. Spans have nanosecond timestamps but no embedded log events.
- **Loki logs (`nimbusteamsdriver`):** 63 streams — all 63 have `trace_id` and `span_id` labels.
- **Loki logs (`nimbuscallhandler`):** 26 streams — only 1 out of 26 has `trace_id` / `span_id`. The rest have no span correlation fields.
- **Jaeger trace file** also present in examples (1094 spans, different JSON structure).
- **Design prototype** (`docs/design/variant-a3/`): React app with IBM Plex Sans/Mono fonts, resizable split pane (span tree left, detail panel right), Tags/Process/Logs sections in detail panel.

---

## Requirements Interview — Q&A

---

### Q1: Log-to-span correlation strategy

**Finding:** With 578 overlapping spans in an 82-second window, matching a log purely by timestamp would assign it to dozens of spans simultaneously. Only 1/26 `nimbuscallhandler` streams have `span_id`; all 63 `nimbusteamsdriver` streams do.

**Question:** What should the app do when a log has no `span_id` — match it to all spans whose time window contains the log's timestamp, or show it in a separate panel?

**Recommendation:** Show logs without `span_id` in a dedicated "Unmatched Logs" section at the trace level, not attached to any span. Only attach logs to span detail panels when `span_id` matches exactly.

**Decision:** ✅ Show unmatched logs at trace level.

---

### Q2: Jaeger trace format support

**Finding:** A Jaeger JSON trace file exists in `docs/examples/` (same trace ID as the OTEL file, 1094 spans, completely different structure).

**Question:** Should the app support Jaeger JSON in addition to OTEL/Tempo JSON?

**Recommendation:** Out of scope — OTEL/Tempo JSON only. The team is migrating away from Jaeger; supporting its format adds a second parser with no benefit.

**Decision:** ✅ Jaeger format out of scope.

---

### Q3: Frontend framework

**Question:** Which frontend framework — React (matching design prototype), Svelte, or something else?

**Recommendation:** React with TypeScript — design prototype is already React, direct component reuse.

**Follow-up from user:** What gives the most responsive app — React, Vite, Svelte?

**Clarification provided:** Vite is a build tool, not a framework. Real choice is React vs Svelte (both use Vite). Svelte compiles to vanilla JS with no virtual DOM — smaller bundle (~10KB vs ~130KB), better runtime performance for filtering 578+ spans and live search.

**Decision:** ✅ Svelte + Vite. Design files used as style reference; styles to be ported with LLM assistance.

---

### Q4: Tauri version

**Question:** Tauri v1 or v2?

**Recommendation:** Tauri v2 — current recommended version, cleaner capability/permission model, better security model.

**Decision:** ✅ Tauri v2.

---

### Q5: Search scope

**Question:** Search span names and attributes only, or also log content?

**Recommendation:** All three — span names, span attributes/tags, and log message text.

**Decision:** ✅ Search across span names, span attributes/tags, and log message text.

---

### Q6: Multiple log files per session

**Question:** One log file per session or multiple?

**Recommendation:** Multiple — real-world example already has two log files (one per service). Restricting to one forces manual merging.

**Decision:** ✅ Multiple log files per session, merged and correlated against the single loaded trace.

---

### Q7: Dependency age enforcement

**Question:** Hard CI gate blocking builds if any dependency is < 7 days old, or documented guideline with manual check?

**Recommendation:** Documented guideline + manual check script in `AGENT.md` querying crates.io API. No hard CI gate — too much infrastructure complexity for a small team.

**Decision:** ✅ Documented guideline with manual check script.

---

### Q8: Application log file location

**Question:** Where to write the app's operational log on macOS — `~/Library/Logs/` or `~/Documents/`?

**Recommendation:** `~/Library/Logs/AppName/` on macOS, next to exe on Windows.

**Follow-up from user:** Is there a cross-platform solution, like a temp folder?

**Clarification provided:** Temp folder is a bad fit — OS can clear it anytime. Tauri v2 provides `app_log_dir()` which resolves to the correct platform-native location automatically:
- Windows: `%APPDATA%\{bundle-id}\logs\`
- macOS: `~/Library/Logs/{bundle-id}/`

"Next to the exe" on Windows also causes issues in `Program Files` (requires admin rights).

**Decision:** ✅ Use `app_log_dir()` cross-platform.

---

### Q9: Font loading strategy

**Question:** Bundle fonts locally or load from CDN at runtime?

**Recommendation:** Bundle locally — desktop app may run offline or in air-gapped environments. CDN dependency causes inconsistent rendering on font load failure.

**Decision:** ✅ Fonts bundled locally (IBM Plex Sans + IBM Plex Mono, ~300KB addition to binary).

---

### Q10: Rust learning notes location

**Question:** Embed Russian Rust explanations in task files, or in a separate directory?

**User proposal:** `docs/sprint-N/tasks/NN-task-name.md` for tasks (git tracked) and `docs/sprint-N/rust-notes/NN-task-name-notes.md` for learning notes (gitignored, local only).

**Decision:** ✅ Task files in `docs/sprint-N/tasks/` (tracked). Rust notes in `docs/sprint-N/rust-notes/` (gitignored). Notes filename mirrors task filename with `-notes` suffix.

---

### Q11: AGENT.md scope

**Question:** Purely Rust/Tauri best-practices reference, or the single authoritative agent instruction file?

**Recommendation:** Single authoritative file — best practices, project structure, build commands, dependency check script, and coding conventions all in one place.

**Decision:** ✅ `AGENT.md` is the single authoritative agent instruction file.

---

### Q12: Trace scale ceiling

**Question:** Design for ~1,000 spans (current real-world size) or higher with virtualized rendering?

**Recommendation:** 5,000 spans with virtualized list rendering in the span tree. Production traces grow as systems expand; retrofitting virtualization later is costly.

**Decision:** ✅ 5,000 spans with virtualized rendering.

---

## Final Decision Summary

| # | Topic | Decision |
|---|---|---|
| 1 | Log correlation | `span_id` match → attach to span. No `span_id` → trace-level "Unmatched Logs" panel |
| 2 | Trace format | OTEL/Tempo JSON only. Jaeger out of scope |
| 3 | Frontend | Svelte + Vite. Design prototype as style reference |
| 4 | Tauri version | Tauri v2 |
| 5 | Search scope | Span names, span attributes/tags, log message text |
| 6 | Log files | Multiple per session, merged |
| 7 | Dependency age | Documented guideline + manual crates.io check script in AGENT.md |
| 8 | App logging | `app_log_dir()` cross-platform |
| 9 | Fonts | Bundled locally |
| 10 | Rust notes | `docs/sprint-N/rust-notes/` gitignored, Russian language, mirrors task filenames |
| 11 | AGENT.md | Single authoritative agent instruction file |
| 12 | Scale | 5,000 spans with virtualized rendering |

---

## Outputs

- **`docs/PRD.md`** — Product Requirements Document ready for Architecture review
- **Branch:** `feature/add-prd`
- **PR:** https://github.com/CroixANI/tempo-trace-viewer/pull/1

---

## Open Questions Left for Architect

1. Which Rust crates for JSON parsing and logging? Verify publish dates meet 7-day rule.
2. Svelte virtual list — library or custom implementation?
3. Tauri v2 IPC strategy: full trace payload in one shot or chunked for large files?
4. Should `AGENT.md` be initialized as Sprint 1 Task 1 prerequisite or before any sprint begins?
