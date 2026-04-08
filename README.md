![License](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)
SPDX-License-Identifier: MIT

# Rusty Breakout

A polished open-source Breakout/Arkanoid game built in Rust with Macroquad.

It includes 10 levels, advanced brick types, power-ups and power-downs, procedural music, controller support, persistent progress, and a built-in dev menu for fast QA.

## Overview

- 10 playable levels with escalating difficulty
- 8 power-up / power-down types
- 4 advanced brick types: Frozen, Exploding, Steel, Regenerating
- 3 difficulty modes and 5 visual themes
- Procedural soundtrack, particle effects, and achievement tracking
- Keyboard and gamepad support
- Windows release downloads through GitHub Releases

For detailed mechanics, controls, scoring, and system notes, see [gameplay-mechanics.md](/E:/Code/rusty-breakout/docs/gameplay-mechanics.md).

## Windows Install

Prebuilt Windows builds are published on GitHub Releases:

- Portable ZIP: unzip and run `breakout.exe`
- Installer: run the setup `.exe` for shortcuts and uninstall support

Downloads:
[GitHub Releases](https://github.com/rajandiappan/rusty-breakout/releases)

For release packaging details, see [windows-releases.md](/E:/Code/rusty-breakout/docs/windows-releases.md).

## Build From Source

Prerequisites:

- Rust 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- Cargo

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run --release
```

If you want to work on the game instead of just play it, clone the repo and use Cargo locally.

## Development

Local checks:

- `cargo check --all-targets --all-features`
- `cargo test --all-features -- --nocapture`
- `cargo build --release`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`

GitHub Actions:

- [ci.yml](/E:/Code/rusty-breakout/.github/workflows/ci.yml) runs validation on pushes and pull requests
- [release.yml](/E:/Code/rusty-breakout/.github/workflows/release.yml) builds Windows release assets for tags like `v0.1.0`

Project planning docs:

- [implementation_backlog.md](/E:/Code/rusty-breakout/docs/implementation_backlog.md)
- [phase_5_implementation_plan.md](/E:/Code/rusty-breakout/docs/phase_5_implementation_plan.md)
- [BREAKOUT_PRD.md](/E:/Code/rusty-breakout/BREAKOUT_PRD.md)

## Contributing

Contributions are welcome.

Good areas to help with:

- gameplay polish
- new levels and hazards
- visual and audio feedback
- test coverage
- documentation and release tooling

If you want to contribute:

1. Fork the repository
2. Create a branch for your change
3. Run the local checks
4. Open a pull request

## Screenshot

![Gameplay Screenshot](assets/screenshots/Screenshot.png)

## License

This project is released under the MIT License. See [LICENSE](/E:/Code/rusty-breakout/LICENSE).
