# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SubTracker is a terminal-based (TUI) personal subscription tracker for managing household digital subscriptions. It is a **Rust learning project**. The design documents are written in French (`docs/`).

## Build & Development Commands

```bash
cargo build              # Build the project
cargo run                # Run the application
cargo check              # Type-check without building
cargo fmt                # Format code (default rustfmt settings)
cargo clippy             # Lint (default clippy settings)
cargo test               # Run all tests
cargo test test_name     # Run a single test by name
```

Pre-commit hooks run `cargo fmt` and `cargo check` automatically via `.pre-commit-config.yaml`.

## Architecture

Layered architecture with six top-level modules declared in `main.rs`:

| Module | Purpose |
|---|---|
| `models/` | Domain structs/enums (Subscription, Need, FamilyMember, etc.) |
| `db/` | SQLite via `rusqlite` (bundled) — migrations and CRUD queries |
| `services/` | Business logic — duplicate detection, recommendations, projections, catalog, reports |
| `ui/` | TUI screens built with `ratatui` + `crossterm` (one file per screen) |
| `app.rs` | Application state (active screen, running flag) |
| `config.rs` | Configuration and file paths |

Data flow: `ui/` → `services/` → `db/` → SQLite. Models are shared across layers.

## Key Technical Choices

- **Rust edition 2024** — uses the newest stable edition
- **Synchronous** — no async runtime; fully blocking I/O
- **rusqlite with `bundled` feature** — SQLite compiled into the binary, no system dependency
- **serde + toml** — TOML-based catalog and alternatives config files
- **ratatui + crossterm** — terminal UI framework

## Formatting Conventions

- 4-space indentation, UTF-8, LF line endings (`.editorconfig`)
- Rust files: max 100 characters per line
- Other files: max 120 characters per line

## Design Documents

- `docs/subscription-manager-prd.md` — Full PRD with SQLite schema (7 tables), feature list, and planned TOML formats
- `docs/iteration-1.md` — Step-by-step implementation guide for iteration 1 (10 tasks)

## Current State

The project is in early development (iteration 1). Module scaffolding is complete but most files contain only `// TODO` stubs. The `models/subscription.rs` struct is the active work-in-progress.
