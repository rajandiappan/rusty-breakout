use crate::constants::*;
use crate::types::{Brick, BrickType};

pub fn create_level_bricks(level: usize) -> Vec<Brick> {
    let mut bricks = Vec::new();

    match level {
        1 => create_full_grid(&mut bricks),
        2 => create_alternating_rows(&mut bricks),
        3 => create_spiral(&mut bricks),
        4 => create_checkerboard(&mut bricks),
        5 => create_random(&mut bricks),
        6 => create_tundra(&mut bricks),
        7 => create_minefield(&mut bricks),
        8 => create_pendulum(&mut bricks),
        9 => create_fortress(&mut bricks),
        10 => create_chaos(&mut bricks),
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
                brick_type: BrickType::Normal,
                health: 0,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}

fn create_alternating_rows(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
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
                brick_type: BrickType::Normal,
                health: 0,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}

fn create_spiral(bricks: &mut Vec<Brick>) {
    let center_col = BRICK_COLS / 2;
    let center_row = BRICK_ROWS / 2;

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let dist_from_center = ((col as i32 - center_col as i32)
                .abs()
                .max((row as i32 - center_row as i32).abs()))
                as f32;

            if (dist_from_center as usize).is_multiple_of(2) {
                let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
                let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

                bricks.push(Brick {
                    x,
                    y,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                    color: BRICK_COLORS[row % BRICK_COLORS.len()],
                    brick_type: BrickType::Normal,
                    health: 0,
                    regen_timer: 0,
                    is_hit: false,
                });
            }
        }
    }
}

fn create_checkerboard(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
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
                    brick_type: BrickType::Normal,
                    health: 0,
                    regen_timer: 0,
                    is_hit: false,
                });
            }
        }
    }
}

fn create_random(bricks: &mut Vec<Brick>) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(12345);

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let hash = ((col * 73 + row * 97) as u64).wrapping_mul(12345);
            if hash % 10 < 6 {
                let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
                let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

                bricks.push(Brick {
                    x,
                    y,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                    color: BRICK_COLORS[row % BRICK_COLORS.len()],
                    brick_type: BrickType::Normal,
                    health: 0,
                    regen_timer: 0,
                    is_hit: false,
                });
            }
        }
    }
}

fn create_tundra(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            let is_corner =
                (row < 2 || row >= BRICK_ROWS - 2) && (col < 2 || col >= BRICK_COLS - 2);

            let brick_type = if is_corner {
                BrickType::Frozen
            } else {
                BrickType::Normal
            };
            let health = if brick_type == BrickType::Steel {
                STEEL_BRICK_HEALTH
            } else {
                0
            };

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color: if is_corner {
                    macroquad::color::Color {
                        r: 0.6,
                        g: 0.8,
                        b: 1.0,
                        a: 0.7,
                    }
                } else {
                    BRICK_COLORS[row % BRICK_COLORS.len()]
                },
                brick_type,
                health,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}

fn create_minefield(bricks: &mut Vec<Brick>) {
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            let is_exploding = (row + col) % 4 < 2;

            let brick_type = if is_exploding {
                BrickType::Exploding
            } else {
                BrickType::Steel
            };
            let health = if brick_type == BrickType::Steel {
                STEEL_BRICK_HEALTH
            } else {
                0
            };

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color: if is_exploding {
                    macroquad::color::Color {
                        r: 1.0,
                        g: 0.3,
                        b: 0.0,
                        a: 1.0,
                    }
                } else {
                    macroquad::color::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.55,
                        a: 1.0,
                    }
                },
                brick_type,
                health,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}

fn create_pendulum(bricks: &mut Vec<Brick>) {
    create_full_grid(bricks);
}

fn create_fortress(bricks: &mut Vec<Brick>) {
    let center_row = BRICK_ROWS / 2;
    let center_col = BRICK_COLS / 2;

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            let dist_from_center = ((col as i32 - center_col as i32).pow(2)
                + (row as i32 - center_row as i32).pow(2))
                as f32;

            let (brick_type, color, health) = if dist_from_center <= 2.0 {
                (
                    BrickType::Regenerating,
                    macroquad::color::Color {
                        r: 0.6,
                        g: 0.2,
                        b: 0.8,
                        a: 0.8,
                    },
                    0,
                )
            } else if dist_from_center <= 8.0 {
                (
                    BrickType::Steel,
                    macroquad::color::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.55,
                        a: 1.0,
                    },
                    2,
                )
            } else {
                (
                    BrickType::Steel,
                    macroquad::color::Color {
                        r: 0.4,
                        g: 0.4,
                        b: 0.45,
                        a: 1.0,
                    },
                    3,
                )
            };

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color,
                brick_type,
                health,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}

fn create_chaos(bricks: &mut Vec<Brick>) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(54321);

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = BRICK_START_X + col as f32 * (BRICK_WIDTH + BRICK_SPACING);
            let y = BRICK_START_Y + row as f32 * (BRICK_HEIGHT + BRICK_SPACING);

            let hash = ((col * 73 + row * 97) as u64).wrapping_mul(54321);
            let type_index = (hash % 100) as usize;

            let (brick_type, color, health) = if type_index < 20 {
                (
                    BrickType::Steel,
                    macroquad::color::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.55,
                        a: 1.0,
                    },
                    STEEL_BRICK_HEALTH,
                )
            } else if type_index < 35 {
                (
                    BrickType::Exploding,
                    macroquad::color::Color {
                        r: 1.0,
                        g: 0.3,
                        b: 0.0,
                        a: 1.0,
                    },
                    0,
                )
            } else if type_index < 50 {
                (
                    BrickType::Frozen,
                    macroquad::color::Color {
                        r: 0.6,
                        g: 0.8,
                        b: 1.0,
                        a: 0.7,
                    },
                    0,
                )
            } else if type_index < 60 {
                (
                    BrickType::Regenerating,
                    macroquad::color::Color {
                        r: 0.6,
                        g: 0.2,
                        b: 0.8,
                        a: 0.8,
                    },
                    0,
                )
            } else {
                (BrickType::Normal, BRICK_COLORS[row % BRICK_COLORS.len()], 0)
            };

            bricks.push(Brick {
                x,
                y,
                width: BRICK_WIDTH,
                height: BRICK_HEIGHT,
                active: true,
                color,
                brick_type,
                health,
                regen_timer: 0,
                is_hit: false,
            });
        }
    }
}
