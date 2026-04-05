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

## 10. Architecture & Code Structure (Enhanced for Phase 2)

### 10.1 Module Organization (Updated)

```
src/
├── main.rs              // Entry point, main game loop
├── game.rs              // Game state & logic
├── physics.rs           // Collision detection & responses
├── paddle.rs            // Paddle logic
├── ball.rs              // Ball logic
├── brick.rs             // Brick grid & management
├── powerup.rs           // Power-up system
├── ui.rs                // Core UI rendering
├── level.rs             // Level definitions & loading
├── types.rs             // Shared data structures
├── constants.rs         // Game constants
│
├── systems/             // [NEW] Modular systems
│   ├── audio.rs         // Audio manager, SFX, music
│   ├── effects.rs       // Particle effects system
│   ├── achievements.rs  // Achievement tracking
│   ├── settings.rs      // Settings management
│   └── analytics.rs     // Statistics tracking
│
├── ui/                  // [NEW] Enhanced UI modules
│   ├── menu.rs          // Main menu screens
│   ├── hud.rs           // In-game HUD rendering
│   ├── settings_menu.rs // Settings UI
│   ├── achievements_ui.rs // Achievement gallery
│   ├── themes.rs        // Theme system & definitions
│   └── animations.rs    // UI animations
│
├── config/              // [NEW] Configuration
│   ├── settings.json    // User preferences
│   ├── theme_classic.json
│   ├── theme_dark.json
│   ├── theme_neon.json
│   ├── theme_crt.json
│   └── theme_minimal.json
│
└── assets/              // [NEW] Game assets
    ├── sounds/
    │   ├── paddle_hit.wav
    │   ├── brick_destroy.wav
    │   ├── powerup_collect.wav
    │   ├── level_complete.ogg
    │   └── music/
    │       ├── menu_theme.ogg
    │       ├── level_1_theme.ogg
    │       └── ...
    └── sprites/
        ├── paddle.png
        ├── balls.png
        ├── bricks.png
        └── particles.png
```

### 10.2 Key Data Structures (Extended)

```rust
// [NEW] Audio Manager
struct AudioManager {
    sfx_volume: f32,        // 0.0 - 1.0
    music_volume: f32,      // 0.0 - 1.0
    current_music: Option<Music>,
    sfx_enabled: bool,
    music_enabled: bool,
}

// [NEW] Settings
struct GameSettings {
    difficulty: Difficulty,
    theme: Theme,
    sfx_volume: f32,
    music_volume: f32,
    particle_effects: bool,
    screen_shake: bool,
    fullscreen: bool,
    show_fps: bool,
    colorblind_mode: ColorblindMode,
    control_scheme: ControlScheme,
}

// [NEW] Difficulty
enum Difficulty {
    Easy,
    Normal,
    Hard,
    Hardcore,
}

// [NEW] Achievement System
struct AchievementManager {
    achievements: HashMap<AchievementId, Achievement>,
    unlocked: HashSet<AchievementId>,
}

struct Achievement {
    id: AchievementId,
    name: String,
    description: String,
    progress: u32,
    target: u32,
    unlock_date: Option<DateTime>,
}

// [NEW] Statistics
struct PlayerStats {
    total_games: u32,
    wins: u32,
    total_playtime: Duration,
    total_score: u64,
    high_scores: Vec<HighScoreEntry>,
    best_completion_time: Option<Duration>,
}

// [NEW] Themes
struct Theme {
    name: String,
    colors: ThemeColors,
    fonts: ThemeFonts,
}

struct ThemeColors {
    background: Color,
    primary: Color,
    secondary: Color,
    accent: Color,
    text: Color,
    brick_palette: [Color; 6],
}

// Extended Game State
struct GameState {
    level: usize,
    score: u32,
    high_score: u32,
    lives: u8,
    game_phase: GamePhase,
    difficulty: Difficulty,      // [NEW]
    active_theme: Theme,          // [NEW]
    combo_counter: u32,           // [NEW]
    is_paused: bool,              // [NEW]
    settings: GameSettings,       // [NEW]
    achievements: AchievementManager, // [NEW]
    stats: PlayerStats,           // [NEW]
}

// [NEW] Particle System
struct ParticleSystem {
    particles: Vec<Particle>,
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    color: Color,
    size: f32,
}

// [NEW] Effects Manager
struct EffectsManager {
    particles: ParticleSystem,
    screen_shake_intensity: f32,
    screen_shake_duration: f32,
}
```

