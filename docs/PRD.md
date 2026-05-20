# Product Requirements Document — Tempo Trace Viewer

**Version:** 1.0  
**Date:** 2026-05-20  
**Author:** Alexander Nichiporovich  
**Status:** Draft — ready for Architecture review

---

## 1. Purpose

Tempo Trace Viewer is a cross-platform desktop application for loading, visualizing, and searching OpenTelemetry (OTEL) traces exported from Grafana Tempo, with correlated log display from Grafana Loki exports. It fills a gap in the Grafana Tempo ecosystem: Tempo can export traces but cannot import and visualize them offline, and has no per-trace search capability.

The application is also a learning vehicle for Rust. Every implementation task includes a companion Rust learning notes file (Russian language) explaining the Rust concepts used and why.

---

## 2. Background and Problem Statement

The team is migrating from OpenTracing (Jaeger) to OpenTelemetry (Grafana Tempo). Under the old stack, Jaeger UI allowed importing a trace JSON file locally and searching within it. Grafana Tempo lacks both capabilities:

- No offline import/visualization of exported trace files
- No search within a single trace (span names, tags, log content)
- No correlated log display within span details

This tool replicates the Jaeger offline workflow for OTEL traces, adding Loki log correlation.

---

## 3. Target Platforms

| Platform | Build target |
|---|---|
| macOS | `.app` bundle (Apple Silicon + Intel via universal binary) |
| Windows | `.exe` installer |

---

## 4. Technology Stack

| Layer | Choice | Rationale |
|---|---|---|
| Application shell | Tauri v2 | Modern capability model, cross-platform, active development |
| Backend logic | Rust | Performance, memory safety, team learning goal |
| Frontend framework | Svelte + Vite | No virtual DOM overhead, fast rendering for large span trees |
| UI fonts | IBM Plex Sans + IBM Plex Mono | Matches design prototype; bundled locally (no CDN dependency) |
| Theming | White theme (v1.0); dark theme deferred | Focus on core functionality first |

---

## 5. Supported File Formats

### 5.1 Trace Files
- **Format:** OTEL/Tempo JSON (`batches[]` structure as exported by Grafana Tempo)
- **Jaeger JSON:** Explicitly out of scope

### 5.2 Log Files
- **Format:** Grafana Loki JSON query response (`data.result[]` streams structure)
- **Loading:** Multiple log files per session supported; all loaded files are merged and correlated against the single active trace

---

## 6. Core Features

### 6.1 Trace Loading
- User opens a single OTEL/Tempo JSON trace file via native file picker
- Application parses and displays the trace immediately
- A session holds exactly one trace at a time; opening a new trace replaces the current one

### 6.2 Log File Loading
- User can load one or more Loki JSON log files after (or before) loading a trace
- All loaded log files are merged into a single log dataset in memory
- Log files can be added incrementally without reloading the trace

### 6.3 Trace Visualization
- **Span tree** (left panel): hierarchical tree of spans with Gantt-style timeline bars
  - Displays: service name, operation name, duration, relative start offset
  - Color-coded by service (consistent hue per service name)
  - Collapsible subtrees
  - Virtualized rendering to support up to 5,000 spans without performance degradation
- **Span detail** (right panel): shown when a span is selected
  - Sections: Tags, Process/Resource attributes, Logs
  - Logs section shows only logs whose `span_id` label matches the selected span's ID
- **Resizable divider** between tree and detail panels
- **Trace header**: service name, operation name, trace ID, start time, total duration, service count, span count, error count

### 6.4 Log Correlation

Log-to-span matching follows a two-tier strategy:

| Log has `span_id` label? | Behavior |
|---|---|
| Yes | Log is attached to the matching span's detail panel under the Logs section |
| No | Log is shown in a trace-level "Unmatched Logs" panel, not attached to any span |

The "Unmatched Logs" panel is accessible from the trace header area and shows all logs that could not be matched to a specific span.

### 6.5 Search
Search operates within the currently loaded trace and its associated logs.

**Searchable fields:**
- Span name (operation name)
- Span attributes / tags (key and value)
- Log message text (the log line value)

