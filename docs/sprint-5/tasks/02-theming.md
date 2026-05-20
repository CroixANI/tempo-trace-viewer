# Task 02: Theming & Font Bundling

**Sprint:** 5 — Search & Polish  
**Branch:** `feature/sprint-5-task-02-theming`  
**GitHub Issue:** https://github.com/CroixANI/tempo-trace-viewer/issues/15  
**Depends on:** Sprint 1 / Task 01 (fonts directory must exist)

---

## Objective

Audit and finalize the theming system. Ensure every component uses CSS custom properties
from `tokens.css` with no hardcoded values. Verify fonts are bundled correctly and
load without a CDN. This task is a polish and consistency pass — not adding new functionality.

---

## Acceptance Criteria

- [ ] IBM Plex Sans (regular 400, medium 500) and IBM Plex Mono (regular 400) are present as WOFF2 files in `src/assets/fonts/`
- [ ] `@font-face` declarations in `global.css` reference local font files only — no Google Fonts or CDN URLs
- [ ] App loads fonts offline (disconnect from internet, launch app — fonts must render correctly)
- [ ] `grep` across all `.svelte` files finds no hardcoded hex colors, pixel font sizes, or spacing values that are not CSS token references
- [ ] All spacing in components uses token-based values (e.g., `var(--space-2)` not `8px`)
- [ ] `tokens.css` defines a complete spacing scale (4px base: `--space-1: 4px`, `--space-2: 8px`, etc.)
- [ ] No component imports a CSS file other than via Svelte `<style>` blocks or the global stylesheet

---

## Files to Create / Modify

- `src/assets/fonts/` — add any missing WOFF2 font files
- `src/lib/styles/global.css` — audit and fix `@font-face` declarations
- `src/lib/styles/tokens.css` — add spacing scale if missing
- Any `.svelte` component that has hardcoded values — replace with token references

---

## Definition of Done

- [ ] All acceptance criteria checked
- [ ] Manual test: app runs offline with fonts rendering correctly
- [ ] `grep -r "#[0-9a-fA-F]" src/lib/components/` returns no matches
- [ ] Code reviewed and approved by user before commit
- [ ] PR created linking the GitHub Issue from the header above
- [ ] PR merged to `main`

---

## LLM Workflow Instructions

1. **Sync with main:** `git checkout main && git pull origin main`
2. **Create branch:** `git checkout -b feature/sprint-5-task-02-theming`
3. Run `grep -r "#[0-9a-fA-F]\|font-size:\|padding:\|margin:" src/lib/components/` to find hardcoded values.
4. Download IBM Plex Sans and IBM Plex Mono WOFF2 files from the IBM Plex GitHub repository if not already present.
5. Fix all hardcoded values and font references.
6. Run the offline test: disable network in DevTools → reload app → verify fonts render.
7. Create `docs/sprint-5/rust-notes/02-theming-notes.md` (Russian, gitignored) explaining: CSS custom properties (variables), `@font-face` and WOFF2 format, why bundling fonts matters for a desktop offline app.
8. **Pre-commit review:** Present `git diff`, list every component that was changed and what hardcoded value was replaced, and wait for explicit approval.
9. **After approval:** Commit: `chore(ui): finalize theming — replace hardcoded values with CSS tokens, bundle fonts locally`.
10. Push and create PR linking the GitHub Issue from the header.
