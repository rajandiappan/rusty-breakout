use crate::constants::*;
use crate::types::{BrickType, GameState, PowerUpType};
use macroquad::prelude::*;

fn draw_corner_brackets(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    size: f32,
    thickness: f32,
    color: Color,
) {
    draw_line(x, y, x + size, y, thickness, color);
    draw_line(x, y, x, y + size, thickness, color);
    draw_line(x + width - size, y, x + width, y, thickness, color);
    draw_line(x + width, y, x + width, y + size, thickness, color);
    draw_line(x, y + height - size, x, y + height, thickness, color);
    draw_line(x, y + height, x + size, y + height, thickness, color);
    draw_line(
        x + width - size,
        y + height,
        x + width,
        y + height,
        thickness,
        color,
    );
    draw_line(
        x + width,
        y + height - size,
        x + width,
        y + height,
        thickness,
        color,
    );
}

fn powerup_color(power_type: PowerUpType) -> Color {
    match power_type {
        PowerUpType::MultiBall => crate::constants::GOLD,
        PowerUpType::PaddleExtend => GREEN,
        PowerUpType::SlowTime => Color::new(0.55, 0.35, 1.0, 1.0),
        PowerUpType::Laser => CYAN,
        PowerUpType::Shield => ORANGE,
        PowerUpType::Bomb => RED,
        PowerUpType::Magnetize => MAGENTA,
        PowerUpType::PaddleShrink => Color::new(0.7, 0.1, 0.1, 1.0),
    }
}

fn powerup_label(power_type: PowerUpType) -> &'static str {
    match power_type {
        PowerUpType::MultiBall => "Multi",
        PowerUpType::PaddleExtend => "Extend",
        PowerUpType::SlowTime => "Slow",
        PowerUpType::Laser => "Laser",
        PowerUpType::Shield => "Shield",
        PowerUpType::Bomb => "Bomb",
        PowerUpType::Magnetize => "Magnet",
        PowerUpType::PaddleShrink => "Shrink",
    }
}

