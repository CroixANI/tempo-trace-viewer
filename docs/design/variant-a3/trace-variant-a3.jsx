// Variant A v3 — A v2's compact aesthetic + a roomier hero header
// (inspired by Variant B): big service title with operation on the same
// line, trace id, and a generous stat row beneath. Toolbar, column
// header, tree rows, split pane, colored detail band and collapsible
// log entries are all kept exactly as in A v2.
//
// All top-level helpers are VA3-prefixed so they can't collide with the
// other variants' helpers in the shared global scope.

const VA3 = {};

VA3.Row = function ({ span, depth, expanded, hasChildren, onToggle, onSelect, selected, matched }) {
  const svc = window.TRACE.services[span.service] || { hue: 220 };
  const indent = 14 + depth * 18;
  return (
    <div
      onClick={() => onSelect(span.id)}
      style={{
        position:"relative", height: 28,
        display:"flex", alignItems:"center", gap: 8,
        paddingLeft: indent, paddingRight: 12,
        background: selected ? "#F4F4F5" : (matched ? "rgba(252, 211, 77, 0.10)" : "transparent"),
        borderLeft: `2px solid ${selected ? TV.serviceColor(svc.hue,"bar") : "transparent"}`,
        borderBottom: "1px solid #F4F4F5",
        cursor: "pointer",
        fontFamily: '"IBM Plex Mono", ui-monospace, monospace',
        fontSize: 12.5,
        color: "#1C1917",
      }}
    >
      <button onClick={(e)=>{ e.stopPropagation(); onToggle(span.id); }}
        style={{
          width:16, height:16, padding:0, border:0, background:"transparent",
          display:"grid", placeItems:"center", color:"#A8A29E",
          cursor: hasChildren ? "pointer" : "default", visibility: hasChildren ? "visible":"hidden",
        }}>
        <TV.Icon name={expanded?"chev-down":"chev-right"} size={12} />
      </button>
      <span style={{
        width: 2, height: 14, borderRadius: 1,
        background: TV.serviceColor(svc.hue,"bar"),
      }}/>
      {span.error ? (
        <span title="error" style={{ display:"inline-flex", color:"oklch(0.55 0.18 25)" }}>
          <TV.Icon name="error" size={11} />
        </span>
      ) : null}
      <span style={{ color:"#27272A", fontWeight: 500, minWidth: 130 }}>{span.service}</span>
      <span style={{
        flex: "1 1 auto", overflow:"hidden", textOverflow:"ellipsis", whiteSpace:"nowrap",
        color: matched ? "#92400E" : "#52525B",
      }}>{va3RenderMatched(span.op, matched)}</span>
      <TV.KindBadge kind={span.kind} muted />
      <span style={{ minWidth: 72, textAlign:"right", color:"#71717A", fontVariantNumeric:"tabular-nums" }}>
        {window.fmtDuration(span.durMs)}
      </span>
    </div>
  );
};

function va3RenderMatched(text, q) {
  if (!q || typeof q !== "string") return text;
  const i = text.toLowerCase().indexOf(q.toLowerCase());
  if (i < 0) return text;
  return (<>
    {text.slice(0,i)}
    <mark style={{ background:"#FEF3C7", color:"#78350F", padding:"0 1px", borderRadius:2 }}>{text.slice(i, i+q.length)}</mark>
    {text.slice(i+q.length)}
  </>);
}

VA3.Tree = function ({ spans, expanded, selected, onToggle, onSelect, query }) {
  const out = [];
  const walk = (s, depth) => {
    const matched = query && (s.op.toLowerCase().includes(query.toLowerCase()) || s.service.toLowerCase().includes(query.toLowerCase())) ? query : null;
    out.push(<VA3.Row key={s.id} span={s} depth={depth}
      expanded={expanded.has(s.id)} hasChildren={s.children.length>0}
      onToggle={onToggle} onSelect={onSelect}
      selected={selected===s.id} matched={matched}/>);
    if (expanded.has(s.id)) s.children.forEach(c => walk(c, depth+1));
  };
  spans.forEach(s => walk(s, 0));
  return out;
};

