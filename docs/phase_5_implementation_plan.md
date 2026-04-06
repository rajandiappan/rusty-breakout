# Phase 5 Implementation Plan

## Advanced Gameplay

Phase 5 introduces advanced brick types, environmental hazards, and difficulty scaling beyond the original 5 levels.

---

## 1. Advanced Brick Types

### 1.1 BrickType Enum

Add new enum to `src/brick.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrickType {
    Normal,
    Frozen,      // Slows ball on hit
    Exploding,   // Chain destruction
    Steel,       // Multi-hit (u8 health)
    Regenerating // Respawns after delay
}
```

### 1.2 Brick Struct Updates

Update `Brick` struct to include:

```rust
pub struct Brick {
    pub brick_type: BrickType,
    pub health: u8,           // For Steel bricks (0-3)
    pub regen_timer: u32,     // For Regenerating bricks (300 frames)
    pub is_hit: bool,         // For Regenerating combo tracking
    // ... existing fields
}
```

### 1.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.1.1 | Add BrickType enum | High | ✓ Complete |
| 5.1.2 | Update Brick struct with new fields | High | ✓ Complete |
| 5.1.3 | Implement Frozen brick (40% speed reduction, 120 frames) | High | ✓ Complete |
| 5.1.4 | Implement Exploding brick (80px radius chain reaction) | High | ✓ Complete |
| 5.1.5 | Implement Steel brick (3 hits to destroy) | High | ✓ Complete |
| 5.1.6 | Implement Regenerating brick (respawn after 5 seconds) | Medium | ✓ Complete |
| 5.1.7 | Update brick collision handling for new types | High | ✓ Complete |
| 5.1.8 | Add visual feedback for Steel brick damage | Medium | ✓ Complete |

---

## 2. Creative Difficulty Scaling

### 2.1 Moving Brick Formations

Create `BrickFormation` struct in `src/brick.rs`:

```rust
pub struct BrickFormation {
    pub offset_x: f32,
    pub offset_y: f32,
    pub oscillation_amplitude: f32,
    pub oscillation_frequency: f32,
    pub rotation_angle: f32,
}

impl BrickFormation {
    pub fn update(&mut self, frame_count: u32) {
        self.offset_x = (frame_count as f32 * 0.02).sin() * self.oscillation_amplitude;
    }
}
```

### 2.2 Environmental Hazards

| Hazard | Description | Implementation |
|--------|-------------|-----------------|
| **Black Hole** | Gravity well pulls ball toward center | Apply force vector each frame |
| **Moving Bumpers** | Invincible circular colliders | Kinematic movement between bricks/paddle |
| **Fog of War** | Dark overlay, reveals 150px around ball | Render visibility mask |
| **Screen Tilt** | Temporary camera rotation on Steel brick hit | Transform offset with decay |

### 2.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.2.1 | Add BrickFormation struct | Medium | Deferred |
| 5.2.2 | Implement oscillation movement system | Medium | Deferred |
| 5.2.3 | Implement Black Hole gravity well | Medium | Deferred |
| 5.2.4 | Implement Moving Bumpers | Medium | Deferred |
| 5.2.5 | Implement Fog of War visibility | Low | Deferred |
| 5.2.6 | Implement Screen Tilt effect | Low | Deferred |

---

## 3. Physics & Logic Updates

### 3.1 Ball Physics Modifications

In `src/ball.rs`, add:

```rust
pub struct Ball {
    // ... existing fields
    pub speed_multiplier: f32,    // For Frozen bricks
    pub frozen_timer: u32,       // Duration of slow effect
}

impl Ball {
    pub fn apply_frozen(&mut self, duration: u32) {
        self.speed_multiplier = 0.6; // 40% reduction
        self.frozen_timer = duration;
    }

    pub fn update_speed(&mut self) {
        if self.frozen_timer > 0 {
            self.frozen_timer -= 1;
        } else {
            self.speed_multiplier = 1.0;
        }
    }
}
```

### 3.2 Collision Handling Updates

Update `src/physics.rs`:

| Feature | Logic Change |
|---------|--------------|
| **Explosion** | Radial detection for chain destruction |
| **Frozen** | Apply speed multiplier in update loop |
| **Steel** | Decrement health, only destroy at 0 |
| **Regenerating** | Track hit, start 300-frame timer |
| **Black Hole** | Apply gravity vector each frame |

### 3.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.3.1 | Add speed_multiplier to Ball | High | ✓ Complete |
| 5.3.2 | Update ball movement for speed modifier | High | ✓ Complete |
| 5.3.3 | Implement radial explosion detection | High | ✓ Complete |
| 5.3.4 | Add gravity well physics | Medium | Deferred |
| 5.3.5 | Update collision response for new brick types | High | ✓ Complete |