fn draw_powerup_object(
    center_x: f32,
    center_y: f32,
    size: f32,
    power_type: PowerUpType,
    pulse: f32,
) {
    let base = powerup_color(power_type);
    let outline = Color::new(0.08, 0.08, 0.12, 0.95);
    let glow = Color::new(base.r, base.g, base.b, 0.14 + pulse * 0.12);
    draw_circle(center_x, center_y, size * 0.74, glow);

    match power_type {
        PowerUpType::MultiBall => {
            for (dx, dy) in [
                (-size * 0.22, size * 0.08),
                (size * 0.22, size * 0.08),
                (0.0, -size * 0.2),
            ] {
                draw_circle(center_x + dx, center_y + dy, size * 0.20, outline);
                draw_circle(center_x + dx, center_y + dy, size * 0.16, base);
                draw_circle(
                    center_x + dx - size * 0.05,
                    center_y + dy - size * 0.05,
                    size * 0.04,
                    WHITE,
                );
            }
        }
        PowerUpType::PaddleExtend => {
            draw_rectangle(
                center_x - size * 0.5,
                center_y - size * 0.14,
                size,
                size * 0.28,
                outline,
            );
            draw_rectangle(
                center_x - size * 0.44,
                center_y - size * 0.09,
                size * 0.88,
                size * 0.18,
                base,
            );
            draw_circle(center_x - size * 0.5, center_y, size * 0.14, outline);
            draw_circle(center_x + size * 0.5, center_y, size * 0.14, outline);
            draw_circle(center_x - size * 0.44, center_y, size * 0.09, base);
            draw_circle(center_x + size * 0.44, center_y, size * 0.09, base);
        }
        PowerUpType::SlowTime => {
            draw_circle(center_x, center_y, size * 0.34, outline);
            draw_circle(center_x, center_y, size * 0.28, base);
            draw_circle_lines(
                center_x,
                center_y,
                size * (0.44 + pulse * 0.08),
                2.0,
                Color::new(base.r, base.g, base.b, 0.85),
            );
            draw_line(
                center_x,
                center_y,
                center_x,
                center_y - size * 0.14,
                2.5,
                WHITE,
            );
            draw_line(
                center_x,
                center_y,
                center_x + size * 0.12,
                center_y + size * 0.05,
                2.5,
                WHITE,
            );
        }
        PowerUpType::Laser => {
            let top = vec2(center_x, center_y - size * 0.44);
            let left = vec2(center_x - size * 0.22, center_y + size * 0.28);
            let right = vec2(center_x + size * 0.22, center_y + size * 0.28);
            draw_triangle(top, left, right, outline);
            draw_triangle(
                vec2(center_x, center_y - size * 0.36),
                vec2(center_x - size * 0.14, center_y + size * 0.18),
                vec2(center_x + size * 0.14, center_y + size * 0.18),
                base,
            );
            draw_rectangle(
                center_x - size * 0.08,
                center_y + size * 0.18,
                size * 0.16,
                size * 0.2,
                WHITE,
            );
        }
        PowerUpType::Shield => {
            let top = vec2(center_x, center_y - size * 0.42);
            let left = vec2(center_x - size * 0.34, center_y - size * 0.12);
            let right = vec2(center_x + size * 0.34, center_y - size * 0.12);
            let bottom = vec2(center_x, center_y + size * 0.42);
            draw_triangle(top, left, right, outline);
            draw_triangle(right, left, bottom, outline);
            draw_triangle(
                vec2(center_x, center_y - size * 0.3),
                vec2(center_x - size * 0.24, center_y - size * 0.08),
                vec2(center_x + size * 0.24, center_y - size * 0.08),
                base,
            );
            draw_triangle(
                vec2(center_x + size * 0.24, center_y - size * 0.08),
                vec2(center_x - size * 0.24, center_y - size * 0.08),
                vec2(center_x, center_y + size * 0.28),
                base,
            );
        }
        PowerUpType::Bomb => {
            draw_circle(center_x, center_y, size * 0.3, outline);
            draw_circle(center_x, center_y, size * 0.24, base);
            draw_line(
                center_x + size * 0.1,
                center_y - size * 0.18,
                center_x + size * 0.26,
                center_y - size * 0.34,
                3.0,
                outline,
            );
            draw_circle(
                center_x + size * 0.28,
                center_y - size * 0.36,
                size * 0.06,
                YELLOW,
            );
        }
        PowerUpType::Magnetize => {
            draw_line(
                center_x - size * 0.22,
                center_y - size * 0.28,
                center_x - size * 0.22,
                center_y + size * 0.18,
                6.0,
                outline,
            );
            draw_line(
                center_x + size * 0.22,
                center_y - size * 0.28,
                center_x + size * 0.22,
                center_y + size * 0.18,
                6.0,
                outline,
            );
            draw_line(
                center_x - size * 0.22,
                center_y + size * 0.18,
                center_x + size * 0.22,
                center_y + size * 0.18,
                6.0,
                outline,
            );
            draw_line(
                center_x - size * 0.18,
                center_y - size * 0.24,
                center_x - size * 0.18,
                center_y + size * 0.12,
                5.0,
                base,
            );
            draw_line(
                center_x + size * 0.18,
                center_y - size * 0.24,
                center_x + size * 0.18,
                center_y + size * 0.12,
                5.0,
                base,
            );
            draw_line(
                center_x - size * 0.18,
                center_y + size * 0.12,
                center_x + size * 0.18,
                center_y + size * 0.12,
                5.0,
                base,
            );
            draw_rectangle(
                center_x - size * 0.28,
                center_y - size * 0.34,
                size * 0.12,
                size * 0.12,
                WHITE,
            );
            draw_rectangle(
                center_x + size * 0.16,
                center_y - size * 0.34,
                size * 0.12,
                size * 0.12,
                WHITE,
            );
        }
        PowerUpType::PaddleShrink => {
            draw_circle(center_x, center_y, size * 0.3, outline);
            draw_circle(center_x, center_y, size * 0.24, base);
            draw_line(
                center_x - size * 0.18,
                center_y - size * 0.08,
                center_x + size * 0.18,
                center_y + size * 0.12,
                2.5,
                WHITE,
            );
            draw_line(
                center_x + size * 0.02,
                center_y - size * 0.2,
                center_x - size * 0.08,
                center_y + size * 0.22,
                2.5,
                WHITE,
            );
            draw_line(
                center_x - size * 0.16,
                center_y + size * 0.16,
                center_x + size * 0.04,
                center_y - size * 0.18,
                2.0,
                Color::new(0.15, 0.0, 0.0, 1.0),
            );
        }
    }
}

