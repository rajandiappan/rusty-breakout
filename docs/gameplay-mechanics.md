# Gameplay Mechanics

This document holds the detailed gameplay and implementation-facing notes that used to live in the README.

## Core Features

- Deterministic, frame-based ball physics
- 10 playable levels
- 8 power-up / power-down types
- 4 advanced brick types
- 3 difficulty modes
- 5 visual themes
- Persistent high score, settings, and achievements
- Procedural music, particle effects, and controller support

## Controls

| Key | Action |
|-----|--------|
| LEFT Arrow / A | Move paddle left |
| RIGHT Arrow / D | Move paddle right |
| SPACE | Start game / Play again |
| P | Pause / Resume during gameplay |
| T | Cycle themes |
| D | Cycle difficulty |
| M | Toggle music |
| `+` / `-` | Adjust SFX volume |
| ESC | Quit to menu / Exit game |
| F1 | Open dev menu when enabled |

Controller support is available through the gamepad input path.

## Difficulty Modes

| Mode | Ball Speed | Paddle Width | Lives | Power-Up Chance |
|------|-----------|--------------|-------|-----------------|
| Easy | 0.8x | 130px | 5 | 25% |
| Normal | 1.0x | 100px | 3 | 15% |
| Hard | 1.3x | 70px | 2 | 10% |

## Advanced Brick Types

- Frozen: slows the ball on hit
- Exploding: triggers local chain destruction
- Steel: requires multiple hits
- Regenerating: respawns after a delay

## Power-Ups And Power-Downs

- Multi-Ball: spawns 2 additional balls
- Paddle Extend: widens the paddle
- Slow Time: reduces ball speed temporarily
- Laser: fires shots upward from the paddle
- Shield: saves a falling ball once
- Bomb: destroys a local brick cluster
- Magnetize: sticks a ball to the paddle temporarily
- Paddle Shrink: shrinks the paddle

## Scoring

- Brick destroyed: +10 points
- Level completed: +1000 bonus points
- Full game completion: +5000 bonus points

## Game Flow

1. Main Menu: press `SPACE` to start
2. Playing: clear all active bricks
3. Level Complete: auto-advance after a short delay
4. Victory: clear all 10 levels
5. Game Over: lose all lives

## Physics Notes

- Ball movement is frame-based, not delta-time based
- Paddle bounce angle changes based on hit position
- Collision resolution uses closest-point checks
- Ball velocity is clamped to avoid extreme trajectories

## Rendering And Feedback

- Object-style pickup icons are used instead of simple glyphs
- Advanced bricks have distinct visual states
- Special brick types now also have distinct sound cues
- Particle feedback exists for impacts, destruction, and power-up events

## Dev / QA Menu

In debug builds, or with `BREAKOUT_DEV_TOOLS=1`, the dev menu is enabled.

- Open with `F1`
- Jump to any level
- Start a fresh run at a chosen level
- Grant power-ups
- Toggle infinite lives
- Clear active effects
- Refill/reset play state

## Code Structure

- [main.rs](/E:/Code/rusty-breakout/src/main.rs): entry point and game loop
- [game.rs](/E:/Code/rusty-breakout/src/game.rs): game flow, collisions, progression
- [physics.rs](/E:/Code/rusty-breakout/src/physics.rs): collision handling
- [level.rs](/E:/Code/rusty-breakout/src/level.rs): level generation
- [ui.rs](/E:/Code/rusty-breakout/src/ui.rs): rendering and HUD
- [audio.rs](/E:/Code/rusty-breakout/src/audio.rs): procedural audio and SFX
- [effects.rs](/E:/Code/rusty-breakout/src/effects.rs): particle effects
- [types.rs](/E:/Code/rusty-breakout/src/types.rs): shared game data structures

## Related Docs

- [windows-releases.md](/E:/Code/rusty-breakout/docs/windows-releases.md)
- [implementation_backlog.md](/E:/Code/rusty-breakout/docs/implementation_backlog.md)
- [phase_5_implementation_plan.md](/E:/Code/rusty-breakout/docs/phase_5_implementation_plan.md)
- [BREAKOUT_PRD.md](/E:/Code/rusty-breakout/BREAKOUT_PRD.md)
