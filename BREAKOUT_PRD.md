# Breakout Game - Product Requirements Document (PRD)

## 1. Executive Summary

**Project Name:** Breakout: Classic Arcade Revival  
**Platform:** Cross-platform (Windows, macOS, Linux, Web via WASM)  
**Target Audience:** Casual gamers, retro game enthusiasts  
**Engine:** Rust + Macroquad  
**Scope:** Complete, playable 5-level arcade-style Breakout game with power-ups and scoring  
**Development Complexity:** Intermediate (physics + state management)

---

## 2. Game Overview

### 2.1 Core Concept

A faithful implementation of the classic Breakout/Arkanoid arcade game. The player controls a paddle to bounce a ball, destroying bricks while managing power-ups, lives, and increasing difficulty across 5 levels.

### 2.2 Game Loop

1. **Initialize:** Load level, position paddle/ball, display UI
2. **Input:** Read keyboard input (arrow keys) for paddle movement
3. **Update:** Move ball, check collisions, apply physics, spawn power-ups
4. **Render:** Draw paddle, ball, bricks, UI, particles
5. **Check Win/Lose:** If all bricks destroyed → next level. If ball falls → lose life.
6. **Repeat:** Loop until game over or all 5 levels completed

### 2.3 Target Specifications

- **Screen Resolution:** 800×600 pixels (16:12 aspect ratio)
- **Frame Rate:** 60 FPS
- **Input Latency:** <16ms (responsive paddle control)
- **Physics:** Deterministic (frame-based, no delta-time variance)

---

## 3. Gameplay Mechanics

### 3.1 Core Objects

#### Ball

