# Architecture — Tempo Trace Viewer

**Version:** 1.0  
**Date:** 2026-05-20  
**Author:** Alexander Nichiporovich  
**Status:** Approved for implementation

---

## 1. System Overview

Tempo Trace Viewer is a Tauri v2 desktop application. The process model has two sides:

- **Rust backend process** — file I/O, JSON parsing, data model, business logic
- **Svelte frontend (WebView)** — all rendering, user interaction, state

They communicate exclusively via Tauri's IPC bridge. The frontend never touches the file system directly.

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Application                     │
│                                                          │
│  ┌──────────────────────┐    ┌────────────────────────┐ │
│  │   Rust Backend        │    │   Svelte Frontend      │ │
│  │                       │    │                        │ │
│  │  • File dialog        │◄──►│  • Trace header        │ │
│  │  • JSON parsing       │IPC │  • Span tree + Gantt   │ │
│  │  • Data model         │    │  • Span detail panel   │ │
│  │  • Log correlation    │    │  • Log panels          │ │
│  │  • Operational logs   │    │  • Search toolbar      │ │
│  └──────────────────────┘    └────────────────────────┘ │
│                                                          │
│  Tauri Capabilities (minimal):                           │
│  • fs:read  — user-selected files via dialog only        │
│  • fs:write — app_log_dir() only                         │
│  • No network, no shell                                  │
└─────────────────────────────────────────────────────────┘
```

---

## 2. Crate and Package Selection

### 2.1 Rust Crates (answers Open Question 1)

| Crate | Version | Purpose | Rationale |
|---|---|---|---|
| `serde` | 1.x | Serialization framework | De facto standard; required by Tauri |
| `serde_json` | 1.x | JSON parsing | Standard; handles OTEL/Tempo JSON format |
| `tracing` | 0.1.x | Structured logging macros | Standard for async Rust; integrates with Tauri |
| `tracing-subscriber` | 0.3.x | Log filtering + formatting | Companion to `tracing`; configures output format |
| `tracing-appender` | 0.2.x | Rolling file output | Handles the 10MB rolling log file requirement |
| `thiserror` | 2.x | Error type derivation | Reduces boilerplate for typed errors in parser modules |

All crates listed above have been published to crates.io for multiple years and meet the 7-day rule by a wide margin.

### 2.2 Frontend Packages

| Package | Purpose |
|---|---|
| `@tanstack/virtual` | Span tree virtualization (variable row height) |
| IBM Plex Sans + IBM Plex Mono | Fonts — bundled locally, no CDN |

---

## 3. Data Model

All types are defined in `src-tauri/src/model/`. They are derived `Serialize`/`Deserialize` so they cross the IPC boundary as JSON.

### 3.1 Trace (input format: OTEL/Tempo `batches[]`)

```rust
// src-tauri/src/model/trace.rs

pub struct Trace {
    pub trace_id: String,
    pub resource_spans: Vec<ResourceSpans>,
}

pub struct ResourceSpans {
    pub resource: Resource,
    pub scope_spans: Vec<ScopeSpans>,
}

pub struct Resource {
    pub attributes: Vec<KeyValue>,
}

pub struct ScopeSpans {
    pub scope: InstrumentationScope,
    pub spans: Vec<Span>,
}

pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub start_time_unix_nano: u64,
    pub end_time_unix_nano: u64,
    pub attributes: Vec<KeyValue>,
    pub status: SpanStatus,
    pub events: Vec<SpanEvent>,
}

pub struct KeyValue {
    pub key: String,
    pub value: AnyValue,
}

pub enum AnyValue {
    StringValue(String),
    BoolValue(bool),
    IntValue(i64),
    DoubleValue(f64),
    ArrayValue(Vec<AnyValue>),
    KvlistValue(Vec<KeyValue>),
}
```

### 3.2 Frontend View Model

The Rust backend builds a flattened, frontend-ready view model before sending over IPC.
The raw `batches[]` nesting is not exposed to the frontend.

```rust
// src-tauri/src/model/view.rs