### 10.3 Extended Module Functions

```rust
// Audio Module
pub struct AudioManager {
    pub fn new() -> Self;
    pub fn load_music(path: &str) -> Result<Music>;
    pub fn load_sfx(path: &str) -> Result<SoundEffect>;
    pub fn play_sfx(name: &str);
    pub fn play_music(name: &str, loop_music: bool);
    pub fn set_sfx_volume(volume: f32);
    pub fn set_music_volume(volume: f32);
    pub fn stop_music();
    pub fn stop_all();
}

// Settings Module
pub struct SettingsManager {
    pub fn new() -> Self;
    pub fn load_from_file() -> Result<GameSettings>;
    pub fn save_to_file(&self) -> Result<()>;
    pub fn reset_to_defaults();
    pub fn set_difficulty(difficulty: Difficulty);
    pub fn set_theme(theme: Theme);
    pub fn get_difficulty_multipliers() -> DifficultyMultipliers;
}

// Achievement Module
pub struct AchievementManager {
    pub fn new() -> Self;
    pub fn check_achievement(event: AchievementEvent);
    pub fn unlock(id: AchievementId);
    pub fn is_unlocked(id: AchievementId) -> bool;
    pub fn get_progress(id: AchievementId) -> (u32, u32);
    pub fn increment_progress(id: AchievementId, amount: u32);
    pub fn get_all_achievements() -> Vec<Achievement>;
    pub fn save_achievements();
}

// Theme Module
pub struct ThemeManager {
    pub fn load_theme(name: &str) -> Result<Theme>;
    pub fn get_available_themes() -> Vec<String>;
    pub fn apply_theme(theme: Theme);
    pub fn get_current_theme() -> Theme;
}

// Analytics Module
pub struct AnalyticsManager {
    pub fn track_game_start(difficulty: Difficulty);
    pub fn track_game_end(score: u32, completed: bool, time: Duration);
    pub fn track_level_complete(level: usize, time: Duration);
    pub fn record_high_score(score: u32);
    pub fn get_statistics() -> PlayerStats;
}

// Effects Module
pub struct EffectsManager {
    pub fn emit_brick_explosion(position: Vec2, color: Color);
    pub fn emit_power_up_sparkles(position: Vec2);
    pub fn emit_collision_dust(position: Vec2, velocity: Vec2);
    pub fn screen_shake(intensity: f32, duration: f32);
    pub fn update(&mut self, dt: f32);
    pub fn render(&self);
}
```

### 10.4 Updated Game Loop

```rust
fn main() {
    let mut game = Game::new();
    let mut audio = AudioManager::new();
    let mut effects = EffectsManager::new();
    let mut achievements = AchievementManager::new();
    
    audio.play_music("menu_theme", true);
    
    loop {
        // State Management
        match game.state {
            GameState::MainMenu => {
                render_main_menu(&game);
                handle_menu_input(&mut game);
            }
            
            GameState::Settings => {
                render_settings_menu(&game);
                handle_settings_input(&mut game);
            }
            
            GameState::Playing => {
                // Input
                handle_gameplay_input(&mut game);
                
                if game.is_paused {
                    render_pause_menu(&game);
                    handle_pause_input(&mut game);
                } else {
                    // Update
                    update_ball_position(&mut game);
                    check_collisions(&mut game);
                    update_powerups(&mut game);
                    effects.update(dt);
                    check_win_lose(&mut game);
                    
                    // Track achievements
                    achievements.check_achievement(AchievementEvent::BrickDestroyed);
                    
                    // Render
                    render_game_scene(&game, &effects);
                    render_hud(&game);
                }
            }
            
            GameState::GameOver => {
                render_game_over_screen(&game);
                // Save stats and achievements
                achievements.save_achievements();
                handle_gameover_input(&mut game);
            }
        }
        
        present_frame();
        frame_time = clock.tick(60);  // 60 FPS cap
    }
}
```