pub fn render_game(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    // Render screen flash overlay (for chain reactions)
    if state.screen_flash > 0.0 {
        draw_rectangle(
            0.0,
            0.0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: state.screen_flash,
            },
        );
    }

    // Calculate pulsing factor based on frame count
    let pulse = ((state.frame_count as f32 * 0.05).sin() + 1.0) / 2.0; // 0.0 to 1.0 oscillating

    // Render bricks
    for brick in &state.bricks {
        if !brick.active && brick.brick_type != BrickType::Regenerating {
            continue;
        }

        let mut render_color = brick.color;
        let is_regen_cooling_down =
            brick.brick_type == BrickType::Regenerating && !brick.active && brick.regen_timer > 0;

        match brick.brick_type {
            BrickType::Steel => {
                let health_factor = brick.health.max(1) as f32 / 3.0;
                render_color = Color {
                    r: 0.25 + render_color.r * (0.35 + health_factor * 0.4),
                    g: 0.27 + render_color.g * (0.35 + health_factor * 0.4),
                    b: 0.32 + render_color.b * (0.4 + health_factor * 0.35),
                    a: 1.0,
                };
            }
            BrickType::Frozen => {
                render_color = Color::new(
                    (render_color.r * 0.65 + 0.28).min(1.0),
                    (render_color.g * 0.75 + 0.34).min(1.0),
                    (render_color.b * 0.85 + 0.42).min(1.0),
                    0.82,
                );
            }
            BrickType::Regenerating => {
                if is_regen_cooling_down {
                    render_color = Color::new(0.16, 0.07, 0.22, 0.32);
                } else {
                    render_color.a = 0.88;
                }
            }
            _ => {}
        }

        match brick.brick_type {
            BrickType::Frozen => {
                let glow_color = Color::new(0.55, 0.82, 1.0, 0.22 + pulse * 0.18);
                draw_rectangle(
                    brick.x - 4.0,
                    brick.y - 4.0,
                    brick.width + 8.0,
                    brick.height + 8.0,
                    glow_color,
                );
            }
            BrickType::Exploding => {
                let glow_alpha = 0.22 + pulse * 0.26;
                let glow_color = Color::new(1.0, 0.35, 0.0, glow_alpha);
                draw_rectangle(
                    brick.x - 4.0,
                    brick.y - 4.0,
                    brick.width + 8.0,
                    brick.height + 8.0,
                    glow_color,
                );
            }
            BrickType::Regenerating => {
                let glow_alpha = if is_regen_cooling_down {
                    0.1 + pulse * 0.14
                } else {
                    0.22 + pulse * 0.22
                };
                let glow_color = Color::new(0.7, 0.3, 0.9, glow_alpha);
                draw_rectangle(
                    brick.x - 4.0,
                    brick.y - 4.0,
                    brick.width + 8.0,
                    brick.height + 8.0,
                    glow_color,
                );
            }
            BrickType::Steel => {
                let damage_phase = (3_u8.saturating_sub(brick.health)).min(2) as f32;
                let glow_color = Color::new(
                    0.65 + damage_phase * 0.1,
                    0.68 + damage_phase * 0.08,
                    0.74 + damage_phase * 0.06,
                    0.18 + damage_phase * 0.07,
                );
                draw_rectangle(
                    brick.x - 3.0,
                    brick.y - 3.0,
                    brick.width + 6.0,
                    brick.height + 6.0,
                    glow_color,
                );
            }
            _ => {}
        }

        draw_rectangle(brick.x, brick.y, brick.width, brick.height, render_color);

        match brick.brick_type {
            BrickType::Frozen => {
                let border = Color::new(0.92, 0.98, 1.0, 0.92);
                draw_rectangle_lines(
                    brick.x + 1.0,
                    brick.y + 1.0,
                    brick.width - 2.0,
                    brick.height - 2.0,
                    2.0,
                    border,
                );
                draw_line(
                    brick.x + 5.0,
                    brick.y + brick.height * 0.28,
                    brick.x + brick.width - 8.0,
                    brick.y + brick.height * 0.62,
                    1.8,
                    Color::new(0.8, 0.96, 1.0, 0.85),
                );
                draw_line(
                    brick.x + brick.width * 0.45,
                    brick.y + 4.0,
                    brick.x + brick.width * 0.58,
                    brick.y + brick.height - 4.0,
                    1.6,
                    Color::new(0.72, 0.9, 1.0, 0.8),
                );
                draw_line(
                    brick.x + brick.width * 0.24,
                    brick.y + brick.height * 0.52,
                    brick.x + brick.width * 0.72,
                    brick.y + brick.height * 0.34,
                    1.4,
                    Color::new(0.9, 0.98, 1.0, 0.72),
                );
                draw_rectangle(
                    brick.x + 3.0,
                    brick.y + 3.0,
                    brick.width - 6.0,
                    4.0,
                    Color::new(1.0, 1.0, 1.0, 0.22 + pulse * 0.18),
                );
            }
            BrickType::Exploding => {
                let warning = Color::new(1.0, 0.86, 0.3, 0.9);
                draw_rectangle_lines(
                    brick.x + 1.0,
                    brick.y + 1.0,
                    brick.width - 2.0,
                    brick.height - 2.0,
                    2.0,
                    warning,
                );
                let center_x = brick.x + brick.width / 2.0;
                let center_y = brick.y + brick.height / 2.0;
                let core_radius = 4.0 + pulse * 2.2;
                draw_circle(
                    center_x,
                    center_y,
                    core_radius + 3.0,
                    Color::new(0.45, 0.05, 0.02, 0.55),
                );
                draw_circle(
                    center_x,
                    center_y,
                    core_radius,
                    Color::new(1.0, 0.88, 0.38, 0.95),
                );
                draw_line(
                    brick.x + 6.0,
                    center_y,
                    brick.x + brick.width - 6.0,
                    center_y,
                    1.8,
                    Color::new(1.0, 0.78, 0.32, 0.82),
                );
                draw_line(
                    center_x,
                    brick.y + 4.0,
                    center_x,
                    brick.y + brick.height - 4.0,
                    1.8,
                    Color::new(1.0, 0.7, 0.25, 0.82),
                );
                draw_rectangle_lines(
                    brick.x - 2.0,
                    brick.y - 2.0,
                    brick.width + 4.0,
                    brick.height + 4.0,
                    1.4,
                    Color::new(1.0, 0.55, 0.14, 0.28 + pulse * 0.18),
                );
            }
            BrickType::Steel => {
                let damage_stage = 3_u8.saturating_sub(brick.health.min(3));
                let border_color = match brick.health {
                    3 => Color::new(0.86, 0.9, 0.98, 0.9),
                    2 => Color::new(1.0, 0.85, 0.5, 0.95),
                    _ => Color::new(1.0, 0.45, 0.35, 0.98),
                };
                draw_rectangle_lines(
                    brick.x + 1.0,
                    brick.y + 1.0,
                    brick.width - 2.0,
                    brick.height - 2.0,
                    2.0,
                    border_color,
                );
                draw_corner_brackets(
                    brick.x + 2.0,
                    brick.y + 2.0,
                    brick.width - 4.0,
                    brick.height - 4.0,
                    6.0,
                    1.5,
                    Color::new(0.95, 0.98, 1.0, 0.65),
                );

                for i in 0..brick.health.min(3) {
                    let pip_x = brick.x + 7.0 + i as f32 * 9.0;
                    draw_circle(pip_x, brick.y + 7.0, 2.2, border_color);
                }

                if damage_stage > 0 {
                    let crack_color = Color::new(0.18, 0.2, 0.24, 0.95);
                    draw_line(
                        brick.x + 6.0,
                        brick.y + brick.height * 0.28,
                        brick.x + brick.width * 0.52,
                        brick.y + brick.height * 0.48,
                        2.0,
                        crack_color,
                    );
                    if damage_stage > 1 {
                        draw_line(
                            brick.x + brick.width * 0.42,
                            brick.y + 4.0,
                            brick.x + brick.width - 7.0,
                            brick.y + brick.height - 5.0,
                            2.5,
                            Color::new(0.35, 0.05, 0.05, 0.95),
                        );
                        draw_line(
                            brick.x + brick.width * 0.66,
                            brick.y + 5.0,
                            brick.x + brick.width * 0.54,
                            brick.y + brick.height - 5.0,
                            1.8,
                            Color::new(1.0, 0.82, 0.45, 0.75 + pulse * 0.2),
                        );
                    }
                }
            }
            BrickType::Regenerating => {
                let regen_progress =
                    1.0 - (brick.regen_timer as f32 / REGENERATING_DURATION as f32);
                let border = if is_regen_cooling_down {
                    Color::new(0.85, 0.5 + 0.3 * pulse, 1.0, 0.8)
                } else {
                    Color::new(0.95, 0.75, 1.0, 0.95)
                };
                draw_rectangle_lines(
                    brick.x + 1.0,
                    brick.y + 1.0,
                    brick.width - 2.0,
                    brick.height - 2.0,
                    2.0,
                    border,
                );

                if is_regen_cooling_down {
                    let fill_w = (brick.width - 6.0) * regen_progress.clamp(0.0, 1.0);
                    draw_rectangle(
                        brick.x + 3.0,
                        brick.y + brick.height - 6.0,
                        fill_w,
                        3.0,
                        Color::new(0.95, 0.65, 1.0, 0.95),
                    );
                    let ring_alpha = 0.22 + 0.22 * pulse;
                    draw_rectangle_lines(
                        brick.x - 2.0,
                        brick.y - 2.0,
                        brick.width + 4.0,
                        brick.height + 4.0,
                        1.5,
                        Color::new(0.85, 0.55, 1.0, ring_alpha),
                    );
                    if regen_progress > 0.7 {
                        let flash_alpha =
                            ((regen_progress - 0.7) / 0.3).clamp(0.0, 1.0) * (0.35 + pulse * 0.35);
                        draw_rectangle(
                            brick.x,
                            brick.y,
                            brick.width,
                            brick.height,
                            Color::new(1.0, 0.88, 1.0, flash_alpha),
                        );
                    }
                } else {
                    draw_line(
                        brick.x + 5.0,
                        brick.y + brick.height / 2.0,
                        brick.x + brick.width - 5.0,
                        brick.y + brick.height / 2.0,
                        2.0,
                        Color::new(0.98, 0.88, 1.0, 0.65),
                    );
                    draw_line(
                        brick.x + brick.width / 2.0,
                        brick.y + 4.0,
                        brick.x + brick.width / 2.0,
                        brick.y + brick.height - 4.0,
                        2.0,
                        Color::new(0.98, 0.88, 1.0, 0.65),
                    );
                }
            }
            _ => {}
        }
    }

    // Render balls
    for ball in &state.balls {
        if ball.active {
            draw_circle(ball.x, ball.y, ball.radius, state.theme_colors.ball);
        }
    }

    // Render paddle
    draw_rectangle(
        state.paddle.x,
        state.paddle.y,
        state.paddle.width,
        state.paddle.height,
        state.theme_colors.paddle,
    );

    // Render power-ups
    for powerup in &state.powerups {
        if powerup.active {
            let center_x = powerup.x;
            let center_y = powerup.y + POWERUP_HEIGHT / 2.0;
            draw_powerup_object(
                center_x,
                center_y,
                POWERUP_WIDTH * 0.8,
                powerup.power_type,
                pulse,
            );
        }
    }

    // [NEW] Render laser shots
    for laser in &state.laser_shots {
        if laser.active {
            draw_rectangle(laser.x, laser.y, laser.width, laser.height, CYAN);
        }
    }

    // Render HUD
    let hud_text = format!("Lives: {}", state.lives);
    draw_text(&hud_text, 10.0, 20.0, 24.0, state.theme_colors.text);

    let score_text = format!("Score: {}", state.score);
    let score_width = measure_text(&score_text, None, 24, 1.0).width;
    draw_text(
        &score_text,
        SCREEN_WIDTH / 2.0 - score_width / 2.0,
        20.0,
        24.0,
        state.theme_colors.text,
    );

    let level_text = format!("Level: {}/{}", state.level, NUM_LEVELS);
    let level_width = measure_text(&level_text, None, 24, 1.0).width;
    draw_text(
        &level_text,
        SCREEN_WIDTH - level_width - 10.0,
        20.0,
        24.0,
        state.theme_colors.text,
    );

    // Render active power-ups with object icons
    let powerup_y = SCREEN_HEIGHT - 30.0;
    for (i, active) in state.active_powerups.iter().enumerate() {
        let row_y = powerup_y - (i as f32 * 28.0);
        draw_powerup_object(18.0, row_y - 6.0, 12.0, active.power_type, pulse);
        let timer_text = format!(
            "{} {}",
            powerup_label(active.power_type),
            active.remaining_frames
        );
        draw_text(
            &timer_text,
            34.0,
            row_y,
            18.0,
            powerup_color(active.power_type),
        );
    }

    let mut status_row = state.active_powerups.len();
    if state.paddle.is_extended {
        let row_y = powerup_y - (status_row as f32 * 28.0);
        draw_powerup_object(18.0, row_y - 6.0, 12.0, PowerUpType::PaddleExtend, pulse);
        draw_text(
            "Extend",
            34.0,
            row_y,
            18.0,
            powerup_color(PowerUpType::PaddleExtend),
        );
        status_row += 1;
    }
    if state.paddle.is_shrunk {
        let row_y = powerup_y - (status_row as f32 * 28.0);
        draw_powerup_object(18.0, row_y - 6.0, 12.0, PowerUpType::PaddleShrink, pulse);
        draw_text(
            "Shrink",
            34.0,
            row_y,
            18.0,
            powerup_color(PowerUpType::PaddleShrink),
        );
        status_row += 1;
    }
    if state.paddle.shield_count > 0 {
        let row_y = powerup_y - (status_row as f32 * 28.0);
        draw_powerup_object(18.0, row_y - 6.0, 12.0, PowerUpType::Shield, pulse);
        let text = format!("Shield x{}", state.paddle.shield_count);
        draw_text(&text, 34.0, row_y, 18.0, powerup_color(PowerUpType::Shield));
    }

    // Render pause overlay if paused
    if state.is_paused {
        render_pause_overlay(state);
    }

    // Render score popups
    for popup in &state.score_popups {
        let alpha = popup.lifetime / popup.max_lifetime;
        let text = format!("+{}", popup.value);
        draw_text(
            &text,
            popup.x - 10.0,
            popup.y,
            20.0,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: alpha,
            },
        );
    }

    // Render particle effects
    state.particle_system.render();

    if state.dev_tools.open {
        render_dev_menu(state);
    }
}

