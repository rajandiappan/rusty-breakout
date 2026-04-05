# Breakout Game - Rust + Macroquad Implementation

A classic arcade-style Breakout/Arkanoid game implemented in Rust using the Macroquad game engine.

## Project Structure

```
breakout/
├── Cargo.toml                    # Project dependencies and metadata
├── BREAKOUT_PRD.md              # Complete Product Requirements Document
├── README.md                     # This file
└── src/
    ├── main.rs                  # Entry point and main game loop
    ├── constants.rs             # Game constants (screen size, speeds, colors)
    ├── types.rs                 # Core data structures (Ball, Paddle, Brick, etc)
    ├── game.rs                  # Game state management and logic
    ├── physics.rs               # Collision detection and response
    ├── level.rs                 # Level definitions and generation
    ├── ui.rs                    # Rendering and UI display
    ├── ball.rs                  # Ball-specific behavior (expandable)
    ├── paddle.rs                # Paddle-specific behavior (expandable)
    ├── brick.rs                 # Brick-specific behavior (expandable)
    └── powerup.rs               # Power-up-specific behavior (expandable)
```

## Overview

This is a fully-featured Breakout game with:

- **5 Progressive Levels** with increasing difficulty
- **Ball Physics** with deterministic, frame-based movement
- **Collision Detection** (walls, paddle, bricks)
- **3 Power-Up Types:**
  - Multi-Ball: Spawn 2 additional balls
  - Paddle Extend: Increase paddle width temporarily
  - Slow Time: Reduce ball velocity by 50%
- **Lives System** (start with 3 lives)
- **Score Tracking** with high score persistence
- **Game States:** Main Menu, Playing, Level Complete, Game Over, Victory

## Key Features

### Physics Engine
- Deterministic ball movement (frame-based, no delta-time)
- Angle-variation on paddle bounces based on hit position
- Proper collision detection using closest-point algorithm
- Ball velocity clamping to prevent speed explosion

### Level Design
| Level | Pattern | Speed | Difficulty |
|-------|---------|-------|------------|
| 1 | Full grid | 4 px/f | Easy |
| 2 | Alternating rows | 4.6 px/f | Easy-Medium |
| 3 | Spiral pattern | 5.3 px/f | Medium |
| 4 | Checkerboard | 6.1 px/f | Medium-Hard |
| 5 | Random distribution | 7.0 px/f | Hard |

### Game Constants
- Screen: 800×600 pixels
- Ball radius: 5 pixels
- Paddle: 100 pixels wide (150 when extended), 15 pixels tall
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
| ESC | Quit to menu / Exit game |

## Gameplay

1. **Main Menu:** Press SPACE to start
2. **Playing:** Control paddle with arrow keys, destroy all bricks
3. **Power-ups:** Fall from destroyed bricks (15% chance each)
   - Gold (M): Spawn extra balls
   - Green (P): Extend paddle temporarily
   - Purple (S): Slow down ball
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
    width: f32,               // Current width
    height: f32,              // Height (constant)
    normal_width: f32,        // Normal width
    extended_width: f32,      // Extended width
    is_extended: bool,        // Currently extended?
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
- Increases paddle width to 150 pixels
- Duration: 60 frames (1 second)
- Defensive, safety-focused
- Restacking resets timer

### Slow Time (S - Purple)
- Reduces ball velocity to 50%
- Duration: 60 frames
- Defensive, skill-focused
- Only affects ball speed, not paddle

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

- Boss levels with special mechanics
- Difficulty modes (Easy/Normal/Hard)
- Sound effects and music
- Particle effects on collisions
- Mobile touch controls
- Advanced brick types (steel, explosive)
- Curved paddle surface physics
- Achievements/badges

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