### 10.5 Scalability Considerations

**For Future Expansion:**
- Plugin system for custom themes
- Scripting support for custom levels (Lua/WASM)
- Network module for online leaderboards
- Mobile input abstraction for touch controls
- Replay system infrastructure (recording frame data)
- Modding API for community content

---

## 9. Implementation Roadmap

### Phase 1: Foundation (Complete)

1. ✓ Set up Macroquad project structure
2. ✓ Implement basic rendering (paddle, ball, bricks)
3. ✓ Implement paddle input & movement
4. ✓ Implement ball movement & basic wall collisions
5. ✓ Implement ball-paddle collision with angle variation
6. ✓ Implement ball-brick collision detection
7. ✓ Implement brick destruction & point system
8. ✓ Implement level system & progression
9. ✓ Implement lives & game over conditions
10. ✓ Implement power-up system
11. ✓ Implement state machine (menu, playing, game over)
12. ✓ Implement UI/HUD rendering
13. ✓ Implement high score persistence
14. ✓ Test all 5 levels

**Status:** All core gameplay mechanics fully implemented and tested.

### Phase 2: Professional Polish & Enhancement (In Development)

This phase transforms Breakout from a functional prototype into a professional, polished game with enhanced visuals, audio, accessibility, and extended content.

#### 2.1 Enhanced Visuals & Graphics (30%)

**New Graphics Assets:**
- Professional sprite sheet for ball with rotational states
- Animated paddle with smooth extend/shrink transitions
- Particle effects system:
  - Brick explosion particles (5-8 per brick)
  - Ball impact dust clouds
  - Power-up collection sparkles
  - Level completion celebration particles
- Visual feedback for collision impacts
- Screen shake effects on major events (lives lost, level complete)
- Smooth fade transitions between screens

**Brick Enhancements:**
- 3D depth/shadow effect on bricks
- Crack animation when hit (pre-destruction state)
- Special "armor" bricks requiring 2 hits
- Boss bricks with health bars

**Color Themes:**
- **Classic:** Original 8-bit arcade palette (default)
- **Dark Mode:** Low-light gaming friendly (dark grays, accent colors)
- **Neon:** High-contrast cyberpunk aesthetic (black + bright neons)
- **Retro CRT:** Scanlines, glow, monitor curvature simulation
- **Minimalist:** Clean, flat design with sans-serif fonts

#### 2.2 Audio System (20%)

**Sound Effects:**
- Paddle hit: Crisp "plop" sound with pitch variation based on hit position
- Brick destroy: Satisfying "crunch" with frequency-swept audio
- Power-up pickup: Ascending tone progression
- Power-up expiration: Warning beep
- Level complete: Fanfare/victory jingle
- Game over: Descending tone

**Background Music:**
- Dynamic adaptive music system
- Level themes (5 different compositions)
- Menu theme with ambient background
- 2 alternate music tracks (toggle in settings)
- Music volume independent from SFX volume

**Audio Implementation:**
- Macroquad native audio or rodio crate integration
- Low-latency sound playback (<50ms delay)
- Spatial audio hints (optional panning for future expansion)

#### 2.3 Difficulty & Difficulty Modes (15%)

**Three Difficulty Levels:**

| Mode | Ball Speed Multiplier | Power-Up Frequency | Lives | Special Features |
|------|----------------------|-------------------|-------|------------------|
| **Easy** | 0.8x | 25% chance | 5 | Larger paddle (130px), Slower ball, More forgiving physics |
| **Normal** | 1.0x | 15% chance | 3 | Standard gameplay, Balanced challenge |
| **Hard** | 1.3x | 10% chance | 2 | Faster ball, Fewer power-ups, Tighter paddle (70px), Randomized brick patterns |

