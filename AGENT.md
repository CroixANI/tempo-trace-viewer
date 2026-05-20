# AGENT.md — Tempo Trace Viewer

This is the single authoritative instruction file for any LLM agent working in this repository.
Read this file completely before starting any task.

---

## 1. Project Context

Tempo Trace Viewer is a cross-platform desktop application (Tauri v2 + Rust backend + Svelte frontend)
for loading, visualizing, and searching OpenTelemetry traces exported from Grafana Tempo,
with correlated log display from Grafana Loki exports.

The project is also a Rust learning vehicle. Every implementation task has a companion
rust-notes file (Russian language, gitignored) explaining the Rust concepts used.

Key documents:
- `docs/PRD.md` — product requirements, features, performance targets
- `docs/ARCHITECTURE.md` — technical architecture, data model, IPC contract, crate choices

---

## 2. Workflow Rules

Every task file contains specific workflow instructions. Follow them exactly.
The general workflow for every task is:

### 2.1 Branch

1. Sync with main: `git checkout main && git pull origin main`
2. Create a task branch: `git checkout -b feature/sprint-N-task-NN-short-description`
   - Use the branch name specified in the task file exactly.

### 2.2 Implementation

- Implement only what the task's acceptance criteria require. Do not add unrequested features,
  refactors, or abstractions.
- Do not add comments that explain what the code does — only add a comment when the WHY
  is non-obvious (a hidden constraint, a workaround, a subtle invariant).
- Do not add error handling for scenarios that cannot happen. Trust framework guarantees.

### 2.3 Pre-commit Review

**Before creating any commit:**
1. Run `git diff` and present the full diff to the user.
2. Summarize what changed and why each change was necessary.
3. Wait for the user's explicit written approval ("approved", "looks good", "go ahead", or similar).
4. Do not commit until approval is received. If the user requests changes, make them and repeat.

### 2.4 Commit

After approval:
1. Stage specific files by name — never `git add .` or `git add -A`.
2. Write a commit message in Conventional Commits format:
   - `feat(scope): description` for new functionality
   - `fix(scope): description` for bug fixes
   - `chore(scope): description` for scaffolding, config, tooling
   - `docs(scope): description` for documentation
3. End every commit message with:
   ```
   Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
   ```
4. Never skip hooks (`--no-verify`). If a hook fails, fix the root cause.

### 2.5 Pull Request

After committing:
1. Push the branch: `git push -u origin feature/sprint-N-task-NN-short-description`
2. Create a PR using the `gh` CLI:
   ```
   gh pr create \
     --title "feat(sprint-N): task title" \
     --body "$(cat <<'EOF'
   ## Summary
   - bullet points describing what was implemented

   ## Acceptance criteria
   - [ ] criterion 1
   - [ ] criterion 2

   ## Related
   Closes <GITHUB_ISSUE_URL>

   🤖 Generated with [Claude Code](https://claude.com/claude-code)
   EOF
   )"
   ```
3. The `<GITHUB_ISSUE_URL>` is found in the `GitHub Issue` field at the top of the task file.

---

## 3. Security Rules

### 3.1 Dependency Age Rule (Supply-Chain Protection)

**Before adding any new dependency to `Cargo.toml` or `package.json`:**

For Rust crates, verify the publish date via the crates.io API:
```bash
# Replace CRATE_NAME with the actual crate name
curl -s "https://crates.io/api/v1/crates/CRATE_NAME/versions" \
  | python3 -c "
import sys, json
versions = json.load(sys.stdin)['versions']
latest = versions[0]
print(f\"Crate: {latest['num']}\")
print(f\"Published: {latest['created_at']}\")
"
```

For npm/pnpm packages, verify via the npm registry:
```bash
# Replace PACKAGE_NAME with the actual package name
curl -s "https://registry.npmjs.org/PACKAGE_NAME" \
  | python3 -c "
import sys, json
data = json.load(sys.stdin)
time = data.get('time', {})
versions = [v for v in time if v not in ('created', 'modified')]
if versions:
    latest = sorted(versions)[-1]
    print(f'Package: {latest}')
    print(f'Published: {time[latest]}')
"
```

**Rule:** The version you intend to use must have been published at least **7 days** before today's date.
If it was published more recently, do not add it. Find an older stable version or a different crate.

Document the check result in your pre-commit review summary.

### 3.2 Tauri Capabilities

Do not expand Tauri capabilities beyond what is declared in `src-tauri/capabilities/`.
The permitted capabilities are:
- File system **read**: user-selected files only (opened via dialog)
- File system **write**: `app_log_dir()` only
- No network access
- No shell execution

If a feature requires a new capability, flag it to the user before implementing.

---

## 4. Code Conventions

### 4.1 Rust
- Use `serde_json` for all JSON parsing — no alternative JSON crates.
- Use `tracing` macros (`tracing::info!`, `tracing::error!`, etc.) for all operational logging.
- Derive `Debug`, `Serialize`, `Deserialize` on all data model structs.
- Prefer `thiserror` for error types in library-style modules.

### 4.2 Svelte / TypeScript
- All components in `src/lib/components/`.
- Use CSS custom properties (defined in `src/lib/styles/tokens.css`) for all colors, spacing,
  and typography — never hardcode design values.
- Use `@tanstack/virtual` for any list that may exceed 100 items.
- TypeScript strict mode is enabled — no `any` types.

### 4.3 File Naming
- Rust: `snake_case` for files and modules.
- Svelte components: `PascalCase.svelte`.
- TypeScript modules: `camelCase.ts`.

---

## 5. Rust Learning Notes

Every task that involves Rust implementation must have a companion notes file at:
`docs/sprint-N/rust-notes/NN-task-name-notes.md`

This file is:
- Written in **Russian**
- Gitignored (personal to the developer, not committed)
- Explains every Rust concept introduced in the task: what it is, why it exists in Rust,
  and why it was chosen for this specific task
- Written for a reader with no prior Rust experience

Create the notes file as part of implementing the task, before the pre-commit review.
The notes file is not shown in the diff review (it is gitignored) but its existence
should be confirmed in the review summary.