**Properties:**
- Position: `(x, y)`
- Velocity: `(vx, vy)` measured in pixels/frame
- Radius: 5 pixels
- Color: Bright cyan (#00FFFF)
- Speed: Base 4 pixels/frame, increases slightly per level

**Behavior:**
- Moves every frame: `x += vx`, `y += vy`
- Bounces off walls, paddle, and bricks
- Falls below paddle → lose 1 life
- Resets to paddle position on ball loss
- Maximum 3 balls in play (multi-ball power-up)

#### Paddle

**Properties:**
- Width: 100 pixels (expandable via power-up)
- Height: 15 pixels
- Color: Bright white (#FFFFFF)
- Position: Always at `y = 550` (bottom third of screen)
- Movement Speed: 7 pixels/frame

**Behavior:**
- Controlled by LEFT/RIGHT arrow keys or A/D
- Stays within screen bounds (0 to 700 on x-axis)
- Ball bounces with angle variation based on hit position
- Can be extended to 150 pixels via power-up

#### Bricks

**Properties:**
- Width: 60 pixels
- Height: 20 pixels
- Grid: 12 columns × 6 rows per level (72 bricks)
- Color: Varies by row (rainbow pattern)
- Top-left origin at (20, 60)
- Spacing: 8 pixels between bricks

**Behavior:**
- Destroyed on ball contact
- Award 10 points each
- Drop power-ups with 15% probability

#### Lives & Scoring

- **Lives:** Start with 3
- **Loss Condition:** Lose 1 life when ball falls below paddle
- **Game Over:** 0 lives remaining
- **Scoring:**
  - Brick destroyed: +10 points
  - Level completed: +1000 points
  - All 5 levels completed: +5000 bonus
- **High Score:** Persisted to `data/highscore.txt`

### 3.2 Power-Ups

#### Power-Up Mechanics

- **Spawn:** 15% chance when any brick is destroyed
- **Duration:** 60 frames (1 second at 60 FPS)
- **Fall Speed:** 3 pixels/frame
- **Pickup Zone:** Paddle rectangle
- **Visual:** 20×20 pixel square with icon/letter

#### Power-Up Types

##### 1. Multi-Ball (M)

- **Color:** Golden (#FFD700)
- **Effect:** Spawn 2 additional balls from current ball position
- **Behavior:**
  - New balls have velocities at ±20° angles from original
  - Up to 3 balls maximum in play
  - If any ball is lost, continue with remaining
  - Extra balls = extra chances
- **Score Multiplier:** None (risk/reward based)

##### 2. Paddle Extend (P)

- **Color:** Green (#00FF00)
- **Effect:** Increase paddle width from 100 → 150 pixels
- **Duration:** 60 frames, then shrink back
- **Behavior:**
  - Animation: Smooth expand/shrink (4-pixel transitions)
  - Helps catch faster balls
  - Stacking: New activation resets timer
- **Strategic Value:** Defensive, safe

##### 3. Slow Time (S)

- **Color:** Purple (#9933FF)
- **Effect:** Reduce ball velocity by 50% globally
- **Duration:** 60 frames
- **Behavior:**
  - Affects only ball speed, not paddle speed
  - Multiplier: `velocity *= 0.5`
  - Helps recover from chaos/multi-ball situations
  - Stacking: New activation resets timer
- **Strategic Value:** Defensive, skill-based

#### Power-Up UI

- Active power-ups displayed in top-left corner
- Show icon + remaining time in frames
- Dim when ~10 frames remaining

---

## 4. Level Design & Progression

### 4.1 Level Structure

All levels have:
- **Grid:** 12 columns × 6 rows (72 bricks)
- **Starting Ball Speed:** Base speed × (1.0 + level × 0.15)

### 4.2 Level Specifications

| Level | Pattern | Ball Speed | Difficulty | Special Notes |
|-------|---------|-----------|------------|---------------|
| 1 | Full grid (all bricks) | 4 px/f | Easy | Tutorial level, all bricks same color |
| 2 | Alternating rows | 4.6 px/f | Easy-Medium | Rows alternate between filled/empty |
| 3 | Spiral pattern | 5.3 px/f | Medium | Bricks in spiral from outside → center |
| 4 | Checkerboard | 6.1 px/f | Medium-Hard | Diagonal checkerboard pattern |
| 5 | Random distribution | 7.0 px/f | Hard | Random brick placement (seeded for consistency) |

### 4.3 Progression Mechanics

- Complete level when all bricks destroyed
- Advance to next level automatically after 2-second delay
- Score persists across levels
- Lives persist across levels
- Reset ball position between levels
- High score updates only at game end

---

## 5. Physics Engine

### 5.1 Movement Physics

#### Ball Motion

```
Δt = 1 frame
Position update: 
  x_new = x + vx * Δt
  y_new = y + vy * Δt
```

**Deterministic:** No acceleration, constant velocity until collision.

### 5.2 Collision Detection & Response

#### 5.2.1 Wall Collisions

**Left/Right Walls** (x ≤ 0 or x ≥ 795)
- **Detection:** Ball center crosses wall boundary
- **Response:** `vx = -vx`
- **Correction:** Clamp position to valid range

**Top Wall** (y ≤ 0)
- **Detection:** Ball center crosses top boundary
- **Response:** `vy = -vy`
- **Correction:** Set y = 0

**Bottom (Game Over Zone)** (y ≥ 600)
- **Detection:** Ball center passes bottom edge
- **Response:**
  - Lose 1 life
  - Reset ball to paddle center
  - Check if lives > 0 → continue or game over

#### 5.2.2 Paddle Collision

**Detection Algorithm:**

```
Bounding box intersection:
  paddle_rect = Rect(paddle_x, 550, paddle_w, 15)
  ball_rect = Circle(ball_x, ball_y, 5)
  
  Collision if ball circle overlaps paddle rect
```

**Response with Angle Variation:**

```
1. Reverse vertical velocity:
   vy_new = -abs(vy)  // Ensure upward motion

2. Calculate hit position (normalized):
   hit_pos = (ball_x - paddle_center_x) / (paddle_width / 2)
   // Range: -1.0 (left edge) to +1.0 (right edge)

3. Apply horizontal spin:
   vx_new = hit_pos * 2.5
   // Formula provides responsive angle control
   // At edges: vx ≈ ±2.5 (45° angle)
   // At center: vx ≈ 0 (straight up)

4. Clamp total speed:
   speed = sqrt(vx_new² + vy_new²)
   if speed > max_speed:
     scale = max_speed / speed
     vx_new *= scale
     vy_new *= scale
   // Prevents velocity explosion
   // max_speed = 6.0 pixels/frame
```

**Edge Cases:**
- Multiple paddle collisions in single frame: Process only first
- Ball entering from below paddle: Treated as bottom wall (lose life)

#### 5.2.3 Brick Collision

**Detection Algorithm:**

```
For each brick:
  brick_rect = Rect(brick_x, brick_y, 60, 20)
  
  if ball_circle.overlaps(brick_rect):
    - Record collision
    - Determine entry side
    - Destroy brick
    - Spawn power-up (15% chance)
```

**Entry Side Determination (Most Reliable Method):**

```
Closest point on brick to ball center:
  closest_x = clamp(ball_x, brick.left, brick.right)
  closest_y = clamp(ball_y, brick.top, brick.bottom)

Entry vector:
  dx = ball_x - closest_x
  dy = ball_y - closest_y

Determine side:
  if abs(dx) > abs(dy):  // Horizontal entry
    if dx > 0: entry_side = RIGHT
    else: entry_side = LEFT
  else:  // Vertical entry
    if dy > 0: entry_side = BOTTOM
    else: entry_side = TOP
```

**Velocity Response:**

```
if entry_side in [TOP, BOTTOM]:
  vy = -vy  // Reverse vertical velocity

else if entry_side in [LEFT, RIGHT]:
  vx = -vx  // Reverse horizontal velocity
```

**Physics Edge Cases:**
- Corner hits: Use closest-point algorithm (prevents ambiguity)
- Multiple brick hits in one frame: Process leftmost brick only
- Ball stuck in brick (deep penetration): Allow 1 frame to exit

### 5.3 Power-Up Physics

**Motion:**

```
Power-up falls with constant velocity:
  powerup_y += 3 pixels/frame
```

**Pickup Detection:**

```
if powerup_rect.overlaps(paddle_rect):
  - Activate power-up
  - Remove from scene
  - Add to active power-ups list
```

---

## 6. Game States

### 6.1 State Machine

```
MAIN_MENU
  ↓ (Start Game)
PLAYING (Level 1-5)
  ├─ Ball in play
  ├─ Update physics
  ├─ Check win → LEVEL_COMPLETE or ALL_COMPLETE
  └─ Check lose life → LOST_LIFE
  
LOST_LIFE
  ├─ lives > 0 → Reset ball & PLAYING
  └─ lives = 0 → GAME_OVER

LEVEL_COMPLETE
  ├─ Current level < 5 → Transition to next PLAYING
  └─ Current level = 5 → ALL_COMPLETE

ALL_COMPLETE (Victory)
  ↓ (Show score)
GAME_OVER
  ├─ Show final score & high score
  ├─ (Restart) → MAIN_MENU
  └─ (Quit) → Exit
```

### 6.2 State Management

- **Active Level:** 1-5
- **Score:** Cumulative across all levels
- **Lives:** 3 (decrements on ball loss)
- **Paddle State:** Position, width (normal/extended)
- **Active Power-Ups:** List of (type, remaining_frames)
- **Ball(s):** Position, velocity, existence flag

---

## 7. User Interface (UI)

### 7.1 HUD (Heads-Up Display)

**In-Game HUD:**

```
[Lives: 3] [Score: 1500]                    [Level: 1/5]
┌─────────────────────────────────────────────────────────┐
│                    Game Area (800×600)                  │
│                                                          │
│   [Bricks Grid]                                         │
│                                                          │
│        [Ball]                                           │
│                                                          │
│                   [Paddle]                              │
│                                                          │
│ [Active Power-ups: M(45f) P(20f)]                      │
└─────────────────────────────────────────────────────────┘
```

**Elements:**
- **Lives:** Top-left, white text
- **Score:** Top-center, white text
- **Level:** Top-right, white text
- **Active Power-ups:** Bottom-left corner, shows icon + remaining frames
- **Game Area:** 800×600 black background

### 7.2 Screens

#### Main Menu

```
    BREAKOUT: CLASSIC REVIVAL
    
    High Score: 15,500
    
    [PLAY] [QUIT]
```

#### Level Complete Screen

```
    LEVEL 2 COMPLETE!
    
    Score: 2,500
    Lives: 2
    
    [NEXT LEVEL]
```

#### Game Over Screen

```
    GAME OVER
    
    Final Score: 8,750
    High Score: 15,500
    
    [PLAY AGAIN] [QUIT]
```

### 7.3 Colors

- **Background:** Black (#000000)
- **Ball:** Cyan (#00FFFF)
- **Paddle:** White (#FFFFFF)
- **Bricks:** Rainbow (6 rows)
  - Row 1: Red (#FF0000)
  - Row 2: Orange (#FF7F00)
  - Row 3: Yellow (#FFFF00)
  - Row 4: Green (#00FF00)
  - Row 5: Cyan (#00FFFF)
  - Row 6: Magenta (#FF00FF)
- **Text:** White (#FFFFFF)
- **Power-Ups:**
  - Multi-Ball: Gold (#FFD700)
  - Paddle Extend: Green (#00FF00)
  - Slow Time: Purple (#9933FF)

---

## 8. Architecture & Code Structure

### 8.1 Module Organization

```
src/
├── main.rs              // Entry point, main game loop
├── game.rs              // Game state & logic
├── physics.rs           // Collision detection & responses
├── paddle.rs            // Paddle logic
├── ball.rs              // Ball logic
├── brick.rs             // Brick grid & management
├── powerup.rs           // Power-up system
├── ui.rs                // Rendering & UI
├── level.rs             // Level definitions & loading
├── types.rs             // Shared data structures
└── constants.rs         // Game constants
```

### 8.2 Key Data Structures

```rust
// Game state
struct GameState {
    level: usize,
    score: u32,
    high_score: u32,
    lives: u8,
    game_phase: GamePhase,  // Playing, LevelComplete, GameOver
}

// Ball
struct Ball {
    x: f32, y: f32,
    vx: f32, vy: f32,
    radius: f32,
    active: bool,
}

// Paddle
struct Paddle {
    x: f32,
    width: f32,
    normal_width: f32,
    extended_width: f32,
}

// Brick
struct Brick {
    x: f32, y: f32,
    width: f32, height: f32,
    active: bool,
    color: Color,
}

// Power-up
struct PowerUp {
    x: f32, y: f32,
    power_type: PowerUpType,
    active: bool,
}

// Power-up type
enum PowerUpType {
    MultiBall,
    PaddleExtend,
    SlowTime,
}

// Active power-ups tracking
struct ActivePowerUp {
    power_type: PowerUpType,
    remaining_frames: usize,
}
```

### 8.3 Collision Module Design

```rust
pub fn check_ball_paddle_collision(
    ball: &mut Ball,
    paddle: &Paddle,
) -> bool;

pub fn check_ball_brick_collision(
    ball: &mut Ball,
    brick: &mut Brick,
) -> bool;

pub fn check_ball_wall_collision(ball: &mut Ball);

pub fn check_powerup_pickup(
    powerup: &PowerUp,
    paddle: &Paddle,
) -> bool;

// Helper: Calculate hit position on paddle
fn calculate_paddle_hit_position(
    ball_x: f32,
    paddle_x: f32,
    paddle_width: f32,
) -> f32;
```

### 8.4 Game Loop Structure

```rust
fn main() {
    let mut game = Game::new();
    
    loop {
        // Input
        handle_input(&mut game.paddle);
        
        // Update
        update_ball_position(&mut game.ball);
        check_collisions(&mut game);
        update_powerups(&mut game);
        check_win_lose(&mut game);
        
        // Render
        clear_screen();
        render_bricks(&game.bricks);
        render_paddle(&game.paddle);
        render_balls(&game.balls);
        render_powerups(&game.powerups);
        render_hud(&game);
        present_frame();
        
        // Timing
        frame_time = clock.tick(60);
    }
}
```

---

## 9. Implementation Roadmap

### Phase 1: Foundation (30%)

1. Set up Macroquad project structure
2. Implement basic rendering (paddle, ball, bricks)
3. Implement paddle input & movement
4. Implement ball movement & basic wall collisions

### Phase 2: Physics & Collisions (40%)

1. Implement ball-paddle collision with angle variation
2. Implement ball-brick collision detection
3. Implement brick destruction & point system
4. Test edge cases (corner hits, multiple collisions)

### Phase 3: Game Logic (20%)

1. Implement level system & progression
2. Implement lives & game over conditions
3. Implement power-up system
4. Implement state machine (menu, playing, game over)

### Phase 4: Polish (10%)

1. Implement UI/HUD rendering
2. Implement high score persistence
3. Add sound effects (optional)
4. Add particle effects (optional)
5. Test all 5 levels

---

## 10. Technical Specifications

### 10.1 Development Environment

**Language:** Rust 1.70+  
**Framework:** Macroquad 0.4+  
**Dependencies:**

```toml
[dependencies]
macroquad = "0.4"
```

### 10.2 Compilation & Performance

**Target:** Debug mode (fast iteration), Release mode (optimal performance)  
**Expected Frame Rate:** 60 FPS locked  
**Memory Usage:** <50 MB (minimal)  
**Build Time:** <10 seconds

### 10.3 Platform Support

- Windows 10+
- macOS 10.14+
- Linux (X11/Wayland)
- Web (via WASM)

---

## 11. Testing Strategy

### 11.1 Test Cases

| Category | Test Case | Expected Outcome |
|----------|-----------|------------------|
| **Collision** | Ball hits left wall | Velocity reverses horizontally |
| **Collision** | Ball hits paddle left edge | Bounces left (vx < 0) |
| **Collision** | Ball hits paddle center | Bounces straight up (vx ≈ 0) |
| **Collision** | Ball hits brick top | vy reverses, brick destroyed |
| **Physics** | Multiple balls active | All move independently |
| **Power-Ups** | Multi-ball pickup | 2 new balls spawn |
| **Power-Ups** | Paddle extend timeout | Paddle shrinks back |
| **Gameplay** | Destroy all bricks | Level complete, advance |
| **Gameplay** | Ball falls below paddle | Life decremented |
| **Gameplay** | 5 levels complete | Victory screen shown |

### 11.2 Edge Cases to Handle

1. **Ball stuck in brick:** Implement force exit on subsequent frames
2. **Multiple simultaneous collisions:** Process in priority order (paddle > walls > bricks)
3. **Paddle at screen edge:** Clamp position, prevent off-screen
4. **Power-up overlap:** Allow stacking, reset timers
5. **High velocity ball:** Implement sub-frame collision detection if needed

---

## 12. Future Enhancements (Out of Scope)

- Boss levels with special mechanics
- Difficulty modes (easy/normal/hard)
- Leaderboard (multiple saves)
- Sound effects & music
- Particle effects on collisions
- Mobile touch controls
- Achievements/badges
- Advanced brick types (steel, explosive)
- Curved paddle surface physics

---

## 13. Reference Materials

### Physics Inspiration

- Classic Breakout/Arkanoid arcade manuals
- 2D collision detection best practices
- Simple deterministic physics engines

### Rust Game Dev Resources

- Macroquad official documentation: https://docs.rs/macroquad/
- Macroquad examples: https://github.com/not-fl3/macroquad/tree/master/examples
- Rust Game Development: https://arewegameyet.rs/

---

## 14. Appendix: Quick Reference

### Keyboard Controls

| Input | Action |
|-------|--------|
| LEFT Arrow / A | Move paddle left |
| RIGHT Arrow / D | Move paddle right |
| SPACE | Start game / Pause (future) |
| ESC | Quit to menu |

### Game Constants Summary

- Screen: 800×600px
- Ball radius: 5px, speed: 4-7 px/f
- Paddle: 100px wide (150px extended), 15px tall
- Bricks: 60×20px, 12×6 grid
- Power-ups: 20×20px, 15% spawn chance, 60-frame duration
- Max balls: 3 simultaneous

---

**Document Version:** 1.0  
**Last Updated:** 2026-04-05  
**Status:** Complete - Ready for Implementation
