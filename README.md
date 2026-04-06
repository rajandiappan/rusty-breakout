# Breakout Game - Rust + Macroquad Implementation

A classic arcade-style Breakout/Arkanoid game implemented in Rust using the Macroquad game engine.

## Project Structure

```
breakout/
├── Cargo.toml                    # Project dependencies and metadata
├── BREAKOUT_PRD.md              # Complete Product Requirements Document
├── README.md                     # This file
├── PHASE2_PROGRESS.md           # Phase 2 implementation progress
└── src/
    ├── main.rs                  # Entry point and main game loop
    ├── constants.rs             # Game constants (screen size, speeds, colors)
    ├── types.rs                 # Core data structures (Ball, Paddle, Brick, etc)
    ├── game.rs                  # Game state management and logic
    ├── physics.rs               # Collision detection and response
    ├── level.rs                 # Level definitions and generation
    ├── ui.rs                    # Rendering and UI display (Phase 2: theme-aware)
    ├── ball.rs                  # Ball-specific behavior (expandable)
    ├── paddle.rs                # Paddle-specific behavior (expandable)
    ├── brick.rs                 # Brick-specific behavior (expandable)
    ├── powerup.rs               # Power-up-specific behavior (expandable)
    │
    ├── settings.rs              # [PHASE 2] Difficulty and theme definitions
    ├── themes.rs                # [PHASE 2] 5 color theme system with palettes
    ├── achievements.rs          # [PHASE 2] Achievement tracking and management
    ├── effects.rs               # [PHASE 2] Particle effects system (230 lines)
    └── audio.rs                 # [PHASE 3] Audio system for game sound events
```

## Overview

This is a fully-featured Breakout game with Phase 3 audio and professional polish:

- **5 Progressive Levels** with increasing difficulty
- **Ball Physics** with deterministic, frame-based movement
- **Collision Detection** (walls, paddle, bricks)
- **4 Power-Up Types + 1 Power-Down:**
   - Multi-Ball: Spawn 2 additional balls
   - Paddle Extend: Increase paddle width PERMANENTLY until next level or Paddle Shrink
   - Slow Time: Reduce ball velocity by 50%
   - Paddle Shrink: [POWER-DOWN] Decrease paddle width (collect Paddle Extend to reverse)
- **Lives System** (configurable by difficulty: 2-5 lives)
- **Score Tracking** with high score persistence
- **Game States:** Main Menu, Playing, Level Complete, Game Over, Victory
- **Difficulty Modes:** Easy, Normal, Hard with dynamic multipliers
- **5 Color Themes:** Classic, Dark, Neon, CRT, Minimalist with T-key switching
- **Particle Effects:** Brick explosions, paddle hits, power-up spawns/pickups
- **Pause/Resume:** P-key toggle with pause overlay
- **Audio System:** Sound effects for all game events (Phase 3)
- **Achievement System:** Track gameplay metrics and unlock badges
- **Settings Persistence:** Save and load user preferences

## Key Features

### Physics Engine
- Deterministic ball movement (frame-based, no delta-time)
- Angle-variation on paddle bounces based on hit position
- Proper collision detection using closest-point algorithm
- Ball velocity clamping to prevent speed explosion

### Difficulty System (Phase 2)
Selectable difficulty modes with dynamic multipliers:
| Mode | Ball Speed | Paddle Width | Lives | Power-Up Chance |
|------|-----------|--------------|-------|-----------------|
| Easy | 0.8x | 130px (+30%) | 5 | 25% |
| Normal | 1.0x | 100px | 3 | 15% |
| Hard | 1.3x | 70px (-30%) | 2 | 10% |

### Visual Themes (Phase 2)
5 professional color schemes (cycle with T-key):
- **Classic:** Original 8-bit arcade palette
- **Dark:** Low-light gaming friendly
- **Neon:** High-contrast cyberpunk aesthetic
- **CRT:** Retro monitor scanlines and glow
- **Minimalist:** Clean, flat design

### Particle Effects (Phase 2)
Visual feedback system:
- Brick destruction: 12-particle bursts with velocity
- Paddle hits: 8-particle collision effects
- Power-up spawns: 16-particle emission
- Power-up pickups: 20-particle celebration effect

