# Task 01: Initialize Tauri v2 + Svelte + Vite Project

**Sprint:** 1 — Project Scaffold  
**Branch:** `feature/sprint-1-task-01-init-project`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** None

---

## Objective

Bootstrap the complete project skeleton: Tauri v2 with a Svelte + Vite frontend.
Establish the folder structure from `docs/ARCHITECTURE.md` Section 11, configure minimal
Tauri capabilities, set up fonts, and verify the app launches on both macOS and Windows build targets.

This task produces a running shell — no trace loading, no UI beyond a placeholder screen.
All subsequent sprint tasks build on top of this scaffold.

---

## Acceptance Criteria

- [ ] `cargo tauri dev` launches the app without errors
- [ ] App window opens with a blank Svelte page showing a placeholder title "Tempo Trace Viewer"
- [ ] Tauri capabilities file (`src-tauri/capabilities/default.json`) declares only the permissions listed in `ARCHITECTURE.md` Section 10.1 — nothing more
- [ ] IBM Plex Sans and IBM Plex Mono fonts are present in `src/assets/fonts/` and referenced in `src/lib/styles/global.css` (no CDN)
- [ ] CSS custom properties from `ARCHITECTURE.md` Section 7 are defined in `src/lib/styles/tokens.css`
- [ ] `.gitignore` includes `docs/sprint-*/rust-notes/` and standard Rust/Node ignores
- [ ] `AGENT.md` is present at the project root (already created as a prerequisite — verify only)
- [ ] `pnpm install` completes without errors (or `npm install` if pnpm is unavailable)

---

## Files to Create / Modify

- `src-tauri/Cargo.toml` — add `serde`, `serde_json`, `tracing`, `tracing-subscriber`, `tracing-appender`, `thiserror`
- `src-tauri/capabilities/default.json` — minimal capability declaration
- `src-tauri/tauri.conf.json` — app metadata, window config, bundle identifier
- `src-tauri/src/main.rs` — Tauri entry point
- `src-tauri/src/lib.rs` — command registration stub
- `src/App.svelte` — placeholder UI
- `src/lib/styles/tokens.css` — all design tokens
- `src/lib/styles/global.css` — font-face declarations, resets
- `src/assets/fonts/` — IBM Plex Sans + IBM Plex Mono font files
- `.gitignore` — Rust, Node, and rust-notes ignores
- `package.json` — Svelte + Vite + TypeScript dependencies

---

## Dependency Age Check

Before adding any crate or npm package, run the check scripts from `AGENT.md` Section 3.1
and include the results in your pre-commit review summary.

Key crates to verify: `serde`, `serde_json`, `tracing`, `tracing-subscriber`, `tracing-appender`, `thiserror`.

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Dependency age verified for all new crates and packages
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-1-task-01-init-project`
3. Read `docs/ARCHITECTURE.md` fully before writing any code.
4. Run dependency age checks for all new crates/packages (see `AGENT.md` Section 3.1). Record results.
5. Implement the acceptance criteria above.
6. Create `docs/sprint-1/rust-notes/01-init-project-notes.md` (Russian, gitignored) explaining: Cargo workspaces, Tauri's process model, `main.rs` vs `lib.rs` split, and the Tauri capability model.
7. **Pre-commit review:** Run `git diff --staged` (after staging), present the full diff to the user, summarize changes, confirm rust-notes file was created, and wait for explicit approval.
8. **After approval:** Stage specific files, commit using Conventional Commits format, ending with `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>`.
9. Push branch and create PR: `gh pr create` with title `chore(sprint-1): initialize Tauri v2 + Svelte + Vite project scaffold` and body linking the GitHub Issue URL from the header of this file.