pub fn render_pause_overlay(state: &GameState) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        },
    );

    // Pause text
    let pause_text = "PAUSED";
    let pause_width = measure_text(pause_text, None, 48, 1.0).width;
    draw_text(
        pause_text,
        SCREEN_WIDTH / 2.0 - pause_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 50.0,
        48.0,
        state.theme_colors.text,
    );

    // Theme info
    let theme_text = format!("Theme: {}", state.current_theme.as_str());
    let theme_width = measure_text(&theme_text, None, 20, 1.0).width;
    draw_text(
        &theme_text,
        SCREEN_WIDTH / 2.0 - theme_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 20.0,
        20.0,
        state.theme_colors.primary,
    );

    // Instructions
    let resume_text = "Press P to Resume";
    let resume_width = measure_text(resume_text, None, 20, 1.0).width;
    draw_text(
        resume_text,
        SCREEN_WIDTH / 2.0 - resume_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 60.0,
        20.0,
        state.theme_colors.text,
    );

    let theme_switch_text = "Press T to Change Theme";
    let ts_width = measure_text(theme_switch_text, None, 20, 1.0).width;
    draw_text(
        theme_switch_text,
        SCREEN_WIDTH / 2.0 - ts_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 85.0,
        20.0,
        state.theme_colors.text,
    );

    // Volume control display
    let volume_pct = (state.audio.get_volume() * 100.0) as u32;
    let volume_text = format!("Volume: {}% (±/-)", volume_pct);
    let vol_width = measure_text(&volume_text, None, 18, 1.0).width;
    draw_text(
        &volume_text,
        SCREEN_WIDTH / 2.0 - vol_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 115.0,
        18.0,
        state.theme_colors.primary,
    );

    // Mute toggle display
    let mute_text = if state.audio.sfx_enabled {
        "Press M to Mute"
    } else {
        "Press M to Unmute"
    };
    let mute_width = measure_text(mute_text, None, 18, 1.0).width;
    draw_text(
        mute_text,
        SCREEN_WIDTH / 2.0 - mute_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 140.0,
        18.0,
        state.theme_colors.text,
    );

    if state.dev_tools.enabled {
        let dev_text = "F1: Dev Menu";
        let dev_width = measure_text(dev_text, None, 16, 1.0).width;
        draw_text(
            dev_text,
            SCREEN_WIDTH / 2.0 - dev_width / 2.0,
            SCREEN_HEIGHT / 2.0 + 168.0,
            16.0,
            state.theme_colors.secondary,
        );
    }
}