### Pause & Settings (Phase 2)
- **P-key pause:** Freeze gameplay while keeping particles/UI running
- **Theme switching:** Real-time theme changes with T-key
- **Settings persistence:** Automatically save user preferences

### Achievement System (Phase 2)
10 achievements across 3 categories:
- **Skill:** Sharpshooter, Rapid Fire, Perfect Clear, Speedrunner, Multi-Ball Master
- **Collection:** Power-Up Hoarder, Lucky Break, Time Bender
- **Exploration:** Theme Collector, Hardcore Champion

### Audio System (Phase 3)
AudioManager with sound effects for all game events:
- **Paddle Hit:** 400Hz beep (40ms) when ball touches paddle
- **Brick Destroy:** 600Hz beep (80ms) on brick destruction
- **Power-Up Pickup:** 900Hz beep (150ms) on power-up collection
- **Level Complete:** 700Hz beep (200ms) when level cleared
- **Game Over:** 300Hz beep (300ms) on losing all lives
- **Victory:** 800Hz beep (400ms) on completing all levels

### Game Constants
- Screen: 800×600 pixels
- Ball radius: 5 pixels
- Paddle: 100 pixels wide (150 when extended, 60 when shrunk), 15 pixels tall
- Bricks: 60×20 pixels, 12×6 grid (72 per level)
- Power-ups: 15% spawn chance, 60-frame duration

## Building & Running

### Prerequisites
- Rust 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| LEFT Arrow / A | Move paddle left |
| RIGHT Arrow / D | Move paddle right |
| SPACE | Start game / Play again |
| P | Pause/Resume during gameplay |
| T | Cycle through themes (5 color schemes) |
| ESC | Quit to menu / Exit game |

## Gameplay

1. **Main Menu:** Press SPACE to start
2. **Playing:** Control paddle with arrow keys, destroy all bricks
3. **Power-ups:** Fall from destroyed bricks (15% chance each)
   - Gold (M): Spawn extra balls
   - Green (P): Extend paddle PERMANENTLY (until next level or shrink)
   - Purple (S): Slow down ball
   - Red/Dark (S): Shrink paddle [POWER-DOWN] - collect green extend to reverse
4. **Level Complete:** Auto-advances after 2 seconds
5. **Victory:** Complete all 5 levels to win!
6. **Game Over:** Run out of lives to lose

## Code Architecture

### Main Game Loop (main.rs)
```rust
loop {
    // Input
    handle_input()
    
    // Update
    game.update()
    
    // Render
    clear_background()
    game.render()
    
    // Timing (60 FPS)
    next_frame()
}
```

### Game State Management (game.rs)
- `GameState`: Holds all game data (score, lives, balls, paddle, bricks, etc)
- `Game`: Manages game flow and state transitions
- Methods for level loading, collisions, power-ups, and rendering

### Physics (physics.rs)
- `check_ball_paddle_collision()`: Paddle bounce with angle variation
- `check_ball_brick_collision()`: Brick destruction with side detection
- `check_powerup_pickup()`: Power-up collection

### Rendering (ui.rs)
- `render_game()`: Main HUD and game objects
- `render_main_menu()`: Title screen
- `render_level_complete()`: Level completion screen
- `render_game_over()`: Game over screen
- `render_victory()`: Victory screen

## Data Structures

### Ball
```rust
struct Ball {
    x: f32, y: f32,           // Position
    vx: f32, vy: f32,         // Velocity (pixels/frame)
    radius: f32,              // Collision radius
    active: bool,             // Is ball in play?
}
```

### Paddle
```rust
struct Paddle {
    x: f32, y: f32,           // Position
    width: f32,               // Current width (changes with extend/shrink)
    height: f32,              // Height (constant)
    normal_width: f32,        // Base width (difficulty-adjusted)
    extended_width: f32,      // Width when extended (1.5x normal)
    shrunk_width: f32,        // Width when shrunk (0.6x normal)
    is_extended: bool,        // Currently extended?
    is_shrunk: bool,          // Currently shrunk?
}
```

