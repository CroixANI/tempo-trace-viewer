// Sample trace data — fictional microservice trace, roughly 35 spans
// Mimics Grafana Tempo / OTel structure but simplified for display.

window.TRACE = {
  traceId: "7a1f3c9e8b4d2a6f0e1c8b9d4a7e2f3c",
  startedAt: "2026-05-19T14:32:08.412+02:00",
  durationMs: 1247.32,
  servicesCount: 8,
  depth: 7,
  totalSpans: 312,
  // Service color tokens (calm, low chroma, distinguishable)
  services: {
    "edge-gateway":      { hue: 250, label: "edge-gateway" },
    "auth-service":      { hue: 145, label: "auth-service" },
    "order-service":     { hue:  50, label: "order-service" },
    "inventory-service": { hue: 195, label: "inventory-service" },
    "payment-service":   { hue: 100, label: "payment-service" },
    "notification-svc":  { hue: 220, label: "notification-svc" },
    "postgres":          { hue: 290, label: "postgres" },
    "redis":             { hue: 350, label: "redis" },
    "kafka":             { hue:  25, label: "kafka" },
    "stripe-client":     { hue: 320, label: "stripe-client" },
  },
  // Flat list with parentId for tree assembly
  spans: [
    { id: "s001", parentId: null,  service: "edge-gateway",      op: "HTTP POST /api/v2/orders",     kind: "server",   durMs: 1247.32, startMs: 0,      tags: [["http.method","POST"],["http.route","/api/v2/orders"],["http.status_code","201"],["net.peer.ip","10.4.12.8"],["user.id","u_8472"]] },
    { id: "s002", parentId: "s001", service: "edge-gateway",      op: "middleware.cors",              kind: "internal", durMs:    1.84, startMs: 0.4,    tags: [["origin","https://app.example.com"]] },
    { id: "s003", parentId: "s001", service: "auth-service",      op: "POST /verify",                 kind: "client",   durMs:   38.21, startMs: 3.1,    tags: [["http.method","POST"],["http.status_code","200"]] },
    { id: "s004", parentId: "s003", service: "auth-service",      op: "verify-jwt",                   kind: "server",   durMs:   34.10, startMs: 5.0,    tags: [["jwt.alg","RS256"],["jwt.sub","u_8472"]] },
    { id: "s005", parentId: "s004", service: "redis",             op: "GET session:u_8472",           kind: "client",   durMs:    1.20, startMs: 6.1,    tags: [["db.system","redis"],["db.operation","GET"]] },
    { id: "s006", parentId: "s001", service: "order-service",     op: "POST /orders",                 kind: "client",   durMs: 1198.40, startMs: 44.0,
      tags: [["http.method","POST"],["http.status_code","201"],["order.id","ord_98234"],["order.total","129.40"]],
      logs: [
        { tMs:   2.1, fields: [["event","request.received"],["bytes","412"]] },
        { tMs: 1196.4, fields: [["event","request.completed"],["bytes","218"]] },
      ]},
    { id: "s007", parentId: "s006", service: "order-service",     op: "validate-order",               kind: "internal", durMs:    8.22, startMs: 45.0,   tags: [["items.count","3"]] },
    { id: "s008", parentId: "s006", service: "postgres",          op: "SELECT customers WHERE id=$1", kind: "client",   durMs:   42.10, startMs: 54.0,   tags: [["db.system","postgresql"],["db.name","ecom"],["db.statement","SELECT id, email, tier FROM customers WHERE id = $1"]] },
    { id: "s009", parentId: "s006", service: "inventory-service", op: "POST /reserve",                kind: "client",   durMs:  412.30, startMs: 98.0,
      tags: [["http.method","POST"],["http.status_code","200"]],
      logs: [
        { tMs:   1.0, fields: [["event","reserve.start"],["items","3"]] },
        { tMs:  84.2, fields: [["event","stock.checked"]] },
        { tMs: 410.4, fields: [["event","reserve.committed"],["reservation.id","rsv_44a1"]] },
      ]},
    { id: "s010", parentId: "s009", service: "inventory-service", op: "check-stock-levels",           kind: "internal", durMs:   82.40, startMs: 99.0,   tags: [["items.count","3"]] },
    { id: "s011", parentId: "s010", service: "postgres",          op: "SELECT inventory",             kind: "client",   durMs:   78.40, startMs: 100.0,  tags: [["db.system","postgresql"],["db.statement","SELECT sku, qty FROM inventory WHERE sku = ANY($1)"]] },
    { id: "s012", parentId: "s009", service: "postgres",          op: "BEGIN; UPDATE reservations",   kind: "client",   durMs:  298.10, startMs: 184.0,
      tags: [["db.system","postgresql"],["db.statement","BEGIN; INSERT INTO reservations ... ; COMMIT"]],
      logs: [
        { tMs:   0.5, fields: [["event","tx.begin"]] },
        { tMs: 297.6, fields: [["event","tx.commit"],["rows","3"]] },
      ]},
    { id: "s013", parentId: "s009", service: "kafka",             op: "publish inventory.reserved",   kind: "producer", durMs:   12.40, startMs: 495.0,  tags: [["messaging.system","kafka"],["messaging.destination","inventory.reserved"]] },
    { id: "s014", parentId: "s006", service: "payment-service",   op: "POST /charge",                 kind: "client",   durMs:  612.90, startMs: 513.0,
      tags: [["http.method","POST"],["http.status_code","200"],["payment.amount","129.40"]],
      logs: [
        { tMs:   1.0, fields: [["event","charge.requested"]] },
        { tMs: 611.0, fields: [["event","charge.succeeded"],["charge.id","ch_3kP2"]] },
      ]},
    { id: "s015", parentId: "s014", service: "payment-service",   op: "load-customer-profile",        kind: "internal", durMs:   18.40, startMs: 514.0,  tags: [] },
    { id: "s016", parentId: "s015", service: "redis",             op: "GET profile:u_8472",           kind: "client",   durMs:    0.80, startMs: 514.5,  tags: [["db.system","redis"]], error: true,
      logs: [
        { tMs: 0.2, fields: [["event","exception"],["exception.type","ConnectionTimeout"],["exception.message","i/o timeout after 800µs"]] },
      ]},
    { id: "s017", parentId: "s015", service: "postgres",          op: "SELECT customers (fallback)",  kind: "client",   durMs:   14.10, startMs: 516.0,  tags: [["db.system","postgresql"],["fallback","true"]] },
    { id: "s018", parentId: "s014", service: "stripe-client",     op: "POST /v1/charges",             kind: "client",   durMs:  584.10, startMs: 533.0,  tags: [["http.method","POST"],["http.status_code","200"],["http.url","https://api.stripe.com/v1/charges"]] },
    { id: "s019", parentId: "s018", service: "stripe-client",     op: "tls.handshake",                kind: "internal", durMs:   38.20, startMs: 534.0,  tags: [["tls.version","1.3"]] },
    { id: "s020", parentId: "s018", service: "stripe-client",     op: "http.write_request",           kind: "internal", durMs:    4.20, startMs: 573.0,  tags: [] },
    { id: "s021", parentId: "s018", service: "stripe-client",     op: "http.read_response",           kind: "internal", durMs:  540.80, startMs: 577.5,  tags: [] },
    { id: "s022", parentId: "s006", service: "postgres",          op: "INSERT INTO orders",           kind: "client",   durMs:   48.40, startMs: 1128.0, tags: [["db.system","postgresql"],["db.statement","INSERT INTO orders ... RETURNING id"]] },
    { id: "s023", parentId: "s006", service: "notification-svc",  op: "POST /notify",                 kind: "client",   durMs:   68.10, startMs: 1178.0,
      tags: [["http.method","POST"],["http.status_code","202"]],
      logs: [
        { tMs:   1.0, fields: [["event","enqueue.start"],["channel","email,push"]] },
        { tMs:  66.4, fields: [["event","enqueue.done"]] },
      ]},
    { id: "s024", parentId: "s023", service: "notification-svc",  op: "render-template order.confirmed", kind: "internal", durMs: 14.20, startMs: 1180.0, tags: [["template","order.confirmed.v3"]] },
    { id: "s025", parentId: "s023", service: "kafka",             op: "publish notifications.outbound", kind: "producer", durMs:  9.80, startMs: 1196.0, tags: [["messaging.system","kafka"],["messaging.destination","notifications.outbound"]] },
    { id: "s026", parentId: "s023", service: "redis",             op: "INCR rate:u_8472",             kind: "client",   durMs:    0.70, startMs: 1207.0, tags: [["db.system","redis"]] },
    { id: "s027", parentId: "s006", service: "kafka",             op: "publish order.created",        kind: "producer", durMs:   11.20, startMs: 1230.0, tags: [["messaging.system","kafka"]] },
    { id: "s028", parentId: "s001", service: "edge-gateway",      op: "response.encode",              kind: "internal", durMs:    2.10, startMs: 1244.0, tags: [["encoding","json"]] },
  ],
  // Mark spans that have errors
  errors: ["s016"],
};

// Helper: format duration as h:mm:ss.ms or compact ms/µs
window.fmtDuration = function (ms) {
  if (ms < 1) return (ms * 1000).toFixed(0) + "µs";
  if (ms < 1000) return ms.toFixed(2) + "ms";
  const s = ms / 1000;
  if (s < 60) return s.toFixed(2) + "s";
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = (s % 60).toFixed(2);
  return `${String(h).padStart(2,"0")}:${String(m).padStart(2,"0")}:${sec.padStart(5,"0")}`;
};

// Helper: format start time as local time
window.fmtStart = function (iso, opts = {}) {
  const d = new Date(iso);
  const pad = (n, w = 2) => String(n).padStart(w, "0");
  const date = `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())}`;
  const time = `${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}.${pad(d.getMilliseconds(),3)}`;
  return opts.dateOnly ? date : `${date} ${time}`;
};

// Build a tree from flat spans
window.buildTree = function (spans) {
  const map = new Map(spans.map(s => [s.id, { ...s, children: [] }]));
  const roots = [];
  for (const s of map.values()) {
    if (s.parentId && map.has(s.parentId)) map.get(s.parentId).children.push(s);
    else roots.push(s);
  }
  return roots;
};