pub fn render_main_menu(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    let title = "BREAKOUT: CLASSIC REVIVAL";
    let title_width = measure_text(title, None, 48, 1.0).width;
    draw_text(
        title,
        SCREEN_WIDTH / 2.0 - title_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 100.0,
        48.0,
        state.theme_colors.primary,
    );

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(
        &high_score_text,
        SCREEN_WIDTH / 2.0 - hs_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 20.0,
        32.0,
        state.theme_colors.text,
    );

    let play_text = "Press SPACE to Play";
    let play_width = measure_text(play_text, None, 28, 1.0).width;
    draw_text(
        play_text,
        SCREEN_WIDTH / 2.0 - play_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 50.0,
        28.0,
        state.theme_colors.accent,
    );

    let quit_text = "Press ESC to Quit";
    let quit_width = measure_text(quit_text, None, 20, 1.0).width;
    draw_text(
        quit_text,
        SCREEN_WIDTH / 2.0 - quit_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 100.0,
        20.0,
        state.theme_colors.text,
    );

    let theme_text = format!("Current Theme: {}", state.current_theme.as_str());
    let theme_width = measure_text(&theme_text, None, 16, 1.0).width;
    draw_text(
        &theme_text,
        SCREEN_WIDTH / 2.0 - theme_width / 2.0,
        SCREEN_HEIGHT - 30.0,
        16.0,
        state.theme_colors.secondary,
    );

    if state.dev_tools.enabled {
        let dev_text = "F1: Dev Menu";
        let dev_width = measure_text(dev_text, None, 16, 1.0).width;
        draw_text(
            dev_text,
            SCREEN_WIDTH / 2.0 - dev_width / 2.0,
            SCREEN_HEIGHT - 55.0,
            16.0,
            state.theme_colors.secondary,
        );
    }

    if state.dev_tools.open {
        render_dev_menu(state);
    }
}

