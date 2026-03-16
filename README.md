# PlayFab CLI

Rust CLI for PlayFab service automation with 200+ tool definitions.

[![CI](https://github.com/akiojin/playfab-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/akiojin/playfab-cli/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/playfab-cli.svg)](https://crates.io/crates/playfab-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Installation

### cargo-binstall (recommended)

```bash
cargo binstall playfab-cli
```

### cargo install

```bash
cargo install playfab-cli
```

### GitHub Releases

Download pre-built binaries from [Releases](https://github.com/akiojin/playfab-cli/releases).

Available targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

## Quick Start

Set environment variables:

```bash
export PLAYFAB_TITLE_ID=your_title_id
export PLAYFAB_DEV_SECRET_KEY=your_secret_key
```

Verify connectivity:

```bash
playfab-cli system ping
```

## Usage

### List available tools

```bash
playfab-cli tool list
```

### Show tool schema

```bash
playfab-cli tool schema search_items
```

### Call a tool

```bash
playfab-cli tool call search_items --json '{"Search":"sword","Count":10}'
```

### Batch operations

```bash
playfab-cli batch --json '[{"tool":"search_items","params":{"Search":"sword"}},{"tool":"get_title_data","params":{}}]'
```

### Configuration

```bash
playfab-cli config show
playfab-cli config set title_id YOUR_TITLE_ID
```

### CLI health check

```bash
playfab-cli cli doctor
```

### Output formats

```bash
# Text output (default)
playfab-cli tool list

# JSON output
playfab-cli --output json tool list
```

## Environment Variables

| Variable | Required | Description |
|---|---|---|
| `PLAYFAB_TITLE_ID` | Yes | PlayFab Title ID |
| `PLAYFAB_DEV_SECRET_KEY` | Yes | PlayFab Developer Secret Key |
| `PLAYFAB_API_ENDPOINT` | No | Custom API endpoint (default: `playfabapi.com`) |

## Tool Categories

The CLI provides 200+ tools across 13 categories:

| Category | Description |
|---|---|
| Economy Catalog | Item catalog search, CRUD, draft/publish management |
| Economy Inventory | Inventory operations, virtual currency, transactions (v2) |
| Admin | Title configuration, player management (~90 operations) |
| Server | Server-side player and game management (~130 operations) |
| Authentication | Entity token authentication and validation |
| Profiles | Entity profile read/write operations |
| Data | Entity object and file storage |
| CloudScript | Cloud function management and execution |
| Events | PlayStream and telemetry event operations |
| Groups | Entity group membership management |
| Progression | Statistics and leaderboard operations |
| Multiplayer | Matchmaking, Server Hosting, Lobby, and Party |
| Experimentation | A/B test experiment management |

## Development

### Prerequisites

- Rust 1.75+ (2021 edition)

### Build

```bash
cargo build
cargo build --release
```

### Quality checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

### Run locally

```bash
cargo run -- tool list
cargo run -- system ping
```

## Architecture

```text
src/
├── main.rs              # Entry point
├── cli.rs               # CLI argument definitions (clap)
├── app/
│   ├── mod.rs           # Application module
│   └── runner.rs        # Command dispatch and execution
├── core/
│   ├── mod.rs           # Core module
│   ├── config.rs        # Configuration management
│   ├── self_update.rs   # Self-update from GitHub Releases
│   └── managed_binaries.rs  # Binary management
├── http/
│   ├── mod.rs           # HTTP module
│   ├── client.rs        # HTTP client with retry
│   ├── auth.rs          # Entity Token authentication
│   └── retry.rs         # Retry tiers (Strict/Standard/Bulk)
└── tooling/
    ├── mod.rs           # Tool registry
    ├── tool_executor.rs # Tool execution engine
    ├── schema_builder.rs # JSON Schema builder
    └── catalog/         # Tool definitions by category
        ├── admin.rs
        ├── server.rs
        ├── economy_catalog.rs
        ├── economy_inventory.rs
        ├── authentication.rs
        ├── profiles.rs
        ├── data.rs
        ├── cloudscript.rs
        ├── events.rs
        ├── groups.rs
        ├── progression.rs
        ├── multiplayer.rs
        └── experimentation.rs
```

## Contributing

1. Check existing issues before creating new ones
2. Follow existing code patterns and style
3. Run all quality checks before submitting PRs
4. Update documentation when adding features

## License

[MIT](LICENSE)