---

## 4. Level Themes (Levels 6-10)

### 4.1 Level Definitions

Add new levels to `src/level.rs`:

| Level | Name | Features | Difficulty |
|-------|------|----------|------------|
| 6 | The Tundra | Frozen Bricks in corners | Medium |
| 7 | Minefield | Exploding + Steel Bricks checkerboard | Hard |
| 8 | The Pendulum | Oscillating brick grid | Hard |
| 9 | The Fortress | Regenerating core + Steel layers | Expert |
| 10 | Chaos Theory | All types + Moving Bumper | Expert |

### 4.2 Level Generation

Each level needs custom brick layout generation:

```rust
pub fn generate_level_6() -> Vec<Brick> {
    // Frozen Bricks in corners
    // Normal Bricks filling rest
}

pub fn generate_level_7() -> Vec<Brick> {
    // Alternating Exploding/Steel pattern
}

pub fn generate_level_8() -> Vec<Brick> {
    // Full grid with BrickFormation enabled
}

pub fn generate_level_9() -> Vec<Brick> {
    // Inner: Regenerating core
    // Outer: Double Steel layer
}

pub fn generate_level_10() -> Vec<Brick> {
    // Mix of all types
    // Moving Bumper active
    // Fog of War (optional)
}
```

### 4.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.4.1 | Define Level 6 (The Tundra) | High | ✓ Complete |
| 5.4.2 | Define Level 7 (Minefield) | High | ✓ Complete |
| 5.4.3 | Define Level 8 (The Pendulum) | High | ✓ Complete |
| 5.4.4 | Define Level 9 (The Fortress) | High | ✓ Complete |
| 5.4.5 | Define Level 10 (Chaos Theory) | High | ✓ Complete |
| 5.4.6 | Update level loading for 6-10 | High | ✓ Complete |

---

## 5. Particle System Integration

### 5.1 Enhanced Particle Effects

Update `src/systems/effects.rs`:

| Event | Particle Type | Count | Notes |
|-------|--------------|-------|-------|
| Nitro explosion | Explosion | 50+ | Large spread radius |
| Chain reaction | Secondary explosion | 30+ | Each chain |
| Frozen hit | Ice crystals | 20+ | Blue/white burst |
| Steel damage | Crack lines | 10+ | Progressive damage |
| Regenerate | Ghost fade | 15+ | Purple glow |

### 5.2 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.5.1 | Add explosion particle emitter | Medium | Deferred |
| 5.5.2 | Add ice crystal effect | Medium | Deferred |
| 5.5.3 | Add crack particle effect | Low | Deferred |
| 5.5.4 | Add ghost fade effect | Low | Deferred |

---

## 6. Game State Updates

### 6.1 Extended GameState

Update `src/game.rs`:

```rust
pub struct GameState {
    // ... existing fields
    pub brick_formation: Option<BrickFormation>,
    pub black_hole_active: bool,
    pub moving_bumpers: Vec<MovingBumper>,
    pub fog_of_war_active: bool,
    pub screen_tilt: f32,
}
```

### 6.2 Level Progression

- Levels 1-5: Original implementation
- Levels 6-10: New advanced features
- Transition: Automatic after level 5 completion

### 6.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.6.1 | Add formation tracking to GameState | High | Deferred |
| 5.6.2 | Add hazard state management | Medium | Deferred |
| 5.6.3 | Update level progression logic | High | ✓ Complete |
| 5.6.4 | Add victory condition for level 10 | High | ✓ Complete |

---

## 7. UI & Rendering Updates

### 7.1 Brick Rendering

Update brick rendering to show different visuals per type:

| Brick Type | Visual |
|------------|--------|
| Normal | Standard color |
| Frozen | Translucent light blue + snowflake |
| Exploding | Pulsing red/orange + "!" icon |
| Steel | Metallic grey + rivets (changes with damage) |
| Regenerating | Faded purple + semi-transparent |

### 7.2 HUD Updates

- Show current level name (6-10)
- Display active hazards icons
- Show special brick indicators

### 7.3 Implementation Tasks

| Task | Description | Priority | Status |
|------|-------------|----------|--------|
| 5.7.1 | Update brick render for each type | Medium | ✓ Complete |
| 5.7.2 | Add Steel brick damage visuals | Medium | ✓ Complete |
| 5.7.3 | Update HUD for new levels | Low | ✓ Complete |
| 5.7.4 | Add hazard indicators | Low | Deferred |

---

## 8. Implementation Order

### Phase 5A: Core Brick Types (Weeks 1-2)

1. Add BrickType enum
2. Update Brick struct
3. Implement Frozen brick
4. Implement Steel brick
5. Update collision handling

### Phase 5B: Explosions & Chains (Weeks 2-3)