pub fn render_level_complete(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    let level_text = format!("LEVEL {} COMPLETE!", state.level);
    let level_width = measure_text(&level_text, None, 40, 1.0).width;
    draw_text(
        &level_text,
        SCREEN_WIDTH / 2.0 - level_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 50.0,
        40.0,
        state.theme_colors.primary,
    );

    let score_text = format!("Score: {}", state.score);
    let score_width = measure_text(&score_text, None, 32, 1.0).width;
    draw_text(
        &score_text,
        SCREEN_WIDTH / 2.0 - score_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 20.0,
        32.0,
        state.theme_colors.text,
    );

    if state.dev_tools.open {
        render_dev_menu(state);
    }
}

pub fn render_game_over(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    let game_over_text = "GAME OVER";
    let go_width = measure_text(game_over_text, None, 48, 1.0).width;
    draw_text(
        game_over_text,
        SCREEN_WIDTH / 2.0 - go_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 80.0,
        48.0,
        state.theme_colors.accent,
    );

    let final_score_text = format!("Final Score: {}", state.score);
    let fs_width = measure_text(&final_score_text, None, 32, 1.0).width;
    draw_text(
        &final_score_text,
        SCREEN_WIDTH / 2.0 - fs_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 10.0,
        32.0,
        state.theme_colors.text,
    );

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(
        &high_score_text,
        SCREEN_WIDTH / 2.0 - hs_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 40.0,
        32.0,
        state.theme_colors.text,
    );

    let restart_text = "Press SPACE to Play Again";
    let restart_width = measure_text(restart_text, None, 24, 1.0).width;
    draw_text(
        restart_text,
        SCREEN_WIDTH / 2.0 - restart_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 100.0,
        24.0,
        state.theme_colors.primary,
    );

    if state.dev_tools.enabled {
        let dev_text = "F1: Dev Menu";
        let dev_width = measure_text(dev_text, None, 16, 1.0).width;
        draw_text(
            dev_text,
            SCREEN_WIDTH / 2.0 - dev_width / 2.0,
            SCREEN_HEIGHT / 2.0 + 135.0,
            16.0,
            state.theme_colors.secondary,
        );
    }

    if state.dev_tools.open {
        render_dev_menu(state);
    }
}

