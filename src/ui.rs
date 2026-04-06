use crate::constants::*;
use crate::types::{GameState, PowerUpType};
use macroquad::prelude::*;

pub fn render_game(state: &GameState) {
    // Clear background with theme color
    clear_background(state.theme_colors.background);

    // Render bricks
    for brick in &state.bricks {
        if brick.active {
            draw_rectangle(brick.x, brick.y, brick.width, brick.height, brick.color);
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
            let color = match powerup.power_type {
                PowerUpType::MultiBall => state.theme_colors.accent,
                PowerUpType::PaddleExtend => state.theme_colors.primary,
                PowerUpType::SlowTime => state.theme_colors.secondary,
                PowerUpType::Laser => CYAN,        // [NEW]
                PowerUpType::Shield => ORANGE,     // [NEW]
                PowerUpType::Bomb => RED,          // [NEW]
                PowerUpType::Magnetize => MAGENTA, // [NEW]
                PowerUpType::PaddleShrink => RED,  // [NEW] Power-down - red/dark color
            };
            draw_rectangle(
                powerup.x - POWERUP_WIDTH / 2.0,
                powerup.y,
                POWERUP_WIDTH,
                POWERUP_HEIGHT,
                color,
            );

            // Draw symbol indicator with border
            let symbol = match powerup.power_type {
                PowerUpType::MultiBall => (POWERUP_MULTIBALL_SYMBOL, POWERUP_MULTIBALL_LABEL),
                PowerUpType::PaddleExtend => (POWERUP_EXTEND_SYMBOL, POWERUP_EXTEND_LABEL),
                PowerUpType::SlowTime => (POWERUP_SLOWTIME_SYMBOL, POWERUP_SLOWTIME_LABEL),
                PowerUpType::Laser => (POWERUP_LASER_SYMBOL, POWERUP_LASER_LABEL), // [NEW]
                PowerUpType::Shield => (POWERUP_SHIELD_SYMBOL, POWERUP_SHIELD_LABEL), // [NEW]
                PowerUpType::Bomb => (POWERUP_BOMB_SYMBOL, POWERUP_BOMB_LABEL),    // [NEW]
                PowerUpType::Magnetize => (POWERUP_MAGNETIZE_SYMBOL, POWERUP_MAGNETIZE_LABEL), // [NEW]
                PowerUpType::PaddleShrink => (POWERUP_SHRINK_SYMBOL, POWERUP_SHRINK_LABEL), // [NEW]
            };

            // Draw border around power-up
            draw_rectangle_lines(
                powerup.x - POWERUP_WIDTH / 2.0,
                powerup.y,
                POWERUP_WIDTH,
                POWERUP_HEIGHT,
                2.0,
                state.theme_colors.text,
            );

            // Try to draw the Unicode symbol first, fallback to letter
            let symbol_text = symbol.0;
            let text_width = measure_text(symbol_text, None, 14, 1.0).width;
            draw_text(
                symbol_text,
                powerup.x - text_width / 2.0,
                powerup.y + 13.0,
                14.0,
                state.theme_colors.text,
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

    // Render active power-ups with symbols
    let powerup_y = SCREEN_HEIGHT - 30.0;
    for (i, active) in state.active_powerups.iter().enumerate() {
        let (symbol, _label) = match active.power_type {
            PowerUpType::MultiBall => (POWERUP_MULTIBALL_SYMBOL, POWERUP_MULTIBALL_LABEL),
            PowerUpType::PaddleExtend => (POWERUP_EXTEND_SYMBOL, POWERUP_EXTEND_LABEL),
            PowerUpType::SlowTime => (POWERUP_SLOWTIME_SYMBOL, POWERUP_SLOWTIME_LABEL),
            PowerUpType::Laser => (POWERUP_LASER_SYMBOL, POWERUP_LASER_LABEL), // [NEW]
            PowerUpType::Shield => (POWERUP_SHIELD_SYMBOL, POWERUP_SHIELD_LABEL), // [NEW]
            PowerUpType::Bomb => (POWERUP_BOMB_SYMBOL, POWERUP_BOMB_LABEL),    // [NEW]
            PowerUpType::Magnetize => (POWERUP_MAGNETIZE_SYMBOL, POWERUP_MAGNETIZE_LABEL), // [NEW]
            PowerUpType::PaddleShrink => (POWERUP_SHRINK_SYMBOL, POWERUP_SHRINK_LABEL), // [NEW] (shouldn't appear as timed)
        };
        let color = match active.power_type {
            PowerUpType::MultiBall => state.theme_colors.accent,
            PowerUpType::PaddleExtend => state.theme_colors.primary,
            PowerUpType::SlowTime => state.theme_colors.secondary,
            PowerUpType::Laser => CYAN,        // [NEW]
            PowerUpType::Shield => ORANGE,     // [NEW]
            PowerUpType::Bomb => RED,          // [NEW]
            PowerUpType::Magnetize => MAGENTA, // [NEW]
            PowerUpType::PaddleShrink => RED,  // [NEW] (shouldn't appear as timed)
        };

        // Draw symbol and timer
        let text = format!("{} {}", symbol, active.remaining_frames);
        draw_text(&text, 10.0, powerup_y - (i as f32 * 25.0), 18.0, color);
    }

    // Render pause overlay if paused
    if state.is_paused {
        render_pause_overlay(state);
    }

    // Render particle effects
    state.particle_system.render();
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
}
