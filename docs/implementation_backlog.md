# Implementation Backlog

This backlog reflects the current source tree, not the older README/PRD wording.

## Priority 1

- Adaptive procedural soundtrack polish
  - Extend the new tiered music system with smoother transitions and richer danger overlays.
  - Acceptance: levels 1, 4, and 8 audibly step up in intensity without hard cuts.
- Pickup visual redesign polish
  - Refine the new object-based pickup silhouettes for readability across all themes.
  - Acceptance: every pickup and the shrink power-down are immediately distinguishable during live gameplay.
- Built-in dev / QA flow polish
  - Expand the in-game dev menu so every level and major power-up path can be tested in seconds.
  - Acceptance: QA can jump to any level, grant power-ups, restart, refill, and skip progress gates without normal play.

## Priority 2

- Implement real Level 8 "Pendulum" behavior instead of reusing the static full-grid layout.
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

## Priority 3

- Refactor oversized gameplay logic out of `src/game.rs`.
- Replace placeholder modules:
  - `src/brick.rs`
  - `src/powerup.rs`
- Add broader automated tests around progression, persistence, dev tools, and achievement unlocking.