pub fn render_victory(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    let victory_text = "VICTORY!";
    let victory_width = measure_text(victory_text, None, 48, 1.0).width;
    draw_text(
        victory_text,
        SCREEN_WIDTH / 2.0 - victory_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 80.0,
        48.0,
        state.theme_colors.primary,
    );

    let final_score_text = format!("Final Score: {}", state.score);
    let fs_width = measure_text(&final_score_text, None, 32, 1.0).width;
    draw_text(
        &final_score_text,
        SCREEN_WIDTH / 2.0 - fs_width / 2.0,
        SCREEN_HEIGHT / 2.0 - 10.0,
        32.0,
        state.theme_colors.text,
    );

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(
        &high_score_text,
        SCREEN_WIDTH / 2.0 - hs_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 40.0,
        32.0,
        state.theme_colors.text,
    );

    let restart_text = "Press SPACE to Play Again";
    let restart_width = measure_text(restart_text, None, 24, 1.0).width;
    draw_text(
        restart_text,
        SCREEN_WIDTH / 2.0 - restart_width / 2.0,
        SCREEN_HEIGHT / 2.0 + 100.0,
        24.0,
        state.theme_colors.accent,
    );

    if state.dev_tools.enabled {
        let dev_text = "F1: Dev Menu";
        let dev_width = measure_text(dev_text, None, 16, 1.0).width;
        draw_text(
            dev_text,
            SCREEN_WIDTH / 2.0 - dev_width / 2.0,
            SCREEN_HEIGHT / 2.0 + 135.0,
            16.0,
            state.theme_colors.secondary,
        );
    }

    if state.dev_tools.open {
        render_dev_menu(state);
    }
}