### Brick
```rust
struct Brick {
    x: f32, y: f32,           // Position
    width: f32, height: f32,  // Dimensions
    active: bool,             // Not yet destroyed?
    color: Color,             // Display color
}
```

## Physics Details

### Ball Movement
```
x_new = x + vx
y_new = y + vy
```

### Paddle Collision with Angle Variation
```
// Reverse vertical velocity
vy_new = -abs(vy)

// Calculate normalized hit position (-1.0 to 1.0)
hit_pos = (ball_x - paddle_center) / (paddle_width / 2)

// Apply horizontal spin
vx_new = hit_pos * 2.5

// Clamp total speed to max_speed (6.0)
```

### Brick Collision Detection
Uses closest-point algorithm to determine entry side:
- If |dx| > |dy|: Horizontal entry → flip vx
- Otherwise: Vertical entry → flip vy

## Scoring

- Brick destroyed: +10 points
- Level completed: +1000 bonus points
- All 5 levels completed: +5000 bonus points
- High score is saved and displayed

## Power-Up System

### Multi-Ball (M - Gold)
- Spawns 2 additional balls at ±20° angles
- Maximum 3 balls in play
- Risk/reward mechanic

### Paddle Extend (P - Green)
- Increases paddle width to 150 pixels (1.5x normal)
- **PERMANENT effect**: Lasts until next level or until Paddle Shrink is collected
- Defensive, safety-focused
- Only one extend active at a time

### Slow Time (S - Purple)
- Reduces ball velocity to 50%
- Duration: 60 frames (1 second at 60 FPS)
- Defensive, skill-focused
- Only affects ball speed, not paddle

### Paddle Shrink (S - Red/Dark) [POWER-DOWN]
- Decreases paddle width to 60 pixels (60% of normal)
- Icon: Circle bomb (◈)
- Effect: Reduces paddle size making gameplay more challenging
- **Reversed by**: Collecting a Paddle Extend power-up
- Strategic trade-off: Risk vs. reward on power-up collection

## Collision Handling Priority

1. **Paddle** (highest priority - stops ball from falling)
2. **Walls** (side walls and top)
3. **Bricks** (lowest priority)
4. **Bottom** (game over zone)

## Performance

- Target: 60 FPS locked
- Memory usage: <50 MB
- Deterministic physics (frame-based, no delta-time variance)
- Build time: <10 seconds (with fast compiles configuration)

## Future Enhancement Ideas

Phase 1, 2, and 3 (audio foundation) are complete. Phase 4+ ideas:

- Real audio synthesis using rodio or platform-specific audio APIs
- Boss levels with special mechanics
- Mobile touch controls
- Advanced brick types (steel, explosive)
- Curved paddle surface physics
- Extended level content (15+ additional levels)
- Online leaderboards and cloud sync
- Replay recording and sharing
- Competitive multiplayer modes

## Development Notes

### Adding New Features
1. **New Power-Up:** Add to `PowerUpType` enum in `types.rs`, handle in `game.rs::apply_powerup()`
2. **New Level:** Add pattern function in `level.rs::create_level_bricks()`
3. **New Rendering:** Add functions in `ui.rs`

### Testing Physics
The closest-point collision algorithm handles:
- Corner hits correctly
- Multiple simultaneous collisions (processed in priority order)
- Ball stuck in brick (allows 1-frame exit)
- Edge cases (paddle at screen edge, high velocity)

### Debug Mode
Add debug rendering by modifying `ui.rs::render_game()`:
```rust
// Draw collision boxes
draw_rectangle_lines(paddle.x, paddle.y, paddle.width, paddle.height, 2.0, YELLOW);
```

## References

- **PRD Document:** See `BREAKOUT_PRD.md` for complete specifications
- **Macroquad Docs:** https://docs.rs/macroquad/
- **Macroquad Examples:** https://github.com/not-fl3/macroquad/tree/master/examples

## License

This is an educational project based on the classic Breakout/Arkanoid arcade games.

## Contributing

This project is designed as a learning resource. Feel free to:
- Add new level patterns
- Implement new power-ups
- Enhance physics
- Add visual effects
- Optimize performance

---

**Status:** Ready for compilation and testing with Rust 1.70+
