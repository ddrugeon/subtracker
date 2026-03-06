# AGENTS.md

This file provides guidance for agentic coding agents working in this repository.

## Project Overview

SubTracker is a terminal-based (TUI) personal subscription tracker for managing household
digital subscriptions. It is a Rust learning project using Rust edition 2024.

## Build & Development Commands

```bash
cargo build              # Build the project
cargo run                # Run the application
cargo check              # Type-check without building (fast, no compilation)
cargo fmt                # Format code (default rustfmt settings)
cargo clippy             # Lint with default clippy settings
cargo clippy -- -D warnings  # Treat warnings as errors
cargo test               # Run all tests
cargo test <test_name>   # Run a single test by name (partial match)
cargo test --test-threads=1 # Run tests sequentially (for debugging)
```

### Pre-commit Hooks

Pre-commit hooks run `cargo fmt` and `cargo check` automatically via `.pre-commit-config.yaml`.
Install pre-commit hooks with: `pre-commit install`

## Architecture

Layered architecture with six top-level modules in `main.rs`:

| Module    | Purpose                                                              |
|-----------|----------------------------------------------------------------------|
| `models/` | Domain structs/enums (Subscription, Need, FamilyMember, etc.)      |
| `db/`     | SQLite via `rusqlite` (bundled) — migrations and CRUD queries       |
| `services/`| Business logic — duplicate detection, recommendations, projections |
| `ui/`     | TUI screens built with `ratatui` + `crossterm` (one file per screen)|
| `app.rs`  | Application state (active screen, running flag)                   |
| `config.rs`| Configuration and file paths                                        |

Data flow: `ui/` → `services/` → `db/` → SQLite. Models are shared across layers.

## Code Style Guidelines

### Formatting

- 4-space indentation (no tabs)
- UTF-8 encoding, LF line endings
- Maximum 100 characters per line for Rust files
- Maximum 120 characters per line for other files
- Run `cargo fmt` before committing

### Imports

- Use absolute paths within the crate: `use crate::models::Subscription;`
- Group std library imports, then external crates, then crate modules
- Sort imports alphabetically within each group
- Use `use` for bringing items into scope, avoid `use super::` when possible

Example:
```rust
use std::fmt;

use chrono::NaiveDate;

use crate::models::Subscription;
```

### Naming Conventions

- **Types** (structs, enums): `PascalCase` (e.g., `Subscription`, `Frequency`)
- **Functions and methods**: `snake_case` (e.g., `monthly_cost()`, `with_provider()`)
- **Variables and fields**: `snake_case` (e.g., `start_date`, `is_bundle`)
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Private fields**: prefix with underscore when using getters: `_id`

### Types and Derives

- Use `#[derive(Debug, Clone, PartialEq, Eq)]` for most enums and structs
- Use `#[derive(Serialize, Deserialize)]` for types that need TOML/JSON serialization
- Prefer explicit types over type inference for public APIs
- Use `Option<T>` for nullable fields, not sentinel values
- Use `f64` for monetary amounts (avoid `i32` cents)

Example:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
enum Frequency {
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, PartialEq)]
struct Subscription {
    id: Option<u64>,
    name: String,
    amount: f64,
    frequency: Frequency,
}
```

### Error Handling

- Use `anyhow::Result<T>` for application code (easy error propagation)
- Use `std::io::Result` or specific error types for library code
- Avoid `unwrap()` in production code; use `?` operator or explicit error handling
- Use `expect()` only for truly unrecoverable errors with clear messages

Example:
```rust
fn load_subscription(id: u64) -> anyhow::Result<Subscription> {
    let conn = Connection::open("data.db")?;
    // ... query and return
}
```

### Struct Construction

- Use the Builder pattern for structs with many optional fields
- Builder methods should use `with_*` naming convention
- Builder should return `Self` from setter methods for chaining

Example:
```rust
impl Subscription {
    fn builder(
        name: String,
        amount: f64,
        frequency: Frequency,
        start_date: NaiveDate,
    ) -> SubscriptionBuilder {
        SubscriptionBuilder::new(name, amount, frequency, start_date)
    }
}
```

### Tests

- Place tests in `#[cfg(test)]` modules within the same file
- Use descriptive test names: `test_monthly_cost_from_yearly`
- Include helper functions with French comments for test setup
- Run individual tests with `cargo test test_name`

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_subscription(amount: f64, frequency: Frequency) -> Subscription {
        Subscription::builder(
            "Test".to_string(),
            amount,
            frequency,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        )
        .build()
    }

    #[test]
    fn test_monthly_cost_from_yearly() {
        let subscription = make_test_subscription(99.0, Frequency::Yearly);
        assert_eq!(subscription.monthly_cost(), 8.25);
    }
}
```

### UI Code (ratatui)

- One file per screen in `src/ui/`
- Use `ratatui` widgets and `crossterm` for terminal rendering
- Follow the builder pattern for UI component configuration
- Handle resize events gracefully

### Database (rusqlite)

- Use migrations in `db/migration.rs` for schema changes
- Keep SQL queries in `db/queries.rs`
- Use parameterized queries to prevent SQL injection
- Handle `rusqlite::Error` with `?` operator

### Configuration

- Store configuration in `config.rs`
- Use TOML for configuration files with `serde` + `toml` crate
- Provide sensible defaults with `Option` overrides from config files

## Key Dependencies

- `chrono` — Date/time handling (NaiveDate)
- `rusqlite` with `bundled` feature — SQLite database
- `ratatui` + `crossterm` — Terminal UI framework
- `serde` + `toml` — Configuration and serialization
- `anyhow` — Error handling

## Design Documents

- `docs/subscription-manager-prd.md` — Full PRD with SQLite schema (7 tables), feature list
- `docs/iteration-1.md` — Step-by-step implementation guide for iteration 1 (10 tasks)

## Current Development State

The project is in early development (iteration 1). Module scaffolding is complete but most
files contain only `// TODO` stubs. The `models/subscription.rs` struct is the active
work-in-progress.

## File Patterns

- Module files: `src/<module>/mod.rs`
- Screen files: `src/ui/<screen_name>.rs`
- Model files: `src/models/<entity>.rs`
- Service files: `src/services/<service_name>.rs`
- Database files: `src/db/migration.rs`, `src/db/queries.rs`

## Editor Configuration

The project uses `.editorconfig` for consistent editor settings:
- 4-space indentation
- UTF-8 encoding
- LF line endings
- 100 char line width for Rust
