# Agent Instructions for rusty-breakout

## Build & Run
- `cargo build --release` - Build release binary
- `cargo run --release` - Run the game

## Local Quality Checks
```bash
cargo check --all-targets --all-features
cargo test --all-features -- --nocapture
cargo build --release
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Notes
- Single crate Rust project (no workspace)
- Uses macroquad game engine (v0.4)
- Has Windows-specific dependency (winapi for console control)
- No custom `.cargo` config - uses defaults
- Frame-based deterministic physics (no delta-time)

## CI
- GitHub Actions workflow at `.github/workflows/ci.yml`
- Runs: cargo check, cargo test, release build on push/PRs

## Code Structure
- Entry point: `src/main.rs`
- Game state: `src/game.rs`
- Physics: `src/physics.rs`
- Rendering: `src/ui.rs`
- 5 levels with increasing difficulty, power-ups, themes (T-key), pause (P-key)