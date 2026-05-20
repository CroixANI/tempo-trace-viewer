// Shared UI helpers, icons, badges for all three trace-viewer variants.
// Lives on `window.TV` to avoid name collisions across Babel <script> tags.

const TV = {};

// -- color helpers --------------------------------------------------------
TV.serviceColor = function (hue, kind = "bar") {
  // calm, low-saturation palette suitable for a clean light theme
  switch (kind) {
    case "bar":   return `oklch(0.62 0.13 ${hue})`;
    case "dot":   return `oklch(0.58 0.14 ${hue})`;
    case "soft":  return `oklch(0.95 0.04 ${hue})`;
    case "softer":return `oklch(0.97 0.025 ${hue})`;
    case "text":  return `oklch(0.42 0.10 ${hue})`;
    case "border":return `oklch(0.86 0.05 ${hue})`;
    default:      return `oklch(0.6 0.12 ${hue})`;
  }
};

// -- icons (16x16) --------------------------------------------------------
TV.Icon = function ({ name, size = 16, color = "currentColor", stroke = 1.5 }) {
  const p = { width: size, height: size, viewBox: "0 0 16 16", fill: "none",
              stroke: color, strokeWidth: stroke, strokeLinecap: "round", strokeLinejoin: "round" };
  switch (name) {
    case "search":  return <svg {...p}><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5L14 14"/></svg>;
    case "chev-right":  return <svg {...p}><path d="M6 3l5 5-5 5"/></svg>;
    case "chev-down":   return <svg {...p}><path d="M3 6l5 5 5-5"/></svg>;
    case "chev-up":     return <svg {...p}><path d="M3 10l5-5 5 5"/></svg>;
    case "expand":      return <svg {...p}><path d="M3 7l5-4 5 4M3 9l5 4 5-4"/></svg>;
    case "collapse":    return <svg {...p}><path d="M3 9l5-4 5 4"/></svg>;
    case "x":           return <svg {...p}><path d="M3 3l10 10M13 3L3 13"/></svg>;
    case "copy":        return <svg {...p}><rect x="5" y="5" width="8" height="8" rx="1.5"/><path d="M11 5V3.5A1.5 1.5 0 0 0 9.5 2H3.5A1.5 1.5 0 0 0 2 3.5v6A1.5 1.5 0 0 0 3.5 11H5"/></svg>;
    case "error":       return <svg {...p}><circle cx="8" cy="8" r="6"/><path d="M8 5v3.5M8 10.5v.5"/></svg>;
    case "file":        return <svg {...p}><path d="M3 2.5A.5.5 0 0 1 3.5 2H9l4 4v7.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5z"/><path d="M9 2v4h4"/></svg>;
    case "folder":      return <svg {...p}><path d="M2 4.5A1.5 1.5 0 0 1 3.5 3h2.879a1 1 0 0 1 .707.293L8.5 4.5h4A1.5 1.5 0 0 1 14 6v6a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 12z"/></svg>;
    case "filter":      return <svg {...p}><path d="M2 3h12l-4.5 6v4l-3 1V9z"/></svg>;
    case "tag":         return <svg {...p}><path d="M2 3h5.5a1 1 0 0 1 .7.3l5.5 5.5a1 1 0 0 1 0 1.4l-3.5 3.5a1 1 0 0 1-1.4 0L3.3 8.2A1 1 0 0 1 3 7.5V3z"/><circle cx="5.5" cy="5.5" r=".7" fill={color}/></svg>;
    case "list":        return <svg {...p}><path d="M5 4h9M5 8h9M5 12h9M2 4h.01M2 8h.01M2 12h.01"/></svg>;
    case "moon":        return <svg {...p}><path d="M13 9.5A5 5 0 1 1 6.5 3a4 4 0 0 0 6.5 6.5z"/></svg>;
    case "settings":    return <svg {...p}><circle cx="8" cy="8" r="2"/><path d="M8 1v2M8 13v2M3.5 3.5l1.4 1.4M11.1 11.1l1.4 1.4M1 8h2M13 8h2M3.5 12.5l1.4-1.4M11.1 4.9l1.4-1.4"/></svg>;
    case "external":    return <svg {...p}><path d="M9 2h5v5M14 2L7 9M12 9v4.5A.5.5 0 0 1 11.5 14h-9a.5.5 0 0 1-.5-.5v-9A.5.5 0 0 1 2.5 4H7"/></svg>;
    case "circle":      return <svg {...p}><circle cx="8" cy="8" r="3.5"/></svg>;
    case "dot":         return <svg {...p}><circle cx="8" cy="8" r="3" fill={color}/></svg>;
    case "arrow-up":    return <svg {...p}><path d="M8 13V3M4 7l4-4 4 4"/></svg>;
    case "arrow-down":  return <svg {...p}><path d="M8 3v10M4 9l4 4 4-4"/></svg>;
    case "win-min":     return <svg {...p} strokeWidth="1.2"><path d="M3 8h10"/></svg>;
    case "win-max":     return <svg {...p} strokeWidth="1.2"><rect x="3" y="3" width="10" height="10"/></svg>;
    case "win-close":   return <svg {...p} strokeWidth="1.2"><path d="M3 3l10 10M13 3L3 13"/></svg>;
    default: return null;
  }
};

