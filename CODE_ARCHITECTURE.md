# Code Architecture Guide

## Overview

This guide explains how the Breakout game code is organized and how the major components work together.

## Module Dependency Graph

```
main.rs
  └─→ game.rs
       ├─→ physics.rs (collision detection)
       ├─→ level.rs (level generation)
       ├─→ ui.rs (rendering)
       ├─→ types.rs (data structures)
       └─→ constants.rs (game parameters)

Supporting modules:
  - ball.rs (expandable - currently placeholder)
  - paddle.rs (expandable - currently placeholder)
  - brick.rs (expandable - currently placeholder)
  - powerup.rs (expandable - currently placeholder)
```

## File-by-File Breakdown

### `main.rs` - Entry Point & Game Loop

**Purpose:** Initializes the game and manages the core loop.

**Key Components:**
```rust
#[macroquad::main("Breakout")]
async fn main() {
    let mut game = Game::new();
    
    loop {
        // Input
        if is_key_pressed(KeyCode::Escape) { break; }
        
        // Update
        game.update().await;
        
        // Render
        clear_background(BLACK);
        game.render();
        
        // Frame timing
        next_frame().await;
    }
}
```

**Flow:**
1. Creates a new `Game` instance
2. Runs infinite loop at ~60 FPS (Macroquad handles timing)
3. Each frame: handle input → update → render
4. ESC key exits

### `constants.rs` - Game Parameters

**Purpose:** Centralized configuration for easy tweaking.

**Key Constants:**
- `SCREEN_WIDTH`, `SCREEN_HEIGHT` - Display size (800×600)
- `BALL_RADIUS`, `BALL_BASE_SPEED` - Ball physics
- `PADDLE_WIDTH`, `PADDLE_SPEED` - Paddle control
- `BRICK_WIDTH`, `BRICK_HEIGHT`, `BRICK_COLS`, `BRICK_ROWS` - Brick grid
- `POWERUP_SPAWN_CHANCE`, `POWERUP_DURATION` - Power-up behavior
- `BRICK_COLORS` - Rainbow color array for rows

**Usage:** Import with `use crate::constants::*;`

### `types.rs` - Data Structures

**Purpose:** Defines all core game objects.

**Key Types:**

```rust
enum GamePhase {
    MainMenu, Playing, LevelComplete, GameOver, Victory
}

enum PowerUpType {
    MultiBall, PaddleExtend, SlowTime
}

struct Ball {
    x, y: f32,           // Position
    vx, vy: f32,         // Velocity
    radius: f32,
    active: bool
}

struct Paddle {
    x, y: f32,
    width, height: f32,
    normal_width, extended_width: f32,
    is_extended: bool
}

struct Brick {
    x, y: f32,
    width, height: f32,
    active: bool,
    color: Color
}

struct GameState {
    level, score, high_score: u32,
    lives: u8,
    phase: GamePhase,
    balls: Vec<Ball>,
    paddle: Paddle,
    bricks: Vec<Brick>,
    powerups: Vec<PowerUp>,
    active_powerups: Vec<ActivePowerUp>,
    frame_count, level_complete_timer: usize
}

struct Game {
    state: GameState
}
```

### `game.rs` - Core Game Logic

**Purpose:** Manages game state, updates, and transitions.

**Key Methods:**

| Method | Purpose |
|--------|---------|
| `new()` | Initialize game, show menu |
| `start_menu()` | Set phase to MainMenu |
| `start_game()` | Begin gameplay |
| `load_level(n)` | Generate bricks, reset ball/paddle |
| `update()` | Main update loop (async) |
| `update_menu()` | Handle menu input |
| `update_playing()` | Game logic: paddle, balls, collisions |
| `update_balls()` | Move balls, detect losses |
| `update_powerups()` | Move falling powerups, update timers |
| `check_collisions()` | Ball-paddle, ball-brick, powerup pickups |
| `apply_powerup()` | Apply power-up effect |
| `check_game_conditions()` | Win/lose checks |
| `render()` | Dispatch to UI renderers |

**Game Flow:**
```
MainMenu
  ↓ (SPACE pressed)
Playing (each level 1-5)
  ├─ Update balls, paddle, power-ups
  ├─ Check collisions
  ├─ All bricks destroyed?
  └─ → LevelComplete
  
LevelComplete (2-sec wait)
  ├─ More levels?
  ├─ → Load next level & Playing
  └─ No → Victory

Playing: Ball lost?
  ├─ More lives?
  ├─ → Reset ball & Playing
  └─ No → GameOver

GameOver/Victory
  ↓ (SPACE pressed)
MainMenu
```