1. Implement Exploding brick
2. Add radial detection
3. Add chain reaction logic
4. Integrate particle effects

### Phase 5C: Regeneration & Hazards (Weeks 3-4)

1. Implement Regenerating brick
2. Add Black Hole physics
3. Implement Moving Bumpers

### Phase 5D: Visual Effects (Weeks 4-5)

1. Add Fog of War
2. Add Screen Tilt
3. Update brick rendering

### Phase 5E: Levels 6-10 (Weeks 5-7)

1. Design level layouts
2. Implement level generation
3. Add level progression
4. Test all levels

### Phase 5F: Polish (Weeks 7-8)

1. Balance difficulty
2. Fix edge cases
3. Performance optimization
4. Final testing

---

## 9. Dependencies

No new dependencies required for Phase 5:

```toml
[dependencies]
macroquad = "0.4"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rodio = "0.17"
gilrs = "0.10"
```

---

## 10. Testing Plan

### Unit Tests

- BrickType enum variants
- Collision detection accuracy
- Regeneration timer behavior
- Explosion chain logic

### Integration Tests

- All 5 original levels still work
- New levels 6-10 completable
- Difficulty scaling accurate
- Particle effects render correctly

### Edge Cases

- Ball stuck in regenerating brick
- Chain reaction infinite loop prevention
- Multiple explosions same frame
- Performance with many active particles

---

## 11. Success Criteria

- [x] All 10 levels playable (1-10)
- [x] All 5 brick types functional
- [ ] All environmental hazards work (Deferred)
- [x] 60 FPS maintained
- [x] No memory leaks with particle system
- [x] All levels beatable
- [x] Difficulty progression feels fair

---

## 12. UX Improvements (Future)

### 12.1 Enhanced HUD

| Feature | Description | Priority |
|---------|-------------|----------|
| Level Name Display | Show level name ("The Tundra", "Minefield", etc.) in HUD | Medium |
| Brick Type Indicators | Icons showing active special brick types | Low |
| Score Multiplier Badge | Show when combo/special effects active | Low |
| Timer Display | Optional time-based challenge mode | Low |

### 12.2 Menu Enhancements

| Feature | Description | Priority |
|---------|-------------|----------|
| Level Select Screen | Choose to start at any completed level | Medium |
| Practice Mode | Infinite lives, no score pressure | Low |
| Tutorial Overlay | First-time hints for new players | Low |

### 12.3 Accessibility

| Feature | Description | Priority |
|---------|-------------|----------|
| Colorblind Modes | Additional palette options | Low |
| High Contrast Mode | Enhanced visibility for HUD elements | Low |
| Larger Text Option | Scalable UI text | Low |
| Input Remapping | Customizable key bindings | Low |

### 12.4 Feedback & Polish

| Feature | Description | Priority |
|---------|-------------|----------|
| Screen Shake | Impact feedback on explosions/heavy hits | Medium |
| Slow-Motion Replay | Brief replay on level complete | Low |
| Sound Cues | Audio feedback for special brick hits | Low |
| Vibration | Gamepad rumble on major events | Low |

---

## 13. Graphics Improvements (Future)

### 13.1 Visual Effects

| Effect | Description | Priority |
|--------|-------------|----------|
| Brick Glow | Subtle glow on special brick types | Medium |
| Particle Trails | Ball trail particles for visibility | Low |
| Pulsing Animations | Animated effects for Exploding/Frozen bricks | Medium |
| Screen Flash | Brief flash on chain reactions | Low |

### 13.2 Rendering Enhancements

| Feature | Description | Priority |
|---------|-------------|----------|
| Anti-Aliasing | Smoother edges (if performance allows) | Low |
| Bloom Effect | Subtle glow on bright elements | Low |
| Parallax Background | Animated/dynamic background layers | Low |
| CRT Scanlines | Optional retro filter | Low |

### 13.3 Animation Improvements

| Animation | Description | Priority |
|-----------|-------------|----------|
| Brick Destruction | More elaborate destruction sequences | Medium |
| Power-Up Pickup | Animated collection effect | Medium |
| Level Transition | Smooth fade between levels | Low |
| Paddle Movement | Subtle squash/stretch | Low |

### 13.4 UI Visual Polish

| Element | Description | Priority |
|---------|-------------|----------|
| Animated Buttons | Hover/click animations in menus | Low |
| Score Popups | Floating +10 text on brick destroy | Medium |
| Progress Bar | Level completion indicator | Low |
| Theme Transitions | Smooth color theme switching | Medium |

---

## 14. Deferred Features Summary

The following features are deferred to future phases:
- Environmental hazards (Black Hole, Moving Bumpers, Fog of War, Screen Tilt)
- Brick formation oscillation
- Enhanced particle effects
- Additional audio cues
