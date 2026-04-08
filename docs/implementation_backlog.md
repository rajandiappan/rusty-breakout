# Implementation Backlog

This backlog reflects the current source tree, not the older README/PRD wording.

## Priority 1

- Persist high score across runs.
- Persist achievements across runs.
- Wire missing achievement triggers that already exist in `src/achievements.rs`.
- Fix end-of-run achievement checks that currently rely on outdated assumptions.

## Priority 2

- Update README to match the actual game scope:
  - 10 levels instead of 5
  - advanced brick types
  - expanded power-up set
  - gamepad support
- Document current save files and controls.
- Align PRD and phase docs with implemented gameplay.

## Priority 3

- Implement real Level 8 "Pendulum" behavior instead of reusing the full-grid layout.
- Add deferred Phase 5 environmental hazards:
  - Black Hole
  - Moving Bumpers
  - Fog of War
  - Screen Tilt
- Add deferred advanced particle variants for:
  - exploding bricks
  - frozen hits
  - steel damage
  - regenerating brick respawn

## Priority 4

- Refactor oversized gameplay logic out of `src/game.rs`.
- Replace placeholder modules:
  - `src/brick.rs`
  - `src/powerup.rs`
- Add broader automated tests around progression, persistence, and achievement unlocking.