pub struct TraceView {
    pub trace_id: String,
    pub root_service_name: String,
    pub root_operation_name: String,
    pub start_time_unix_nano: u64,
    pub duration_ns: u64,
    pub service_count: usize,
    pub span_count: usize,
    pub error_count: usize,
    pub spans: Vec<SpanView>,
}

pub struct SpanView {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub service_name: String,
    pub operation_name: String,
    pub start_time_unix_nano: u64,
    pub duration_ns: u64,
    pub relative_start_ns: u64,      // offset from trace start
    pub depth: u32,                   // tree depth for indentation
    pub has_error: bool,
    pub attributes: Vec<KeyValue>,
    pub resource_attributes: Vec<KeyValue>,
    pub events: Vec<SpanEvent>,
    pub logs: Vec<LogEntryView>,      // populated after log correlation
}
```

### 3.3 Log Model (input format: Loki `data.result[]` streams)

```rust
// src-tauri/src/model/log.rs

pub struct LokiQueryResponse {
    pub data: LokiData,
}

pub struct LokiData {
    pub result: Vec<LokiStream>,
}

pub struct LokiStream {
    pub stream: HashMap<String, String>,   // labels (includes span_id if present)
    pub values: Vec<(String, String)>,     // (timestamp_ns, log_line)
}

pub struct LogEntryView {
    pub timestamp_ns: u64,
    pub message: String,
    pub labels: Vec<KeyValue>,
    pub span_id: Option<String>,
}
```

---

## 4. IPC Contract (answers Open Question 3)

Strategy: **single payload per operation**. The full parsed and correlated view model is sent in one `invoke()` call. This meets the < 2s parse + render target for 5,000 spans without the complexity of chunking or streaming.

### 4.1 Tauri Commands

```typescript
// Frontend call signatures (TypeScript)

// Opens native file dialog, parses trace, returns full TraceView
invoke<TraceView>('load_trace')

// Opens native file dialog, parses log file(s), returns updated SpanView[]
// with logs correlated. Merges with any previously loaded logs in backend state.
invoke<LogCorrelationResult>('load_logs')

// Clears all loaded data from backend state
invoke<void>('clear_session')
```

```rust
// Backend command signatures (Rust)

#[tauri::command]
async fn load_trace(app: AppHandle) -> Result<TraceView, String>

#[tauri::command]
async fn load_logs(app: AppHandle, state: State<AppState>) -> Result<LogCorrelationResult, String>

#[tauri::command]
async fn clear_session(state: State<AppState>) -> Result<(), String>
```

### 4.2 Backend State

```rust
// src-tauri/src/state.rs

pub struct AppState {
    pub trace: Mutex<Option<TraceView>>,
    pub log_entries: Mutex<Vec<LogEntryView>>,
}
```

---

## 5. Log Correlation Algorithm

Correlation runs in the Rust backend after each `load_logs` call, before IPC response.

```
for each LogEntry in merged log dataset:
    if entry.labels contains "span_id":
        find SpanView where span_id == entry.labels["span_id"]
        if found: append LogEntryView to span.logs
        if not found: append to unmatched_logs list
    else:
        append to unmatched_logs list

return LogCorrelationResult {
    updated_spans: Vec<SpanView>,  // only spans that received new logs
    unmatched_logs: Vec<LogEntryView>,
}
```

The frontend merges `updated_spans` into its local state and displays `unmatched_logs`
in the Unmatched Logs panel.

---

## 6. Frontend Architecture

### 6.1 Component Hierarchy

```
App.svelte
├── Toolbar.svelte
│   ├── LoadTraceButton.svelte
│   ├── LoadLogsButton.svelte
│   └── SearchBar.svelte
├── TraceHeader.svelte
│   └── UnmatchedLogsPanel.svelte (collapsible)
└── MainLayout.svelte
    ├── SpanTree.svelte                  ← @tanstack/virtual
    │   └── SpanRow.svelte (virtual)
    │       └── GanttBar.svelte
    ├── ResizableDivider.svelte
    └── SpanDetail.svelte
        ├── TagsSection.svelte
        ├── ResourceSection.svelte
        └── LogsSection.svelte
