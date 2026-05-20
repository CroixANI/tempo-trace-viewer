# Session: Architecture & Sprint Planning

**Date:** 2026-05-20  
**Participants:** Alexander Nichiporovich (CEO/Developer), Claude Sonnet 4.6 (Architect)  
**Branch created:** `feature/architecture-and-sprint-planning` → merged via PR #2  
**Branch created:** `feature/phase-2-github-issues` → PR #18  

---

## Context

The CEO had previously created `docs/PRD.md` covering the full product requirements for Tempo Trace Viewer — a cross-platform Tauri v2 desktop app for visualizing OpenTelemetry traces and Loki logs. The PRD left 4 open questions for the Architect and did not include sprint planning.

This session produced the complete architecture document, AGENT.md, GitHub Issue templates, and 13 sprint task files across 5 sprints.

---

## Session Flow

### Phase 1 — Architecture decisions via grilling (`/grill-me`)

The session used the `grill-me` skill. Claude acted as a software architect and resolved all non-trivial decisions one question at a time before writing any documents.

**Decision 1 — Local task files vs GitHub Issues: which is source of truth?**  
Options: A) local file authoritative, B) GitHub Issue authoritative, C) different purposes, no sync.  
**Decision: C** — local file = LLM prompt scaffold; GitHub Issue = human progress tracking. Solo project; keeping them in sync adds overhead with no benefit.

**Decision 2 — GitHub Issue templates: user story + task + bug, or fewer?**  
User stories duplicate the PRD for a solo project.  
**Decision: task + bug only.** User stories stay in the PRD.

**Decision 3 — Sprint model: feature-based or time-boxed?**  
Time-boxing only adds value when coordinating with others. Rust learning curve makes velocity unpredictable.  
**Decision: feature-based sprints.** Each sprint = one coherent deliverable, variable duration.

**Decision 4 — Tauri IPC strategy: single payload, chunked, or streaming?**  
Single payload meets the < 2s target for 5,000 spans without the complexity of chunking.  
**Decision: single payload.** Full `TraceView` returned in one `invoke()` call.

**Decision 5 — GitHub Issue creation workflow (Phase 2 approach)**  
User proposed a two-phase approach: Phase 1 = create all docs + PR + merge; Phase 2 = create GitHub Issues and immediately update task files with Issue URLs.  
**Decision: two-phase workflow adopted.** Phase 2 loops through task files, creates Issues, updates each file with the returned URL in one pass.

**Decision 6 — Span tree virtualization library**  
`svelte-virtual-list` assumes fixed row height. `@tanstack/virtual` supports variable row heights via `measureElement` and is actively maintained.  
**Decision: `@tanstack/virtual`.** Span rows have variable height; TanStack handles this natively.

**Decision 7 — AGENT.md initialization timing**  
AGENT.md contains rules every subsequent task depends on. Creating it as Sprint 1 Task 1 means the first task runs without guardrails.  
**Decision: prerequisite.** Ships in the Phase 1 PR alongside ARCHITECTURE.md, before any sprint begins.

**Decision 8 — Sprint breakdown**  
5 feature-based sprints proposed and confirmed:

| Sprint | Scope | Tasks |
|---|---|---|
| 1 | Project scaffold | 1 |
| 2 | Data model + trace parsing | 3 |
| 3 | Trace visualization | 4 |
| 4 | Log loading + correlation | 3 |
| 5 | Search + polish | 4 |

---

### Phase 1 — Documents created

All changes on branch `feature/architecture-and-sprint-planning`, committed and merged via **PR #2**.

**`AGENT.md`** (project root)  
Single authoritative LLM instruction file covering:
- Workflow rules: sync main → create branch → implement → pre-commit review → wait for user approval → commit → PR
- Pre-commit review protocol: present full diff, summarize changes, wait for explicit written approval before any commit
- Dependency age rule: 7-day minimum for crates.io and npm packages, with check scripts
- Tauri capability guardrails
- Code conventions (Rust + Svelte/TypeScript)
- Rust learning notes instructions (Russian, gitignored)

