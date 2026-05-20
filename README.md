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

## Documentation

- **[PRD](docs/PRD.md)** — problem statement, features, performance targets
- **[Architecture](docs/ARCHITECTURE.md)** — technical design, data model, IPC contract, crate choices
- **[AGENT.md](AGENT.md)** — instructions for LLM-assisted development workflow

---

## Security

Pre-commit hooks automatically run `cargo audit` and `pnpm audit` when dependency files are staged. See [AGENT.md](AGENT.md) for the full dependency age policy (7-day rule for new crates and npm packages).
