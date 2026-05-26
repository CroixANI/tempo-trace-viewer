import { spawn, type ChildProcess } from 'child_process';
import { resolve } from 'path';
import type { Options } from '@wdio/types';

// Path to the debug build produced by `cargo tauri build --debug`
const APPLICATION = resolve(
  import.meta.dirname,
  'src-tauri/target/debug/bundle/macos/Tempo Trace Viewer.app'
);

let tauriDriver: ChildProcess;

export const config: Options.Testrunner = {
  specs: ['./tests/e2e/**/*.e2e.ts'],
  maxInstances: 1,

  // Connect to the tauri-driver WebDriver server (started in onPrepare)
  hostname: 'localhost',
  port: 4444,
  path: '/',

  capabilities: [
    {
      maxInstances: 1,
      browserName: 'wry',
      'tauri:options': {
        application: APPLICATION,
      },
    } as WebdriverIO.Capabilities,
  ],

  logLevel: 'error',
  bail: 0,

  waitforTimeout: 10_000,
  connectionRetryTimeout: 120_000,
  connectionRetryCount: 3,

  framework: 'mocha',
  reporters: [['spec', { addConsoleLogs: true }]],
  mochaOpts: {
    ui: 'bdd',
    timeout: 60_000,
  },

  // Start tauri-driver before the test session
  onPrepare() {
    tauriDriver = spawn('tauri-driver', [], {
      stdio: [null, process.stdout, process.stderr],
    });
  },

  // Tear down tauri-driver after the session ends
  onComplete() {
    tauriDriver?.kill();
  },
};