### `physics.rs` - Collision Detection

**Purpose:** Handle all collision logic.

**Key Functions:**

#### `check_ball_paddle_collision()`
- Uses bounding box to detect collision
- Only bounces if ball comes from above
- Calculates hit position for angle variation
- Formula: `vx_new = hit_pos * 2.5`
- Clamps velocity to `BALL_MAX_SPEED` (6.0)

```rust
// Simplified logic:
if ball overlaps paddle {
    vy = -abs(vy)  // Bounce up
    
    hit_pos = (ball.x - paddle.center) / (paddle.width / 2)
    vx = hit_pos * 2.5  // Angle control
}
```

#### `check_ball_brick_collision()`
- Uses closest-point algorithm
- Determines if hit was from top/bottom or left/right
- Flips appropriate velocity component
- Returns true if collision (for scoring)

```rust
if ball overlaps brick {
    brick.active = false  // Destroy
    
    if hit_from_top_or_bottom {
        vy = -vy
    } else {
        vx = -vx
    }
}
```

#### `check_powerup_pickup()`
- Simple circle-rectangle collision
- Returns true if paddle touched power-up

### `level.rs` - Level Generation

**Purpose:** Create brick patterns for each level.

**Pattern Functions:**

| Level | Function | Description |
|-------|----------|-------------|
| 1 | `create_full_grid()` | All 72 bricks |
| 2 | `create_alternating_rows()` | Every other row filled |
| 3 | `create_spiral()` | Spiral from edge to center |
| 4 | `create_checkerboard()` | Diagonal checkerboard |
| 5 | `create_random()` | Seeded random placement |

**Key Function:**
```rust
pub fn create_level_bricks(level: usize) -> Vec<Brick> {
    let mut bricks = Vec::new();
    
    match level {
        1 => create_full_grid(&mut bricks),
        2 => create_alternating_rows(&mut bricks),
        // ...
    }
    
    bricks
}
```

Each pattern creates bricks with:
- Position: calculated from grid position
- Color: `BRICK_COLORS[row % 6]` (rainbow)
- Active state: `true`

### `ui.rs` - Rendering

**Purpose:** Draw all game objects and UI.

**Key Functions:**

```rust
render_game()           // Draw bricks, balls, paddle, powerups, HUD
render_main_menu()      // Title screen
render_level_complete() // Level complete message
render_game_over()      // Game over screen
render_victory()        // Victory screen
```

**Rendering Order (Front-to-Back):**
1. Bricks (base layer)
2. Balls
3. Paddle
4. Power-ups
5. HUD text (top/bottom)

**Text Rendering:**
```rust
// Centered text example:
let text_width = measure_text(text, None, size, 1.0).width;
draw_text(text, CENTER_X - text_width / 2.0, Y, size, color);
```

## Data Flow

### Per-Frame Flow

```
Input Phase:
  └─ is_key_pressed/is_key_down() → paddle movement

Update Phase:
  ├─ Move paddle (clamp to bounds)
  ├─ Move balls
  │  ├─ Apply slow-time multiplier if active
  │  ├─ Check wall collisions
  │  └─ Check bottom (lose life)
  ├─ Move falling power-ups
  ├─ Update power-up timers
  ├─ Check collisions
  │  ├─ Ball-paddle → apply bounce
  │  ├─ Ball-brick → destroy brick, spawn powerup
  │  └─ Powerup-paddle → apply effect
  └─ Check win/lose conditions

Render Phase:
  └─ Draw all objects and UI
  
Timing:
  └─ next_frame() waits for 1/60th second
```

### Power-Up Application

When paddle touches a power-up:

```rust
match power_type {
    MultiBall => {
        // Clone original ball with rotated velocities
        // Spawn up to 3 total balls
    }
    PaddleExtend => {
        // Set paddle.width = PADDLE_EXTENDED_WIDTH
        // Add ActivePowerUp with POWERUP_DURATION frames
    }
    SlowTime => {
        // Add ActivePowerUp
        // In ball update: apply 0.5 multiplier to velocity
    }
}
```

## State Machine Transitions

```
GamePhase::MainMenu
  ↓ update_menu() sees SPACE
GamePhase::Playing
  ├─ update_playing() runs
  ├─ All bricks destroyed?
  │  ↓ Yes
  │  GamePhase::LevelComplete (120 frame timer)
  │    ├─ Last level?
  │    ├─ No → Load level N+1, set Playing
  │    └─ Yes → GamePhase::Victory
  └─ No more balls?
     ↓
     GamePhase::GameOver
     ↓ update_game_over() sees SPACE
     GamePhase::MainMenu
```