**Search behavior:**
- Triggered on keystroke (live filtering)
- Matching spans are highlighted in the span tree; non-matching spans are dimmed (not hidden, to preserve tree structure context)
- Search results count shown in toolbar
- Clear button resets search state

### 6.6 Theming
- **v1.0:** White (light) theme only
- **Deferred:** Dark theme — architecture should accommodate future theme switching without requiring structural changes (CSS custom properties / Svelte theme tokens from the start)

---

## 7. Application Logging

- The application writes its own operational log (errors, warnings, lifecycle events) using Tauri's `app_log_dir()` API
- Log location resolves per platform:
  - **Windows:** `%APPDATA%\{bundle-id}\logs\`
  - **macOS:** `~/Library/Logs/{bundle-id}/`
- Log rotation: single rolling file, max 10MB
- Log level configurable (default: `INFO`)

---

## 8. Performance Requirements

| Metric | Target |
|---|---|
| Trace file parse + render (5,000 spans) | < 2 seconds on modern hardware |
| Search keystroke response | < 100ms |
| Scroll/pan in span tree | 60fps |
| Memory ceiling (5,000 spans + logs) | < 200MB |

---

## 9. Security Requirements

### 9.1 Supply-Chain Attack Mitigation
- All Rust (`cargo`) dependencies must have been published to crates.io at least **7 days** before being added to the project
- This is enforced as a documented process guideline in `AGENT.md`
- `AGENT.md` includes a manual check script that queries the crates.io API for a crate's publish date before it is added to `Cargo.toml`
- The same guideline applies to npm/pnpm frontend dependencies

### 9.2 Tauri Capability Model
- Tauri v2 capabilities are declared explicitly and minimally:
  - File system read: user-selected files only (via dialog, no arbitrary path access)
  - File system write: `app_log_dir()` only
  - No network access required at runtime
  - No shell execution

---

## 10. Project Structure

```
/
├── src-tauri/          # Rust backend (Tauri v2)
├── src/                # Svelte + Vite frontend
├── docs/
│   ├── PRD.md          # This document
│   ├── ARCHITECTURE.md # To be created by Architect
│   ├── design/         # UI design prototype (reference only)
│   ├── examples/       # Example trace and log files
│   └── sprint-N/
│       ├── tasks/      # Task files (git tracked)
│       │   └── NN-task-name.md
│       └── rust-notes/ # Rust learning notes in Russian (gitignored)
│           └── NN-task-name-notes.md
├── AGENT.md            # Single authoritative agent instruction file
└── .gitignore          # Includes docs/sprint-*/rust-notes/
```

---

## 11. Task and Documentation Conventions

### 11.1 Task Files (`docs/sprint-N/tasks/NN-task-name.md`)
Each task file contains:
- Objective and acceptance criteria
- Files to create or modify
- Dependencies on other tasks
- Definition of done

### 11.2 Rust Learning Notes (`docs/sprint-N/rust-notes/NN-task-name-notes.md`)
- Written in Russian
- Explains every Rust concept used in the corresponding task
- Written for a reader with no prior Rust experience
- Covers: what the concept is, why it exists in Rust, and why it was chosen for this task
- Gitignored — personal to the developer, not committed to the repository

---

## 12. Out of Scope (v1.0)

- Jaeger JSON trace format support
- Dark theme
- Live connection to Grafana Tempo API (file import only)
- Trace comparison (diff between two traces)
- Export/sharing of traces from within the app
- Multiple simultaneous traces
- Authentication or user accounts
- Mobile platforms

---

## 13. Open Questions for Architect

1. Which Rust crates to use for JSON parsing (`serde_json`) and logging (`tracing` + `tracing-appender`)? Verify publish dates meet the 7-day rule before finalising.
2. Should span tree virtualization be implemented via a Svelte virtual list library or a custom implementation? Evaluate available Svelte virtualization crates/packages.
3. Tauri v2 IPC strategy: should the full parsed trace be sent to the frontend as one payload, or streamed in chunks for large files?
4. Should `AGENT.md` be initialised as part of Sprint 1 Task 1, or as a prerequisite before any sprint begins?