// ---- Resizable splitter ---------------------------------------------
VA3.useSplit = function (initial = 460, min = 320, max = 900) {
  const [width, setWidth] = React.useState(initial);
  const dragRef = React.useRef(null);

  const onMouseDown = React.useCallback((e) => {
    e.preventDefault();
    const startX = e.clientX;
    const startW = width;
    const container = dragRef.current?.parentElement;
    const containerW = container?.getBoundingClientRect().width ?? 1200;
    const ceiling = Math.min(max, containerW - 360);
    const onMove = (ev) => {
      const dx = ev.clientX - startX;
      const next = Math.min(ceiling, Math.max(min, startW - dx));
      setWidth(next);
    };
    const onUp = () => {
      document.removeEventListener("mousemove", onMove);
      document.removeEventListener("mouseup", onUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
    document.addEventListener("mousemove", onMove);
    document.addEventListener("mouseup", onUp);
  }, [width, min, max]);

  const Divider = (
    <div ref={dragRef} onMouseDown={onMouseDown}
      style={{
        flex:"0 0 6px", width:6, cursor:"col-resize",
        background:"#FFFFFF",
        borderLeft:"1px solid #E7E5E4",
        borderRight:"1px solid #E7E5E4",
        position:"relative",
      }}
      title="Drag to resize"
    >
      <span style={{
        position:"absolute", left:1, top:"50%", transform:"translateY(-50%)",
        width:2, height:36, borderRadius:2, background:"#E7E5E4",
      }}/>
    </div>
  );

  return { width, Divider };
};

// ---- Detail panel ---------------------------------------------------
VA3.Detail = function ({ span, onClose }) {
  if (!span) return (
    <div style={{
      height:"100%", display:"grid", placeItems:"center",
      color:"#A8A29E", fontSize:12.5, fontFamily:'"IBM Plex Sans", system-ui, sans-serif',
    }}>
      <div style={{ textAlign:"center" }}>
        <TV.Icon name="list" size={20} color="#D6D3D1"/>
        <div style={{ marginTop:8 }}>Select a span to inspect.</div>
      </div>
    </div>
  );
  const svc = window.TRACE.services[span.service] || { hue: 220 };

  return (
    <div style={{
      height:"100%", display:"flex", flexDirection:"column",
      background:"#FFFFFF", overflow:"hidden",
      fontFamily:'"IBM Plex Sans", system-ui, sans-serif',
    }}>
      {/* Colored header band */}
      <div style={{
        padding:"16px 18px 14px",
        background: TV.serviceColor(svc.hue, "softer"),
        borderBottom: `1px solid ${TV.serviceColor(svc.hue, "border")}`,
      }}>
        <div style={{ display:"flex", alignItems:"center", gap:8 }}>
          <span style={{ width:9, height:9, borderRadius:9,
            background: TV.serviceColor(svc.hue,"dot"), flex:"0 0 9px" }}/>
          <span style={{ fontSize:13, fontWeight:600, color:"#1C1917" }}>{span.service}</span>
          <TV.KindBadge kind={span.kind}/>
          {span.error ? (
            <span style={{
              display:"inline-flex", alignItems:"center", gap:4,
              padding:"1px 6px", borderRadius:3,
              background:"oklch(0.96 0.04 25)", color:"oklch(0.45 0.16 25)",
              fontSize:10, fontWeight:600, letterSpacing:".05em", textTransform:"uppercase",
            }}>error</span>
          ) : null}
          <span style={{ flex:1 }}/>
          <button onClick={onClose} style={{
            width:22, height:22, border:0, background:"transparent",
            display:"grid", placeItems:"center", color:"#71717A", cursor:"pointer", borderRadius:4,
          }}><TV.Icon name="x" size={12}/></button>
        </div>
        <div style={{
          marginTop:8,
          fontFamily:'"IBM Plex Mono", monospace',
          fontSize: 15, lineHeight: 1.4, fontWeight: 500,
          color:"#18181B", wordBreak:"break-all",
        }}>{span.op}</div>
        <div style={{
          marginTop:8, display:"flex", flexWrap:"wrap", gap:"4px 14px",
          fontFamily:'"IBM Plex Mono", monospace', fontSize:11, color:"#71717A",
        }}>
          <span><span style={{color:"#A1A1AA"}}>span </span>{span.id}</span>
          <span><span style={{color:"#A1A1AA"}}>trace </span>{window.TRACE.traceId.slice(0,16)}…</span>
          <button style={va3MiniLink}><TV.Icon name="copy" size={10}/> copy id</button>
        </div>
      </div>

      {/* Meta grid */}
      <div style={{ padding:"12px 18px", borderBottom:"1px solid #F4F4F5",
        display:"grid", gridTemplateColumns:"1fr 1fr", gap:"10px 16px" }}>
        <VA3Meta label="Started"  mono>{window.fmtStart(new Date(new Date(window.TRACE.startedAt).getTime() + span.startMs).toISOString())}</VA3Meta>
        <VA3Meta label="Duration" mono>{window.fmtDuration(span.durMs)}</VA3Meta>
        <VA3Meta label="Rel. start" mono>{window.fmtDuration(span.startMs)}</VA3Meta>
        <VA3Meta label="Kind" mono>{span.kind}</VA3Meta>
      </div>

      {/* Body: Tags → Process → Logs */}
      <div style={{ flex:1, overflow:"auto" }}>
        <VA3Section title="Tags" count={span.tags.length}>
          <VA3KVTable rows={span.tags}/>
        </VA3Section>
        <VA3Section title="Process">
          <VA3KVTable rows={[
            ["service.name", span.service],
            ["host.name", `${span.service}-7d4f9c-9831`],
            ["environment", "production"],
            ["otel.library.name", `${span.service}/otelhttp`],
            ["otel.library.version", "1.27.0"],
          ]}/>
        </VA3Section>
        <VA3Section title="Logs" count={(span.logs||[]).length} last>
          {span.logs && span.logs.length ? (
            <VA3Logs logs={span.logs} hue={svc.hue}/>
          ) : (
            <div style={{ color:"#A8A29E", fontSize:12, fontFamily:'"IBM Plex Sans", sans-serif' }}>
              No events recorded for this span.
            </div>
          )}
        </VA3Section>
      </div>
    </div>
  );
};

// ---- Collapsible log entries ---------------------------------------
function VA3Logs({ logs, hue }) {
  return (
    <div style={{ display:"flex", flexDirection:"column", gap:6 }}>
      {logs.map((log, i) => <VA3LogItem key={i} log={log} hue={hue}/>) }
    </div>
  );
}
function VA3LogItem({ log, hue }) {
  const [open, setOpen] = React.useState(false);
  const first = log.fields[0] || ["",""];
  const previewKey = first[0];
  const previewVal = String(first[1]);
  return (
    <div style={{
      border:"1px solid #F4F4F5", borderRadius:5,
      background: open ? "#FFFFFF" : "#FAFAF9",
      overflow:"hidden",
    }}>
      <button onClick={()=>setOpen(o=>!o)} style={{
        width:"100%", padding: "8px 10px",
        border:0, background:"transparent", cursor:"pointer",
        display:"flex", alignItems:"center", gap:8,
        textAlign:"left",
      }}>
        <span style={{
          width:14, height:14, display:"grid", placeItems:"center",
          color:"#A8A29E", flex:"0 0 14px",
        }}>
          <TV.Icon name={open?"chev-down":"chev-right"} size={11}/>
        </span>
        <span style={{
          width:6, height:6, borderRadius:6, flex:"0 0 6px",
          background: TV.serviceColor(hue,"dot"),
        }}/>
        <span style={{
          fontFamily:'"IBM Plex Mono", monospace', fontSize:11.5,
          color:"#71717A", flex:"0 0 64px", textAlign:"right",
          fontVariantNumeric:"tabular-nums",
        }}>+{TV.fmtRelative(log.tMs)}</span>
        {!open && (
          <span style={{
            flex:"1 1 auto", minWidth:0,
            display:"flex", alignItems:"baseline", gap:6,
            overflow:"hidden", whiteSpace:"nowrap", textOverflow:"ellipsis",
            fontFamily:'"IBM Plex Mono", monospace', fontSize:11.5,
          }}>
            <span style={{ color:"#71717A" }}>{previewKey}</span>
            <span style={{
              color:"#1C1917", overflow:"hidden", textOverflow:"ellipsis", whiteSpace:"nowrap",
            }}>{previewVal}</span>
            {log.fields.length > 1 && (
              <span style={{ color:"#A8A29E" }}>+{log.fields.length - 1} more</span>
            )}
          </span>
        )}
        {open && (
          <span style={{
            flex:"1 1 auto", color:"#A1A1AA",
            fontFamily:'"IBM Plex Mono", monospace', fontSize:11,
          }}>{log.fields.length} field{log.fields.length===1?"":"s"}</span>
        )}
      </button>
      {open && (
        <div style={{ padding:"4px 10px 12px 42px", borderTop:"1px solid #F4F4F5" }}>
          <VA3KVTable rows={log.fields}/>
        </div>
      )}
    </div>
  );
}

const va3MiniLink = {
  display:"inline-flex", alignItems:"center", gap:4,
  padding:"1px 6px", borderRadius:3,
  border:"1px solid #E7E5E4", background:"#FFFFFF",
  color:"#52525B", cursor:"pointer",
  fontFamily:'"IBM Plex Mono", monospace', fontSize:10.5,
};

function VA3Section({ title, count, children, last }) {
  return (
    <div style={{ borderBottom: last ? "none" : "1px solid #F4F4F5" }}>
      <div style={{
        padding:"12px 18px 8px", display:"flex", alignItems:"center", gap:6,
        fontSize:10.5, letterSpacing:".06em", textTransform:"uppercase",
        color:"#52525B", fontWeight:600,
      }}>
        <span>{title}</span>
        {count!=null && <span style={{ color:"#A8A29E" }}>· {count}</span>}
      </div>
      <div style={{ padding:"0 18px 14px" }}>{children}</div>
    </div>
  );
}
function VA3Meta({ label, children, mono }) {
  return (
    <div>
      <div style={{ color:"#A1A1AA", fontSize:10.5, marginBottom:2, letterSpacing:".04em", textTransform:"uppercase", fontWeight:600 }}>{label}</div>
      <div style={{ fontFamily: mono?'"IBM Plex Mono", monospace':'inherit', fontSize:12.5, color:"#27272A", fontVariantNumeric:"tabular-nums" }}>{children}</div>
    </div>
  );
}
function VA3KVTable({ rows }) {
  if (!rows || rows.length === 0) return <div style={{color:"#A8A29E", fontSize:12, fontFamily:'"IBM Plex Sans", sans-serif'}}>—</div>;
  return (
    <div style={{ display:"grid", gridTemplateColumns:"minmax(130px, 32%) 1fr", rowGap:5,
      fontFamily:'"IBM Plex Mono", monospace', fontSize: 11.5 }}>
      {rows.map(([k,v], i) => (
        <React.Fragment key={i}>
          <div style={{ color:"#71717A", paddingRight:10 }}>{k}</div>
          <div style={{ color:"#1C1917", wordBreak:"break-all" }}>{String(v)}</div>
        </React.Fragment>
      ))}
    </div>
  );
}

VA3.App = function () {
  const tree = React.useMemo(() => window.buildTree(window.TRACE.spans), []);
  const [expanded, setExpanded] = React.useState(() => new Set(window.TRACE.spans.map(s => s.id)));
  const [selected, setSelected] = React.useState("s006");
  const [query, setQuery] = React.useState("");
  const [tagFilter, setTagFilter] = React.useState("");
  const onToggle = (id) => setExpanded(s => { const n = new Set(s); n.has(id)?n.delete(id):n.add(id); return n; });

  const matches = React.useMemo(() => {
    if (!query) return [];
    const q = query.toLowerCase();
    return window.TRACE.spans.filter(s => s.op.toLowerCase().includes(q) || s.service.toLowerCase().includes(q)).map(s => s.id);
  }, [query]);

  const selectedSpan = window.TRACE.spans.find(s => s.id === selected);
  const split = VA3.useSplit(480, 340, 900);

  return (
    <div style={{
      height:"100%", display:"flex", flexDirection:"column",
      background:"#FAFAF9", color:"#1C1917",
      fontFamily:'"IBM Plex Sans", system-ui, sans-serif',
      overflow:"hidden", position:"relative",
    }}>
      <TV.WinChrome filename="orders.api.trace_7a1f3c9e.json" accent="#3F3F46" />

      {/* Roomy hero header — A v2 palette + IBM Plex, B's generous proportions */}
      <div style={{
        padding:"20px 24px 18px",
        borderBottom:"1px solid #E7E5E4", background:"#FFFFFF",
      }}>
        <div style={{ display:"flex", alignItems:"baseline", gap:14, flexWrap:"wrap" }}>
          <span style={{ fontSize:20, fontWeight:600, color:"#1C1917", letterSpacing:"-0.005em" }}>{tree[0]?.service}</span>
          <span style={{ fontFamily:'"IBM Plex Mono", monospace', fontSize:15, color:"#52525B" }}>{tree[0]?.op}</span>
          <span style={{ fontFamily:'"IBM Plex Mono", monospace', fontSize:11.5, color:"#A1A1AA", marginLeft:4 }}>trace {window.TRACE.traceId.slice(0,16)}…</span>
        </div>
        <div style={{ marginTop:14, display:"flex", gap:36, flexWrap:"wrap" }}>
          <VA3Stat label="Started"   mono>{window.fmtStart(window.TRACE.startedAt)}</VA3Stat>
          <VA3Stat label="Duration"  mono>{window.fmtDuration(window.TRACE.durationMs)}</VA3Stat>
          <VA3Stat label="Services" >{window.TRACE.servicesCount}</VA3Stat>
          <VA3Stat label="Depth"    >{window.TRACE.depth}</VA3Stat>
          <VA3Stat label="Spans"    >{window.TRACE.totalSpans}</VA3Stat>
          <VA3Stat label="Errors"   ><span style={{color:"oklch(0.55 0.18 25)"}}>1</span></VA3Stat>
        </div>
      </div>

      <div style={{
        display:"flex", alignItems:"center", gap:10,
        padding:"10px 14px",
        borderBottom:"1px solid #E7E5E4", background:"#FFFFFF",
      }}>
        <div style={{
          flex:"0 1 360px", display:"flex", alignItems:"center", gap:8,
          height:30, padding:"0 10px",
          border:"1px solid #E7E5E4", borderRadius:6, background:"#FFFFFF",
        }}>
          <TV.Icon name="search" size={13} color="#A1A1AA"/>
          <input value={query} onChange={(e)=>setQuery(e.target.value)}
            placeholder="Search service or operation…"
            style={{ flex:1, border:0, outline:"none", background:"transparent",
              fontFamily:'"IBM Plex Mono", monospace', fontSize:12.5, color:"#1C1917" }}/>
          {query ? <span style={{ fontFamily:'"IBM Plex Mono",monospace', fontSize:11, color:"#71717A" }}>{matches.length} match</span> : null}
        </div>
        <div style={{
          display:"flex", alignItems:"center", gap:4,
          height:30, padding:"0 8px 0 6px",
          border:"1px solid #E7E5E4", borderRadius:6, background:"#FFFFFF",
        }}>
          <TV.Icon name="tag" size={12} color="#A1A1AA"/>
          <input value={tagFilter} onChange={(e)=>setTagFilter(e.target.value)}
            placeholder="key=value  or  key=~regex"
            style={{ width:220, border:0, outline:"none", background:"transparent",
              fontFamily:'"IBM Plex Mono", monospace', fontSize:11.5, color:"#1C1917" }}/>
        </div>
        <div style={{ flex:1 }}/>
        <div style={{ display:"flex", gap:4 }}>
          <VA3ToolBtn icon="arrow-up"/>
          <VA3ToolBtn icon="arrow-down"/>
        </div>
        <div style={{ width:1, height:18, background:"#E7E5E4" }}/>
        <div style={{ display:"flex", gap:4 }}>
          <VA3ToolBtn icon="expand" label="Expand" onClick={()=>setExpanded(new Set(window.TRACE.spans.map(s=>s.id)))}/>
          <VA3ToolBtn icon="collapse" label="Collapse" onClick={()=>setExpanded(new Set([tree[0].id]))}/>
        </div>
      </div>

      <div style={{
        display:"flex", alignItems:"center", gap:8,
        padding:"6px 14px 6px 36px",
        borderBottom:"1px solid #E7E5E4", background:"#FAFAF9",
        fontSize:10.5, letterSpacing:".06em", textTransform:"uppercase",
        color:"#A1A1AA", fontWeight:600,
      }}>
        <span style={{ minWidth: 130 }}>Service</span>
        <span style={{ flex:1 }}>Operation</span>
        <span style={{ width: 42, textAlign:"center" }}>Kind</span>
        <span style={{ minWidth: 72, textAlign:"right" }}>Duration</span>
      </div>

      <div style={{ flex:1, display:"flex", overflow:"hidden", background:"#FFFFFF" }}>
        <div style={{ flex:"1 1 auto", minWidth:0, overflow:"auto" }}>
          <VA3.Tree spans={tree} expanded={expanded} selected={selected}
            onToggle={onToggle} onSelect={setSelected} query={query}/>
        </div>
        {selectedSpan && split.Divider}
        {selectedSpan && (
          <div style={{ flex:`0 0 ${split.width}px`, width: split.width, overflow:"hidden" }}>
            <VA3.Detail span={selectedSpan} onClose={()=>setSelected(null)}/>
          </div>
        )}
      </div>

      <div style={{
        height:24, flex:"0 0 24px",
        display:"flex", alignItems:"center", justifyContent:"space-between",
        padding:"0 14px",
        background:"#F5F5F4", borderTop:"1px solid #E7E5E4",
        fontSize: 11, color:"#78716C",
        fontFamily:'"IBM Plex Mono", monospace',
      }}>
        <span>{window.TRACE.totalSpans} spans · {window.TRACE.servicesCount} services · 1 error</span>
        <span>compact · split · light</span>
      </div>
    </div>
  );
};

function VA3ToolBtn({ icon, label, onClick }) {
  return (
    <button onClick={onClick} style={{
      height:26, padding: label ? "0 8px 0 6px" : "0 6px",
      display:"inline-flex", alignItems:"center", gap:6,
      border:"1px solid #E7E5E4", borderRadius:5, background:"#FFFFFF",
      color:"#52525B", fontSize:11.5, cursor:"pointer",
      fontFamily:'"IBM Plex Sans", system-ui, sans-serif',
    }}>
      <TV.Icon name={icon} size={12}/>
      {label ? <span>{label}</span> : null}
    </button>
  );
}
function VA3Stat({ label, children, mono }) {
  return (
    <div style={{ display:"flex", flexDirection:"column", lineHeight:1.15 }}>
      <span style={{ fontSize:10.5, color:"#A1A1AA", letterSpacing:".05em", textTransform:"uppercase", fontWeight:600 }}>{label}</span>
      <span style={{ fontSize:14, color:"#27272A", marginTop:4,
        fontFamily: mono?'"IBM Plex Mono", monospace':'inherit',
        fontVariantNumeric:"tabular-nums" }}>{children}</span>
    </div>
  );
}

window.VariantA3 = VA3.App;