## Key Physics Concepts

### Deterministic Physics
- No delta-time variance
- Every frame: position += velocity
- Speed in pixels-per-frame
- Makes gameplay predictable

### Collision Response
- **Walls:** Reverse velocity component
- **Paddle:** Reverse Y, adjust X based on hit position
- **Bricks:** Reverse X or Y (whichever side entered)
- **Bottom:** Lose life, reset

### Velocity Clamping
```rust
let speed = (vx*vx + vy*vy).sqrt();
if speed > max_speed {
    let scale = max_speed / speed;
    vx *= scale;
    vy *= scale;
}
```
Prevents velocity explosion from multiple collisions.

## Extensibility Points

### Add New Power-Up
1. Add variant to `PowerUpType` enum (types.rs)
2. Handle in `game.rs::apply_powerup()` match
3. Update power-up rendering in `ui.rs::render_game()`
4. Implement logic (modify ball, paddle, or add new system)

### Add New Level Pattern
1. Create new function in `level.rs` (e.g., `create_waves()`)
2. Update match in `create_level_bricks()`
3. Test pattern

### Enhance Graphics
1. Add drawing calls in `ui.rs::render_game()`
2. Use Macroquad drawing functions:
   - `draw_rectangle()` / `draw_circle()`
   - `draw_texture()` (for sprites)
   - `draw_poly()` (for shapes)

### Add Sound
1. Load audio in `Game::new()`
2. Play in collision handlers or state changes
3. Use Macroquad's audio functions

## Common Patterns

### Iteration Over Active Objects
```rust
for brick in self.state.bricks.iter_mut() {
    if !brick.active { continue; }
    // Process brick
}

// Or cleanup:
self.state.bricks.retain(|b| b.active);
```

### Collision Detection Loop
```rust
for ball in &mut self.state.balls {
    for (idx, brick) in self.state.bricks.iter_mut().enumerate() {
        if check_ball_brick_collision(ball, brick) {
            // Handle collision
            break;  // Only process first brick per ball per frame
        }
    }
}
```

### Text Centering
```rust
let text = "LEVEL COMPLETE";
let width = measure_text(text, None, 40, 1.0).width;
draw_text(text, SCREEN_WIDTH/2.0 - width/2.0, Y, 40.0, color);
```

## Debugging Tips

### Add Debug Rendering
```rust
// In ui.rs::render_game() after other objects:
for ball in &state.balls {
    draw_circle_lines(ball.x, ball.y, ball.radius, 1.0, YELLOW);
}
draw_rectangle_lines(state.paddle.x, state.paddle.y, 
                     state.paddle.width, state.paddle.height, 
                     1.0, RED);
```

### Print Debug Info
```rust
println!("Ball: ({}, {}), Velocity: ({}, {})", 
         ball.x, ball.y, ball.vx, ball.vy);
println!("Active power-ups: {}", state.active_powerups.len());
```

### Frame-by-Frame
Add pause/resume:
```rust
if is_key_pressed(KeyCode::P) { 
    paused = !paused; 
}
if !paused {
    game.update();
}
```

## Performance Considerations

### Collision Detection
- O(balls × bricks) per frame
- For 3 balls × 72 bricks = 216 checks/frame
- At 60 FPS = ~13k checks/sec (very fast)

### Memory Usage
- `GameState` with max capacity: ~10 KB
- Textures/assets: minimal (no sprites yet)
- Total runtime: <50 MB

### Optimization Opportunities
1. **Spatial partitioning** (if many bricks)
2. **Batch rendering** (handled by Macroquad)
3. **Object pooling** (for powerups)
4. **Multi-threading** (for physics, if needed)

## Testing Checklist

- [ ] Ball bounces off walls correctly
- [ ] Paddle controls respond smoothly
- [ ] Ball bounces with angle variation
- [ ] Bricks are destroyed on hit
- [ ] Power-ups spawn and fall
- [ ] Power-ups can be caught
- [ ] Multi-ball spawns 2 extra balls
- [ ] Paddle extend works and times out
- [ ] Slow time effect applies
- [ ] Level completes when all bricks destroyed
- [ ] Levels advance correctly
- [ ] Game over on 0 lives
- [ ] Victory shown on level 5 complete

---

**Ready to explore the code!** Start with `main.rs` and trace through to `game.rs` for the full picture.
