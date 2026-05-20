# Task 01: Live Search

**Sprint:** 5 — Search & Polish  
**Branch:** `feature/sprint-5-task-01-search`  
**GitHub Issue:** <!-- Added in Phase 2 -->  
**Depends on:** Sprint 3 / Task 02 (SpanTree must exist); Sprint 4 / Task 03 (logs in stores)

---

## Objective

Implement live search across the loaded trace and logs. Search runs entirely on the frontend
in a Svelte derived store — no IPC call required.

Search fields (from PRD Section 6.5):
- Span name (operation name)
- Span attribute keys and values
- Log message text (from `span.logs`)

Behavior:
- Triggered on every keystroke (live)
- Non-matching spans are dimmed, not hidden (tree structure preserved)
- Matching spans are highlighted
- Result count shown in toolbar
- Clear button resets search

---

## Acceptance Criteria

- [ ] `SearchBar.svelte` is in `Toolbar.svelte` and binds to `$searchQuery` store
- [ ] `src/lib/stores/search.ts` defines `searchResults` as a derived store returning `Set<string>` of matching span IDs
- [ ] Search matches: span name, any attribute key, any attribute value, any log message in `span.logs`
- [ ] Search is case-insensitive
- [ ] `SpanRow.svelte` applies `data-match="true"` when span ID is in `$searchResults`, `data-dimmed="true"` otherwise (when query is non-empty)
- [ ] CSS in `tokens.css` / `global.css` styles `[data-dimmed="true"]` rows at 30% opacity
- [ ] Toolbar shows result count: "N spans matched" (hidden when query is empty)
- [ ] Clear button (×) in `SearchBar` sets `$searchQuery` to `''`
- [ ] Search response is < 100ms for 5,000 spans (verify in browser DevTools)

---

## Files to Create / Modify

- `src/lib/stores/search.ts` — `searchResults` derived store + `computeSearchResults` function
- `src/lib/components/SearchBar.svelte` — search input with clear button
- `src/lib/components/Toolbar.svelte` — add `SearchBar` and result count
- `src/lib/components/SpanRow.svelte` — add `data-match` / `data-dimmed` attributes
- `src/lib/styles/tokens.css` or `global.css` — dimmed row styles

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: search for a span name, an attribute value, and a log message — all return correct results
- [ ] Manual test: 5,000-span trace — keystroke response < 100ms
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-5-task-01-search`
3. Read `docs/ARCHITECTURE.md` Section 6.4 for the search implementation design.
4. Implement `computeSearchResults` first as a pure function with no Svelte dependencies, so it's easy to reason about correctness.
5. Wire it into a Svelte derived store.
6. Update `SpanRow.svelte` and add CSS for dimmed state.
7. Manual test performance: open DevTools → Performance tab, record while typing in search field with 5,000 spans loaded.
8. Create `docs/sprint-5/rust-notes/01-search-notes.md` (Russian, gitignored) explaining: Svelte derived stores with multiple inputs, `Set<T>` in TypeScript, case-insensitive string matching, browser performance profiling.
9. **Pre-commit review:** Present `git diff`, describe the search algorithm (which fields are searched, how match is determined), confirm < 100ms result, and wait for explicit approval.
10. **After approval:** Commit: `feat(search): implement live span and log search with highlight and dim`.
11. Push and create PR linking the GitHub Issue from the header.