```

### 6.2 State Management

Svelte stores (no external state library required):

```typescript
// src/lib/stores/trace.ts
export const traceView = writable<TraceView | null>(null);
export const selectedSpanId = writable<string | null>(null);
export const searchQuery = writable<string>('');
export const unmatchedLogs = writable<LogEntryView[]>([]);
```

Derived stores compute filtered/highlighted span lists from `traceView` and `searchQuery`
reactively — no manual subscription management in components.

### 6.3 Virtualization Strategy (answers Open Question 2)

`@tanstack/virtual` is used for the span tree list with `measureElement` for variable row heights.
The virtualizer is configured with:
- `overscan: 10` — renders 10 off-screen rows above and below the viewport
- Row height measured on first render per row; cached thereafter
- Scroll container is the `SpanTree` component's root `<div>`

The Gantt timeline bars (`GanttBar.svelte`) are positioned using CSS `left` and `width`
as percentages of the total trace duration — calculated once when the trace loads,
stored in each `SpanView`. This decouples Gantt positioning from the virtualizer's scroll math.

### 6.4 Search Implementation

Search state flows through a single Svelte derived store:

```typescript
// src/lib/stores/search.ts
export const searchResults = derived(
    [traceView, searchQuery],
    ([$trace, $query]) => computeSearchResults($trace, $query)
);
```

`computeSearchResults` runs synchronously on the frontend — no IPC call.
It returns a `Set<string>` of matching span IDs. Components use this set to apply
`data-match` / `data-dimmed` attributes, which CSS targets for highlighting.

Search fields: span name, attribute keys, attribute values, log message text.

---

## 7. Theming Architecture

All design tokens are CSS custom properties in `src/lib/styles/tokens.css`.
Svelte components reference tokens only — never hardcoded values.

```css
/* src/lib/styles/tokens.css */
:root {
  /* Typography */
  --font-sans: 'IBM Plex Sans', system-ui, sans-serif;
  --font-mono: 'IBM Plex Mono', monospace;

  /* Colors — light theme (v1.0) */
  --color-bg: #ffffff;
  --color-surface: #f4f4f4;
  --color-border: #e0e0e0;
  --color-text-primary: #161616;
  --color-text-secondary: #525252;
  --color-accent: #0f62fe;
  --color-error: #da1e28;

  /* Span tree */
  --span-row-height-base: 32px;
  --gantt-bar-height: 16px;
}
```

Dark theme support is deferred to v2.0. Adding it requires only:
1. A `[data-theme="dark"]` selector block in `tokens.css` with overridden values
2. A theme toggle that sets `document.documentElement.dataset.theme`

No structural component changes are needed.

---

## 8. Application Logging

Operational logging (not trace data) uses `tracing` + `tracing-appender`:

```rust
// src-tauri/src/logging.rs

