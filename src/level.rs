use crate::constants::*;
use crate::types::Brick;

pub fn create_level_bricks(level: usize) -> Vec<Brick> {
    let mut bricks = Vec::new();

    match level {
        1 => create_full_grid(&mut bricks),
        2 => create_alternating_rows(&mut bricks),
        3 => create_spiral(&mut bricks),
        4 => create_checkerboard(&mut bricks),
        5 => create_random(&mut bricks),
        _ => create_full_grid(&mut bricks),
    }

    bricks
}

fn create_full_grid(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color: BRICK_COLORS[row % BRICK_COLORS.len()],
            });
        }
    }
}

fn create_alternating_rows(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            // Skip every other row
            if row % 2 == 1 {
                continue;
            }

            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color: BRICK_COLORS[row % BRICK_COLORS.len()],
            });
        }
    }
}

fn create_spiral(bricks: &mut Vec<Brick>) {
    // Spiral pattern from outside to center
    let center_col = BRICK_COLS / 2;
    let center_row = BRICK_ROWS / 2;

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let dist_from_center = ((col as i32 - center_col as i32).abs().max((row as i32 - center_row as i32).abs())) as f32;

            // Only place bricks at spiral distances
            if dist_from_center as usize % 2 == 0 {
                let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
                let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

                bricks.push(Brick {
                    x,
                    y,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                    color: BRICK_COLORS[row % BRICK_COLORS.len()],
                });
            }
        }
    }
}

fn create_checkerboard(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            // Checkerboard pattern
            if (row + col) % 2 == 0 {
                let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
                let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

                bricks.push(Brick {
                    x,
                    y,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                    color: BRICK_COLORS[row % BRICK_COLORS.len()],
                });
            }
        }
    }
}

fn create_random(bricks: &mut Vec<Brick>) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    // Use a seeded random for consistency
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(12345); // Seed for reproducibility

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            // Pseudo-random placement (simplified)
            let hash = ((col * 73 + row * 97) as u64).wrapping_mul(12345);
            if hash % 10 < 6 {
                // 60% chance of brick
                let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
                let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

                bricks.push(Brick {
                    x,
                    y,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                    color: BRICK_COLORS[row % BRICK_COLORS.len()],
                });
            }
        }
    }
}