// -- span kind glyph --------------------------------------------------
TV.kindLabel = { server: "SRV", client: "CLI", internal: "INT", producer: "PRD", consumer: "CON" };
TV.kindColor = { server: 250, client: 195, internal: 280, producer: 25, consumer: 145 };
TV.KindBadge = function ({ kind, muted }) {
  const hue = TV.kindColor[kind] ?? 220;
  return (
    <span style={{
      display:"inline-flex", alignItems:"center", justifyContent:"center",
      fontFamily:'"IBM Plex Mono", ui-monospace, monospace',
      fontSize: 9, fontWeight: 600, letterSpacing: ".06em",
      padding: "1px 5px", borderRadius: 3,
      color: muted ? "#78716C" : TV.serviceColor(hue,"text"),
      background: muted ? "#F5F5F4" : TV.serviceColor(hue,"soft"),
      border: muted ? "1px solid #E7E5E4" : `1px solid ${TV.serviceColor(hue,"border")}`,
    }}>{TV.kindLabel[kind] || kind.toUpperCase().slice(0,3)}</span>
  );
};

// -- Windows-like title bar (simple, light) ---------------------------
TV.WinChrome = function ({ filename = "trace_4737e2c.json", accent = "#3F3F46", actions = null }) {
  return (
    <div style={{
      height: 36, display:"flex", alignItems:"center", justifyContent:"space-between",
      background:"#F5F5F4", borderBottom:"1px solid #E7E5E4",
      fontFamily:'"IBM Plex Sans", system-ui, sans-serif',
      userSelect:"none", flex:"0 0 36px",
    }}>
      <div style={{ display:"flex", alignItems:"center", gap:10, padding:"0 12px", color:"#52525B", fontSize:12.5 }}>
        <div style={{
          width:14, height:14, borderRadius:3,
          background: accent,
          display:"flex", alignItems:"center", justifyContent:"center",
          color:"#FAFAF9", fontSize:9, fontWeight:700, fontFamily:'"IBM Plex Mono", monospace',
        }}>T</div>
        <span style={{ fontWeight: 500 }}>Trace Viewer</span>
        <span style={{ color:"#A8A29E" }}>—</span>
        <span style={{ fontFamily:'"IBM Plex Mono", monospace', fontSize:11.5, color:"#71717A" }}>{filename}</span>
      </div>
      <div style={{ display:"flex", alignItems:"center", height:"100%" }}>
        {actions}
        {["win-min","win-max","win-close"].map((n, i) => (
          <button key={n} style={{
            width:42, height:36, display:"grid", placeItems:"center",
            border:0, background:"transparent",
            color: n==="win-close" ? "#52525B" : "#71717A", cursor:"pointer",
          }}>
            <TV.Icon name={n} size={12}/>
          </button>
        ))}
      </div>
    </div>
  );
};

// -- format relative event timestamp ----------------------------------
TV.fmtRelative = function (ms) {
  if (ms < 1) return (ms * 1000).toFixed(0) + "µs";
  if (ms < 1000) return ms.toFixed(2) + "ms";
  return (ms / 1000).toFixed(3) + "s";
};

window.TV = TV;
