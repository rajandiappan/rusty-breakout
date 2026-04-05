# Quick Start Guide - Breakout Game

## 1. Prerequisites

### Install Rust
If you don't have Rust installed, download and run the installer:
https://www.rust-lang.org/tools/install

On Windows, this will install:
- `rustc` (Rust compiler)
- `cargo` (Rust package manager)
- `rustup` (Rust version manager)

Verify installation:
```bash
rustc --version
cargo --version
```

## 2. Project Setup

### Clone or navigate to project
```bash
cd "C:\Users\Administrator\Documents\Code\test project"
```

### Check project structure
```bash
# You should see:
# - Cargo.toml          (project config)
# - BREAKOUT_PRD.md     (full specification)
# - README.md           (documentation)
# - src/                (source code)
```

## 3. Build the Project

### Debug build (faster compilation, slower runtime)
```bash
cargo build
```

### Release build (slower compilation, optimized runtime)
```bash
cargo build --release
```

**First build takes 2-5 minutes** (downloading and compiling dependencies)

## 4. Run the Game

### From debug build
```bash
cargo run
```

### From release build
```bash
cargo run --release
```

Or run the compiled binary directly:
```bash
# Windows
.\target\release\breakout.exe

# macOS/Linux
./target/release/breakout
```

## 5. Game Controls

Once the game starts:

| Key | Action |
|-----|--------|
| **SPACE** | Start game / Play again |
| **LEFT ARROW** | Move paddle left |
| **RIGHT ARROW** | Move paddle right |
| **A** | Move paddle left (alternate) |
| **D** | Move paddle right (alternate) |
| **ESC** | Quit / Return to menu |

## 6. Gameplay Tips

1. **Timing is everything** - Hit the ball at the paddle edges for angle control
2. **Catch power-ups** - Gold (M), Green (P), and Purple (S) icons
3. **Multi-ball is risky** - More balls = more chaos, but more chances
4. **Paddle extend is safe** - Green icon gives you a bigger target area
5. **Slow time helps** - Purple icon lets you recover from bad situations

## 7. Project Files Overview

| File | Purpose |
|------|---------|
| `main.rs` | Game loop and entry point |
| `game.rs` | Core game logic and state |
| `physics.rs` | Collision detection |
| `level.rs` | Level definitions (5 levels) |
| `ui.rs` | Rendering and display |
| `constants.rs` | Game parameters |
| `types.rs` | Data structures |

## 8. Customization

### Change ball speed
Edit `constants.rs`:
```rust
pub const BALL_BASE_SPEED: f32 = 4.0;  // Increase for faster
```

### Change paddle width
Edit `constants.rs`:
```rust
pub const PADDLE_WIDTH: f32 = 100.0;  // Increase for easier
```

### Change power-up spawn chance
Edit `constants.rs`:
```rust
pub const POWERUP_SPAWN_CHANCE: f32 = 0.15;  // 0.15 = 15%
```

### Add new level pattern
Edit `level.rs` and add a new function like `create_spiral()`, then update `create_level_bricks()`.

## 9. Troubleshooting

### Build fails with "cargo not found"
```bash
# Ensure Rust is properly installed
rustup update

# Reinstall if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Window doesn't appear
- The game creates an 800×600 window
- Check taskbar for "Breakout" window
- Try running in release mode (may be faster)

### Very slow performance
```bash
# Use release build for optimization
cargo run --release
```

### Compile errors about dependencies
```bash
# Update and rebuild
cargo update
cargo clean
cargo build --release
```

## 10. Next Steps

1. **Read the PRD** (`BREAKOUT_PRD.md`) for complete specifications
2. **Explore the code** starting with `main.rs`
3. **Modify and experiment** with constants and physics
4. **Add features** like sound, particles, or new power-ups
5. **Share and contribute** improvements!

## 11. Useful Commands

```bash
# Check code for errors without building
cargo check

# Format code automatically
cargo fmt

# Run with optimized checks
cargo run --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Show detailed build output
cargo build -vv
```

## 12. System Requirements

- **OS:** Windows 10+, macOS 10.14+, Linux (X11/Wayland)
- **RAM:** 256 MB minimum
- **Disk:** ~500 MB for full build (including target/)
- **Graphics:** GPU supporting OpenGL 3.0+ (or via WGPU)

## 13. Development Resources

- **Macroquad Documentation:** https://docs.rs/macroquad/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Cargo Guide:** https://doc.rust-lang.org/cargo/

## 14. Support

If you encounter issues:

1. Check `README.md` for architecture overview
2. Review `BREAKOUT_PRD.md` for specifications
3. Examine the relevant source file (check comments)
4. Try running `cargo clean` then `cargo build --release`

---

**You're all set! Press SPACE to start playing!** 🎮