**Hardcore Mode (Unlock):**
- One life total
- No power-ups except slow-time
- All 5 levels with increased difficulty modifiers
- Leaderboard tracking
- Medal system (bronze/silver/gold based on score)

#### 2.4 Achievement & Progression System (15%)

**Achievement Categories:**

**Skill Achievements:**
- "Sharpshooter" - Destroy 100 bricks in a single level
- "Rapid Fire" - Complete a level in under 60 seconds
- "Perfect Clear" - Complete all 5 levels without losing a life
- "Speedrunner" - Beat game in under 5 minutes
- "Multi-Ball Master" - Keep 3 balls active for 30 consecutive seconds

**Collection Achievements:**
- "Power-Up Hoarder" - Collect 5 power-ups in one level
- "Lucky Break" - Trigger 3 power-ups in 5 seconds
- "Time Bender" - Use slow-time power-up 50 times total

**Exploration Achievements:**
- "Theme Collector" - Unlock all 5 color themes
- "Hidden Depths" - Discover secret Easter egg level
- "Hardcore Champion" - Complete hardcore mode

**Display:**
- Achievement UI in main menu (gallery view)
- Progress tracking (e.g., "Sharpshooter: 67/100")
- Unlock notifications (toast popup during gameplay)
- Badge icons for completed achievements

#### 2.5 Settings & Customization (10%)

**Settings Menu:**
```
┌─────────────────────────────────┐
│       GAME SETTINGS             │
├─────────────────────────────────┤
│ Difficulty:     [Easy ▼]        │
│ Theme:          [Classic ▼]     │
│ Music Volume:   [████░░░░░░] 60%│
│ SFX Volume:     [██████░░░░] 75%│
│ Particle Effect:[ON/OFF]        │
│ Screen Shake:   [ON/OFF]        │
│ Fullscreen:     [ON/OFF]        │
│ FPS Counter:    [ON/OFF]        │
│ Colorblind Mode:[OFF ▼]         │
│                                 │
│      [SAVE] [RESET] [BACK]      │
└─────────────────────────────────┘
```

**Settings Storage:**
- Persistent config file (JSON): `config/settings.json`
- Player preferences saved automatically
- Quick toggle hotkeys (e.g., M for mute, T for theme cycle)

**Accessibility Options:**
- Colorblind-friendly palettes (Deuteranopia, Protanopia, Tritanopia)
- High contrast mode
- Larger UI text option
- Input remapping (custom keybinds)
- Screen reader support (audio cues for menu items)

#### 2.6 Enhanced UI/UX (10%)

**Main Menu Redesign:**
```
╔════════════════════════════════════╗
║     BREAKOUT: ARCADE REVIVAL       ║
║                                    ║
║    🎮 Your Score: 125,450          ║
║    🏆 High Score: 1,845,230        ║
║                                    ║
║       [PLAY]  [SETTINGS]           ║
║       [ACHIEVEMENTS]  [QUIT]       ║
║                                    ║
║    Last Session: Level 3/5         ║
║    Play Time: 47h 23m              ║
╚════════════════════════════════════╝
```

**Pause Menu (NEW):**
- Pause with ESC or P key
- Resume, Settings, Main Menu options
- Music continues (faded)
- Game time continues counting in background

**HUD Improvements:**
- Combo counter (consecutive bricks without paddle miss)
- Real-time multiplier display
- Speed indicator (visual bar showing ball velocity)
- Adaptive UI scaling for different resolutions
- Smooth animations for score/life changes

**New Screens:**
- Settings menu with tabs (Audio, Video, Gameplay, Accessibility)
- Achievement gallery with filters
- Statistics screen (total plays, win rate, fastest time)
- Credits screen with team info and music credits

#### 2.7 Extended Level Content (10%)

**Expansion Pack (15 new levels):**
- Levels 6-10: Progressive difficulty beyond original 5
- Levels 11-15: Expert challenge levels
- Each with unique brick patterns and themes
- Boss levels with special enemy bricks

