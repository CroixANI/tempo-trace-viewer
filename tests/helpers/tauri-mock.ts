import type { Page } from '@playwright/test';
import type { TraceView } from '../../src/lib/types/trace';

/**
 * Injects a minimal window.__TAURI_INTERNALS__ implementation before page scripts run.
 * Matches the interface expected by @tauri-apps/api/core invoke().
 * Must be called before page.goto().
 */
export async function injectTauriMock(page: Page): Promise<void> {
  await page.addInitScript(() => {
    const callbacks = new Map<number, (data: unknown) => void>();

    window.__TAURI_INTERNALS__ = {
      transformCallback(callback: (data: unknown) => void, once = false) {
        const id = window.crypto.getRandomValues(new Uint32Array(1))[0];
        callbacks.set(id, once
          ? (data) => { callbacks.delete(id); callback(data); }
          : callback);
        return id;
      },
      unregisterCallback(id: number) {
        callbacks.delete(id);
      },
      runCallback(id: number, data: unknown) {
        callbacks.get(id)?.(data);
      },
      callbacks,
      invoke: async (cmd: string, args: unknown) => {
        const handlers = (window as unknown as { __TEST_IPC__: Record<string, (a: unknown) => unknown> }).__TEST_IPC__;
        const handler = handlers?.[cmd];
        if (handler) return handler(args);
        throw new Error(`Unmocked IPC command: ${cmd}`);
      },
    } as unknown as typeof window.__TAURI_INTERNALS__;
  });
}

/**
 * Registers a mock response for a Tauri IPC command.
 * Must be called after page.goto() but before the action that triggers the IPC call.
 */
export async function mockCommand(
  page: Page,
  cmd: string,
  response: TraceView | null
): Promise<void> {
  await page.evaluate(
    ({ cmd, response }) => {
      const w = window as unknown as { __TEST_IPC__: Record<string, () => unknown> };
      w.__TEST_IPC__ = w.__TEST_IPC__ ?? {};
      w.__TEST_IPC__[cmd] = () => response;
    },
    { cmd, response }
  );
}
