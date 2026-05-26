# Tempo Trace Viewer

A cross-platform desktop application for loading, visualizing, and searching OpenTelemetry traces exported from Grafana Tempo, with correlated log display from Grafana Loki exports.

Built with Tauri v2 (Rust backend + Svelte frontend). Also a personal Rust learning project — every implementation task includes companion notes in Russian explaining the Rust concepts used.

---

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust + Cargo | 1.80+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 22 LTS | via [nvm](https://github.com/nvm-sh/nvm): `nvm install --lts` |
| pnpm | 11+ | `curl -fsSL https://get.pnpm.io/install.sh \| sh -` |
| Tauri CLI | 2.x | `cargo install tauri-cli --version "^2"` |
| Xcode CLI tools | latest | `xcode-select --install` (macOS only) |

---

## Getting Started

```bash
# 1. Clone the repository
git clone git@github.com:CroixANI/tempo-trace-viewer.git
cd tempo-trace-viewer

# 2. Activate pre-commit security hooks (one-time, per clone)
git config core.hooksPath .githooks

# 3. Install frontend dependencies
pnpm install

# 4. Start the development app
cargo tauri dev
```

---

## Project Structure

```
├── src-tauri/        # Rust backend (Tauri v2)
├── src/              # Svelte + Vite frontend
├── docs/
│   ├── PRD.md        # Product requirements
│   ├── ARCHITECTURE.md  # Technical architecture
│   └── sprint-N/    # Sprint task files
├── .githooks/        # Pre-commit security hooks
└── AGENT.md          # LLM agent instructions
```

---

## Testing

### Rust unit tests

Run the parser and model unit tests. No app build or browser required.

```bash
cargo test
```

All 22 tests in `src-tauri/src/parser/trace_parser.rs` exercise the parser in isolation using
inline JSON fixtures and the real example trace file from `docs/examples/`.

---

### Frontend component tests (Playwright)

Run the Svelte component tests against the Vite dev server with Tauri IPC mocked.
No compiled Tauri binary required.

```bash
pnpm test
```

The Vite dev server starts automatically on port 1420. Tests are in `tests/trace-header.spec.ts`.

---

### Full E2E tests (WebdriverIO + tauri-driver)

Run the end-to-end tests against the real compiled Tauri app using `tauri-driver`. These tests
exercise the Rust parser through real IPC with the example trace file.

> **Platform note:** `tauri-driver` only supports **Linux and Windows**. It does not run on macOS
> (the binary exits immediately on that platform). Run these tests on Linux (e.g. Ubuntu in CI) or
> Windows.

**One-time setup (Linux/Windows):**

```bash
# 1. Install tauri-driver
cargo install tauri-driver

# 2. Build the debug app bundle
cargo tauri build --debug
```

**Run the tests:**

```bash
pnpm test:e2e
```

Tests are in `tests/e2e/trace-header.e2e.ts`.
The debug command `load_trace_from_path` (only registered in debug builds) is used to bypass
the native file dialog and load the example trace directly from disk.

---

## Documentation

- **[PRD](docs/PRD.md)** — problem statement, features, performance targets
- **[Architecture](docs/ARCHITECTURE.md)** — technical design, data model, IPC contract, crate choices
- **[AGENT.md](AGENT.md)** — instructions for LLM-assisted development workflow

---

## Security

Pre-commit hooks automatically run `cargo audit` and `pnpm audit` when dependency files are staged. See [AGENT.md](AGENT.md) for the full dependency age policy (7-day rule for new crates and npm packages).