**Special Level Types:**

**Survival Levels (Unlockable):**
- Endless mode: Bricks spawn infinitely
- Brick rows advance downward over time
- Score accumulation until failure
- Leaderboard tracking

**Timed Challenge (Unlockable):**
- Destroy X bricks in Y seconds
- 5 difficulty tiers
- Daily challenge rotation

**Custom Level Editor (Future):**
- In-game level designer
- Save/load custom levels
- Share level codes with community

#### 2.8 Data & Analytics (5%)

**Statistics Tracking:**
```
Player Statistics:
├── Total Games: 156
├── Win Rate: 42.3%
├── Average Score: 54,230
├── Best Score: 285,150
├── Total Play Time: 47h 23m
├── Favorite Difficulty: Normal
├── Most Used Power-Up: Paddle Extend
└── Fastest Completion: 4:32
```

**High Score Leaderboard:**
- Local high scores (top 20 saved)
- Optional cloud sync (future: multiplayer leaderboard)
- Timestamps for each score entry
- Difficulty and settings recorded with each score

### Phase 3: Community & Multiplayer (Future)

- Online leaderboards
- Replay recording & sharing
- Competitive multiplayer modes
- Steam integration (achievements, stats)

### Phase 4: Monetization & Distribution (Future)

- Steam release
- App store (iOS/Android) deployment
- Free-to-play optional cosmetics
- DLC content packs (new level themes, paddle skins)

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

## 11. Testing Strategy (Enhanced for Phase 2)

### 11.1 Test Cases (Extended)

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
| **[NEW] Difficulty** | Easy mode selected | Ball speed -20%, paddle +30%, 5 lives |
| **[NEW] Difficulty** | Hard mode selected | Ball speed +30%, paddle -30%, 2 lives |
| **[NEW] Audio** | SFX plays on brick hit | Sound plays within 50ms |
| **[NEW] Audio** | Music loops seamlessly | No gap between loops |
| **[NEW] Themes** | Dark theme applied | UI colors update immediately |
| **[NEW] Achievements** | Unlock "Perfect Clear" | Achievement unlocked on 5-level win with no lives lost |
| **[NEW] Settings** | Save & load settings | Settings persist after restart |
| **[NEW] Pause** | Press P during gameplay | Game pauses, music fades |
| **[NEW] Pause** | Resume after pause | Game continues from exact state |
| **[NEW] Statistics** | Track playtime | Session duration recorded accurately |
| **[NEW] Particles** | Brick explosion | 5-8 particles spawn with correct physics |

### 11.2 Edge Cases to Handle

1. **Ball stuck in brick:** Implement force exit on subsequent frames
2. **Multiple simultaneous collisions:** Process in priority order (paddle > walls > bricks)
3. **Paddle at screen edge:** Clamp position, prevent off-screen
4. **Power-up overlap:** Allow stacking, reset timers
5. **High velocity ball:** Implement sub-frame collision detection if needed
6. **[NEW] Difficulty switching:** Validate difficulty multipliers apply correctly
7. **[NEW] Theme switching mid-game:** Ensure smooth transition without gameplay interruption
8. **[NEW] Audio playing:** No audio stack overflow with rapid SFX triggers
9. **[NEW] Save file corruption:** Graceful fallback to defaults
10. **[NEW] Achievement unlocks:** Prevent duplicate unlock notifications

### 11.3 Performance Benchmarks

- **Frame Time:** <16.67ms (60 FPS target)
- **Memory Usage:** <100 MB including all assets
- **Audio Latency:** <50ms between trigger and playback
- **Theme Switching:** <100ms transition time
- **Load Time:** <2 seconds from startup to main menu
- **Particle Count:** Maintain 60 FPS with 1000+ active particles

### 11.4 Compatibility Testing

- **OS:** Windows 10+, macOS 10.14+, Linux (X11/Wayland)
- **Screen Resolutions:** 1024×768 (minimum), 4K (maximum)
- **Input Devices:** Keyboard, Gamepad (optional future)
- **Audio Systems:** ALSA, PulseAudio, WASAPI, CoreAudio