pub fn render_dev_menu(state: &GameState) {
    let panel_x = SCREEN_WIDTH / 2.0 - 220.0;
    let panel_y = SCREEN_HEIGHT / 2.0 - 170.0;
    let panel_w = 440.0;
    let panel_h = 340.0;

    draw_rectangle(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        Color::new(0.03, 0.05, 0.08, 0.92),
    );
    draw_rectangle_lines(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        3.0,
        state.theme_colors.accent,
    );

    draw_text(
        "DEV MENU",
        panel_x + 20.0,
        panel_y + 34.0,
        30.0,
        state.theme_colors.text,
    );
    draw_text(
        "Arrows/WASD to navigate, Enter to apply, Esc/F1 to close",
        panel_x + 20.0,
        panel_y + 58.0,
        16.0,
        state.theme_colors.secondary,
    );

    let phase_label = match state.phase {
        crate::types::GamePhase::MainMenu => "Main Menu",
        crate::types::GamePhase::Playing => {
            if state.is_paused {
                "Paused Gameplay"
            } else {
                "Playing"
            }
        }
        crate::types::GamePhase::LevelComplete => "Level Complete",
        crate::types::GamePhase::GameOver => "Game Over",
        crate::types::GamePhase::Victory => "Victory",
    };
    draw_text(
        &format!("Context: {}", phase_label),
        panel_x + 20.0,
        panel_y + 78.0,
        16.0,
        state.theme_colors.secondary,
    );

    let powerups = [
        PowerUpType::MultiBall,
        PowerUpType::PaddleExtend,
        PowerUpType::SlowTime,
        PowerUpType::Laser,
        PowerUpType::Shield,
        PowerUpType::Bomb,
        PowerUpType::Magnetize,
        PowerUpType::PaddleShrink,
    ];
    let selected_powerup = powerups[state.dev_tools.selected_powerup_index];
    let rows = [
        format!(
            "Target Level: {} (Enter = start/jump)",
            state.dev_tools.selected_level
        ),
        format!("Grant Power-Up: {}", powerup_label(selected_powerup)),
        format!(
            "Infinite Lives: {}",
            if state.dev_tools.infinite_lives {
                "ON"
            } else {
                "OFF"
            }
        ),
        "Jump To Selected Level".to_string(),
        "Start Fresh Run At Selected Level".to_string(),
        "Restart Current Level".to_string(),
        "Apply Selected Power-Up".to_string(),
        "Clear Active Power-Ups".to_string(),
        if state.phase == crate::types::GamePhase::LevelComplete {
            "Skip Level Complete Timer".to_string()
        } else {
            "Refill Ball And Reset Paddle".to_string()
        },
    ];

    let availability = [
        true,
        true,
        true,
        matches!(
            state.phase,
            crate::types::GamePhase::Playing | crate::types::GamePhase::LevelComplete
        ),
        true,
        state.phase == crate::types::GamePhase::Playing,
        state.phase == crate::types::GamePhase::Playing,
        state.phase == crate::types::GamePhase::Playing,
        state.phase == crate::types::GamePhase::Playing
            || state.phase == crate::types::GamePhase::LevelComplete,
    ];

    for (index, row) in rows.iter().enumerate() {
        let y = panel_y + 104.0 + index as f32 * 24.0;
        if index == state.dev_tools.selected_row {
            draw_rectangle(
                panel_x + 14.0,
                y - 18.0,
                panel_w - 28.0,
                24.0,
                Color::new(
                    state.theme_colors.accent.r,
                    state.theme_colors.accent.g,
                    state.theme_colors.accent.b,
                    0.18,
                ),
            );
        }
        draw_text(
            row,
            panel_x + 24.0,
            y,
            18.0,
            if index == state.dev_tools.selected_row {
                if availability[index] {
                    state.theme_colors.accent
                } else {
                    Color::new(0.9, 0.5, 0.5, 1.0)
                }
            } else if availability[index] {
                state.theme_colors.text
            } else {
                Color::new(
                    state.theme_colors.secondary.r,
                    state.theme_colors.secondary.g,
                    state.theme_colors.secondary.b,
                    0.65,
                )
            },
        );
    }

    let help_text = match state.dev_tools.selected_row {
        0 => "Apply the selected level immediately in the current context.",
        1 => "Choose which power-up the dev menu will grant during gameplay.",
        2 => "Prevents life loss while testing gameplay.",
        3 => "Jump to the selected level from paused play or the level-complete screen.",
        4 => "Start a fresh run at the selected level with score/lives reset.",
        5 => "Reload the current level and reset its state.",
        6 => "Grant the selected power-up instantly during gameplay.",
        7 => "Clear active power-up effects and pickup drops during gameplay.",
        8 => {
            if state.phase == crate::types::GamePhase::LevelComplete {
                "Skip the level-complete timer and continue immediately."
            } else {
                "Restore a clean ball+paddle state without changing the level."
            }
        }
        _ => "",
    };
    draw_text(
        help_text,
        panel_x + 20.0,
        panel_y + panel_h - 26.0,
        16.0,
        state.theme_colors.secondary,
    );
}
