# Trace Viewer — Variant A v3

A self-contained design prototype for a distributed-trace viewer. Single-page React app rendered via Babel-in-the-browser — no build step required.

## Run it

Any static server works. From this folder:

```bash
python3 -m http.server 8000
# or
npx serve .
```

Then open http://localhost:8000.

Opening `index.html` directly via `file://` will fail because browsers block ES script loading from disk; use a local server.

## Files

| File | Role |
|---|---|
| `index.html` | Page shell. Loads React 18 + Babel standalone from CDN, then the three scripts below. Renders `<VariantA3/>` inside a shadowed card. |
| `trace-data.js` | Mock trace fixture (`window.TRACE`) plus helpers `window.buildTree`, `window.fmtDuration`, `window.fmtStart`. Plain JS — no JSX. |
| `trace-shared.jsx` | Shared design primitives exposed on `window.TV`: `serviceColor`, `Icon` (inline SVG glyphs), `KindBadge`, `WinChrome` (window title bar), `fmtRelative`. |
| `trace-variant-a3.jsx` | The variant itself. All helpers are `VA3`-prefixed. Exports `window.VariantA3`. |

## Design notes

- **Layout (top-to-bottom):** window chrome → roomy hero header (service · operation · trace id, then Started / Duration / Services / Depth / Spans / Errors stats) → search & tag-filter toolbar → column header strip → resizable split pane (span tree on the left, detail panel on the right).
- **Resizable divider:** drag the 6px gutter between tree and detail; min/max clamped by `VA3.useSplit`.
- **Detail panel:** colored band tinted by the selected span's service hue; sections render in order Tags → Process → Logs. Logs are individually collapsible — collapsed state shows relative time + first field one-liner + `+N more`.
- **Type:** IBM Plex Sans for UI, IBM Plex Mono for code/identifiers/numbers.
- **Palette:** neutral stone/zinc background with subtle per-service hue accents via `TV.serviceColor(hue, kind)`.

## Wiring

`trace-shared.jsx` must load before `trace-variant-a3.jsx` (the variant reads `window.TV`). `trace-data.js` is plain JS and can load in `<head>` before the JSX scripts.

If you port this into a real React project:

1. Convert the three scripts to ES modules and replace the `window.*` globals with imports.
2. Drop the `<script type="text/babel">` tags from `index.html` — your bundler handles JSX.
3. Keep `TV.serviceColor` as-is; it returns OKLCH strings that work in any modern browser.