---

## 12. Future Enhancements (Phase 3+)

### Phase 3: Community & Multiplayer (Future Milestone)

- **Online Leaderboards**
  - Cloud sync to backend server
  - Regional leaderboards
  - Friend comparisons
  
- **Replay System**
  - Record gameplay as compressed frame data
  - Playback with scrubbing
  - Share replay codes with community
  - Auto-upload highlight clips
  
- **Competitive Modes**
  - Local 2-player split-screen
  - Turn-based competitive levels
  - Versus mode with score racing
  
- **Social Features**
  - Steam integration (achievements, stats)
  - Discord Rich Presence
  - Twitch extension for streaming

### Phase 4: Monetization & Distribution (Future Milestone)

- **Platform Releases**
  - Steam release with achievements/stats
  - Epic Games Store
  - itch.io
  - App stores (iOS/Android via web wrapper)
  
- **Optional Cosmetics (Free-to-Play Optional)**
  - Paddle skins (character designs)
  - Ball trails & effects
  - Brick pack variations
  - Music track packs
  
- **DLC Content Packs**
  - "Galactic Breakout" - Space theme with 10 levels
  - "Retro Arcade" - Classic cabinet styling
  - "Modern Motion" - Contemporary aesthetics
  
- **Premium Battle Pass (Optional)**
  - Seasonal cosmetics unlocks
  - Exclusive challenge events
  - Premium currency earnable through gameplay

### Phase 5: Advanced Gameplay (Future Milestone)

- **Advanced Brick Types**
  - Steel bricks (2-3 hits to destroy)
  - Explosive bricks (chain destruction)
  - Frozen bricks (slow ball on impact)
  - Regenerating bricks (respawn after delay)
  
- **Advanced Power-Ups**
  - Laser beam paddle (destroys bricks in straight line)
  - Shield power-up (catch falling bricks)
  - Gravity well (attracts ball to center)
  - Time warp (rewind recent seconds)
  
- **Boss Levels**
  - Special boss bricks with health bars
  - Pattern-based attack sequences
  - Unique boss music & visuals
  - Unlock special medals on victory
  
- **Campaign/Story Mode** (Optional)
  - Light narrative between levels
  - Progressive difficulty curve
  - Unlockable secrets and Easter eggs

---

## 13. Out of Scope (For This Phase)

These features are interesting but deferred to maintain focus:

- Mobile touch controls (add later if needed)
- Cross-platform cloud save (requires backend infrastructure)
- Procedural level generation (algorithmic complexity)
- VR/AR support (requires specialized libraries)
- Advanced AI opponent (single-player focus)
- Web multiplayer (requires networking)
- Monetization system (premature for indie title)

## 14. Technical Implementation Details

### 14.1 Phase 2 Technology Stack

**Audio:**
- Library: Macroquad native audio or `rodio` crate for advanced control
- Format Support: WAV (SFX), OGG Vorbis (music for compression)
- Mixing: Support for simultaneous SFX + background music

**Graphics Enhancements:**
- Particle System: Custom Vec-based implementation
- Animations: Frame-interpolation based tweening
- Rendering: Layered (background → objects → UI → effects)

**Data Persistence:**
- Format: JSON for human readability and editability
- Locations:
  - `config/settings.json` - User preferences
  - `data/achievements.json` - Unlocked achievements
  - `data/highscores.json` - High score entries
  - `data/statistics.json` - Player statistics

**Dependencies (New for Phase 2):**
```toml
[dependencies]
macroquad = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"  // For timestamps
rand = "0.8"    // For random events
rodio = "0.17"  // Optional: advanced audio
```

### 14.2 Development Timeline Estimate

**Phase 2 Breakdown:**
- Graphics & Effects: 3-4 weeks
- Audio System: 2-3 weeks
- Difficulty & Settings: 1-2 weeks
- Achievements: 1-2 weeks
- UI/UX Overhaul: 2-3 weeks
- Extended Content: 2-3 weeks
- Testing & Polish: 2-3 weeks