**`docs/ARCHITECTURE.md`**  
Complete technical architecture resolving all 4 PRD open questions:
- System overview with ASCII process diagram
- Rust data model (raw OTEL types, Loki types, frontend view model)
- IPC contract: `load_trace`, `load_logs`, `clear_session` command signatures
- Log correlation algorithm
- Frontend component hierarchy (`App → Toolbar → TraceHeader → MainLayout → SpanTree/SpanDetail`)
- Svelte store design (`traceView`, `selectedSpanId`, `searchQuery`, `unmatchedLogs`, `searchResults`)
- `@tanstack/virtual` configuration for span tree
- Search implementation via derived store
- CSS custom property theming system with token reference
- Application logging setup (`tracing` + `tracing-appender`)
- Performance strategy table (how each of the 4 targets is met)
- Tauri capability declaration
- Complete project folder structure

**`.github/ISSUE_TEMPLATE/task.md`** — Task issue template  
**`.github/ISSUE_TEMPLATE/bug.md`** — Bug issue template  
**`.github/ISSUE_TEMPLATE/config.yml`** — Blank issues disabled

**`.gitignore`** — Added: `node_modules/`, `dist/`, `.svelte-kit/`, `.env`, `src-tauri/target/`, `docs/sprint-*/rust-notes/`, `.DS_Store`

**13 task files** across `docs/sprint-1/` through `docs/sprint-5/`:

| File | GitHub Issue |
|---|---|
| `sprint-1/tasks/01-init-tauri-svelte-project.md` | #3 |
| `sprint-2/tasks/01-rust-data-model.md` | #4 |
| `sprint-2/tasks/02-trace-parser.md` | #5 |
| `sprint-2/tasks/03-ipc-trace-command.md` | #6 |
| `sprint-3/tasks/01-trace-header.md` | #7 |
| `sprint-3/tasks/02-span-tree.md` | #8 |
| `sprint-3/tasks/03-span-detail.md` | #9 |
| `sprint-3/tasks/04-layout.md` | #10 |
| `sprint-4/tasks/01-log-parser.md` | #11 |
| `sprint-4/tasks/02-log-correlation.md` | #12 |
| `sprint-4/tasks/03-log-ui.md` | #13 |
| `sprint-5/tasks/01-search.md` | #14 |
| `sprint-5/tasks/02-theming.md` | #15 |
| `sprint-5/tasks/03-app-logging.md` | #16 |
| `sprint-5/tasks/04-performance-validation.md` | #17 |

Each task file contains:
- Objective, acceptance criteria, files to create/modify, definition of done
- LLM workflow instructions: branch name, implementation steps, rust-notes requirement, pre-commit review protocol, commit message format, PR creation command with `Closes <ISSUE_URL>`

---

### Phase 2 — GitHub Issues created and task files updated

After PR #2 was merged, a new branch `feature/phase-2-github-issues` was created.

- GitHub labels `task` and `bug` already existed on the repo
- 13 Issues (#3–#17) created via `gh issue create` with full acceptance criteria bodies
- Each task file immediately updated with the returned Issue URL
- All changes committed and PR #18 opened

---

## Key Architectural Decisions Summary

| Topic | Decision | Rationale |
|---|---|---|
| IPC strategy | Single payload | Meets < 2s target; avoids chunking complexity |
| Virtualization | `@tanstack/virtual` | Variable row height support; actively maintained |
| JSON parsing | `serde_json` | De facto standard |
| Logging crates | `tracing` + `tracing-appender` + `tracing-subscriber` | Standard Tauri ecosystem |
| Error types | `thiserror` | Reduces boilerplate |
| State management | Svelte writable/derived stores | No external library needed |
| Search | Frontend-only derived store | No IPC; 5k spans < 5ms in JS |
| Theming | CSS custom properties in `tokens.css` | Dark theme addable without structural changes |
| Issue workflow | Two-phase: docs first, Issues second | Clean separation; backlog only created when work starts |
| Sprint model | Feature-based | Solo project; unpredictable Rust learning velocity |

---

## PRs and Issues

| # | Type | Title |
|---|---|---|
| PR #2 | Pull Request | docs(architecture): add ARCHITECTURE.md, AGENT.md, GitHub templates, and 5-sprint task breakdown |
| PR #18 | Pull Request | chore(sprints): add GitHub Issue URLs to all 13 task files (Phase 2) |
| #3–#17 | Issues | One per sprint task (see table above) |