pub fn init_logging(app: &AppHandle) {
    let log_dir = app.path().app_log_dir().expect("log dir unavailable");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(Rotation::NEVER)           // single file; size managed manually
        .max_log_files(1)
        .filename_prefix("tempo-trace-viewer")
        .filename_suffix("log")
        .build(log_dir)
        .expect("failed to create log appender");

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");
}
```

Log level is `INFO` by default. For debug builds, `DEBUG` level is set via a compile-time feature flag.

---

## 9. Performance Strategy

| Target | Strategy |
|---|---|
| < 2s parse + render (5,000 spans) | Single IPC payload; serde_json is ~300MB/s; 5k spans ≈ 5MB → ~17ms parse. Render is virtual (only ~20 DOM nodes visible). |
| < 100ms search keystroke | Search runs on the frontend in a derived store — no IPC. 5,000 spans × simple string match < 5ms in JS. |
| 60fps scroll | @tanstack/virtual renders only visible rows. DOM stays at ~30 nodes regardless of span count. |
| < 200MB memory | Single trace in memory at a time. Rust owns the parsed model; frontend receives a serialized copy. Peak = Rust model + IPC JSON string + JS deserialized object ≈ 3× file size. A 15MB trace file peaks at ~45MB. |

---

## 10. Security

### 10.1 Tauri Capabilities File

```json
// src-tauri/capabilities/default.json
{
  "identifier": "default",
  "description": "Minimal capabilities for Tempo Trace Viewer",
  "platforms": ["macOS", "windows"],
  "permissions": [
    "core:default",
    "dialog:allow-open",
    "fs:allow-read-via-dialog",
    "fs:allow-write-all"
  ]
}
```

`fs:allow-write-all` is scoped in `tauri.conf.json` to `app_log_dir()` only via the scope configuration.
No arbitrary path write is permitted.

### 10.2 Dependency Age Rule

See `AGENT.md` Section 3 for the enforcement process and check scripts.

---

## 11. Project Structure (complete)

```
/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point
│   │   ├── lib.rs               # Command registration
│   │   ├── state.rs             # AppState (Mutex-wrapped session data)
│   │   ├── logging.rs           # tracing + tracing-appender init
│   │   ├── model/
│   │   │   ├── mod.rs
│   │   │   ├── trace.rs         # Raw OTEL/Tempo deserialization types
│   │   │   ├── log.rs           # Raw Loki deserialization types
│   │   │   └── view.rs          # Frontend view model types
│   │   ├── parser/
│   │   │   ├── mod.rs
│   │   │   ├── trace_parser.rs  # Tempo JSON → TraceView
│   │   │   └── log_parser.rs    # Loki JSON → Vec<LogEntryView>
│   │   ├── correlator.rs        # Log-to-span correlation
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── load_trace.rs    # load_trace command
│   │       ├── load_logs.rs     # load_logs command
│   │       └── clear_session.rs
│   ├── capabilities/
│   │   └── default.json
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/
│   ├── app.html
│   ├── App.svelte
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Toolbar.svelte
│   │   │   ├── TraceHeader.svelte
│   │   │   ├── UnmatchedLogsPanel.svelte
│   │   │   ├── MainLayout.svelte
│   │   │   ├── ResizableDivider.svelte
│   │   │   ├── SpanTree.svelte
│   │   │   ├── SpanRow.svelte
│   │   │   ├── GanttBar.svelte
│   │   │   ├── SpanDetail.svelte
│   │   │   ├── TagsSection.svelte
│   │   │   ├── ResourceSection.svelte
│   │   │   └── LogsSection.svelte
│   │   ├── stores/
│   │   │   ├── trace.ts
│   │   │   └── search.ts
│   │   ├── styles/
│   │   │   ├── tokens.css       # All design tokens
│   │   │   └── global.css
│   │   └── types/
│   │       ├── trace.ts         # TypeScript mirrors of Rust view model
│   │       └── log.ts
│   └── assets/
│       └── fonts/               # IBM Plex Sans + IBM Plex Mono (bundled)
├── docs/
│   ├── PRD.md
│   ├── ARCHITECTURE.md          # This document
│   ├── design/
│   ├── examples/
│   └── sprint-N/
│       ├── tasks/               # Task files (git tracked)
│       │   └── NN-task-name.md
│       └── rust-notes/          # Rust learning notes (gitignored)
│           └── NN-task-name-notes.md
├── .github/
│   └── ISSUE_TEMPLATE/
│       ├── task.md
│       └── bug.md
├── AGENT.md                     # LLM instruction file (prerequisite)
└── .gitignore
```

---

## 12. Open Questions Resolved

| # | Question | Decision |
|---|---|---|
| 1 | Rust crates for JSON + logging | `serde_json` + `tracing` + `tracing-appender` + `tracing-subscriber` |
| 2 | Span tree virtualization | `@tanstack/virtual` with `measureElement` for variable row heights |
| 3 | IPC strategy | Single payload — full `TraceView` in one `invoke()` call |
| 4 | AGENT.md initialization timing | Prerequisite — ships in Phase 1 PR before any sprint begins |