**Total Phase 2:** 13-20 weeks (3-5 months)

---

## 15. Reference Materials

### Physics & Game Design

- Classic Breakout/Arkanoid arcade manuals
- 2D collision detection best practices (Real-Time Collision Detection, Christer Ericson)
- Game Feel: A Game Programmer's Guide to Virtual Sensation (Steve Swink)

### Rust Game Development

- Macroquad official: https://docs.rs/macroquad/
- Macroquad examples: https://github.com/not-fl3/macroquad/tree/master/examples
- Rust Game Development: https://arewegameyet.rs/
- Bevy Engine (alternative): https://bevyengine.org/

### Audio for Games

- Game Audio: Advanced Techniques for Interactive Audio (Andrew Phelps, Catherine C. Marshall)
- FMOD Studio: Professional audio middleware
- Wwise: Professional audio engine

### UI/UX Design

- Game UI by Example: https://lotterywest.wa.gov/
- Designing Better Games: https://www.nngroup.com/articles/gamification/

---

## 16. Appendix: Quick Reference

### Phase 1 (Complete) - Keyboard Controls

| Input | Action |
|-------|--------|
| LEFT Arrow / A | Move paddle left |
| RIGHT Arrow / D | Move paddle right |
| SPACE | Start game |
| ESC | Quit to menu |

### Phase 2 (New) - Keyboard Controls

| Input | Action |
|-------|--------|
| P / PAUSE | Pause/Resume game |
| M | Toggle music |
| T | Cycle through themes |
| + / - | Adjust volume |
| TAB | Show statistics |
| ESC | Return to menu (from any screen) |

### Game Constants Summary (Phase 1)

- **Screen:** 800×600px
- **Ball:** radius 5px, speed 4-7 px/f
- **Paddle:** 100px wide (150px extended), 15px tall
- **Bricks:** 60×20px, 12×6 grid
- **Power-ups:** 20×20px, 15% spawn chance, 60-frame duration
- **Lives:** 3 (easy: 5, hard: 2)
- **Max balls:** 3 simultaneous

### Phase 2 New Constants

- **Audio:** Music volume 0-100%, SFX volume 0-100%
- **Particles:** Up to 1000 concurrent particles
- **Difficulty Multipliers:**
  - Easy: Speed 0.8x, Lives +2, Paddle +30%
  - Normal: Speed 1.0x, Lives 3, Standard paddle
  - Hard: Speed 1.3x, Lives 2, Paddle -30%
- **Themes:** 5 color schemes, each with 8-10 colors
- **Achievements:** 15 total achievements to unlock

---

## 17. Success Criteria (Phase 2)

**Functional Requirements:**
- ✓ All Phase 1 features working flawlessly
- ✓ Audio system with music + SFX
- ✓ 3 difficulty modes with proper multipliers
- ✓ 15+ achievements with progress tracking
- ✓ 5 unique visual themes
- ✓ Pause/resume functionality
- ✓ Settings persistence
- ✓ Player statistics tracking
- ✓ Particle effects on key events

**Performance Requirements:**
- ✓ 60 FPS maintained under normal load
- ✓ <100 MB memory footprint
- ✓ <2 second startup time
- ✓ <50ms audio latency

**Quality Requirements:**
- ✓ Zero crashes in extended play sessions (8+ hours)
- ✓ All achievements validate correctly
- ✓ Save files never corrupt
- ✓ No audio glitches or crackling
- ✓ Smooth theme transitions

**Polish Requirements:**
- ✓ Professional main menu design
- ✓ Smooth UI animations
- ✓ Consistent visual style across all screens
- ✓ Accessible colorblind mode
- ✓ Responsive to all input methods

---

**Document Version:** 2.0 (Phase 1 Complete + Phase 2 Design)  
**Last Updated:** 2026-04-06  
**Phase 1 Status:** ✓ Complete - Ready for Extended Content  
**Phase 2 Status:** 📋 Design Document Complete - Ready for Implementation
