# SubTracker

A terminal-based (TUI) personal subscription tracker for managing household digital subscriptions.

## About

SubTracker is a **Rust learning project** built with the help of Claude (AI-assisted coding).
It's designed to teach Rust fundamentals through a practical, real-world application.

## Features

- Track digital subscriptions (streaming, software, gaming, etc.)
- Monthly cost projections
- Duplicate detection
- Recommendations for alternatives
- Family plan management
- Monthly reports

## Tech Stack

- **Rust** (edition 2024) — Programming language
- **rusqlite** (bundled) — SQLite database
- **ratatui + crossterm** — Terminal UI framework
- **chrono** — Date/time handling
- **serde + toml** — Configuration and serialization

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2024)

### Build & Run

```bash
# Build
cargo build

# Run
cargo run

# Run tests
cargo test
```

### Development Commands

| Command | Description |
|---------|-------------|
| `cargo check` | Type-check without building |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint code |
| `cargo test <name>` | Run a single test |

## Architecture

```
src/
├── main.rs      # Entry point
├── app.rs       # Application state
├── config.rs    # Configuration
├── models/      # Domain structs/enums
├── db/          # SQLite database layer
├── services/    # Business logic
└── ui/          # TUI screens (ratatui)
```

## License

MIT
