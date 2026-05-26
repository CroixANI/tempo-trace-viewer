import { resolve } from 'path';

// Absolute path to the fixed example trace; the Rust backend reads it from disk.
const TRACE_FILE = resolve(
  import.meta.dirname,
  '../../docs/examples/traces-and-logs/otel-trace-01003a80-5371-49a3-93fa-cb471d6e2c6d.json'
);

// Known-good values derived from the example file (verified by the Rust unit test in trace_parser.rs).
const EXPECTED = {
  rootService: 'nimbusteamsdriver',
  rootOperation: 'New Conversation',
  spanCount: '578',
  errorCount: '17',
  serviceCount: '21',
} as const;

async function loadTrace() {
  // Call the debug-only IPC command that bypasses the file dialog.
  // The command is only registered in debug builds (cfg(debug_assertions)).
  await browser.executeAsync(function (path: string, done: (r: unknown) => void) {
    window.__TAURI_INTERNALS__
      .invoke('load_trace_from_path', { path })
      .then(done)
      .catch(done);
  }, TRACE_FILE);
}

describe('TraceHeader — E2E with real trace file', () => {
  before(async () => {
    await loadTrace();
  });

  it('shows the root service name from the parsed trace', async () => {
    const el = await $('[data-testid="root-service"]');
    await el.waitForDisplayed();
    expect(await el.getText()).toBe(EXPECTED.rootService);
  });

  it('shows the root operation name from the parsed trace', async () => {
    const el = await $('[data-testid="root-operation"]');
    expect(await el.getText()).toBe(EXPECTED.rootOperation);
  });

  it('shows the correct span count', async () => {
    const el = await $('[data-testid="span-count"]');
    expect(await el.getText()).toBe(EXPECTED.spanCount);
  });

  it('shows the correct error count in error color', async () => {
    const el = await $('[data-testid="error-count"]');
    expect(await el.getText()).toBe(EXPECTED.errorCount);
    expect(await el.getAttribute('class')).toContain('error-nonzero');
  });

  it('shows the correct service count', async () => {
    const el = await $('[data-testid="service-count"]');
    expect(await el.getText()).toBe(EXPECTED.serviceCount);
  });

  it('shows a non-empty trace ID', async () => {
    const el = await $('[data-testid="trace-id"]');
    const text = await el.getText();
    expect(text.length).toBeGreaterThan(0);
  });

  it('shows a duration formatted in ms or s', async () => {
    const el = await $('[data-testid="duration"]');
    const text = await el.getText();
    expect(text).toMatch(/^\d+\.\d{2}(ms|s)$/);
  });
});
