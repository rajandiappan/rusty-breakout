# Phase 5 Status

This document reflects the current state of the Phase 5 feature set as of 2026-04-08.

## Implemented

### Advanced Brick Types

- `BrickType::Frozen`
- `BrickType::Exploding`
- `BrickType::Steel`
- `BrickType::Regenerating`

Implemented behavior includes slowed balls on frozen hits, explosion chain reactions, multi-hit steel bricks, and delayed respawn for regenerating bricks.

### Level Coverage

- Levels 1-10 are implemented.
- Levels 6-10 already use the advanced brick variants.

### Visual Feedback

- Special brick glow effects are rendered.
- Steel bricks show damage visuals.
- Exploding bricks trigger screen flash feedback.
- Score popups, ball trails, and brick destruction particles are in place.

### Physics And Logic

- Ball slowdown from frozen bricks is implemented.
- Explosion chain detection is implemented.
- Regenerating brick timers are implemented.
- Collision and level progression logic for the existing Phase 5 content are wired through `game.rs`.

### Persistence And Meta Progress

- High score is persisted.
- Settings are persisted.
- Achievements are persisted.

## Deferred

### Brick Formation And Hazards

- Brick formation oscillation for a moving Level 8 is deferred.
- `BrickFormation` is not implemented in code.
- Black Hole gravity wells are deferred.
- Moving Bumpers are deferred.
- Fog of War is deferred.
- Screen Tilt is deferred.

### Particle Variants

- Dedicated explosion particle variants are deferred.
- Ice crystal effects are deferred.
- Crack particle effects are deferred.
- Ghost fade effects are deferred.

### Game State Extensions

- Hazard-specific `GameState` plumbing is deferred.
- Formation-tracking state for moving brick layouts is deferred.

## Recommended Next Steps

1. Add the deferred hazard and formation systems.
2. Expand the particle system with special effect variants.
3. Add UI indicators for any new hazard states.
4. Keep the README and backlog aligned with whatever lands next.
