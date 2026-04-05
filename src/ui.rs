use crate::constants::*;
use crate::types::{GameState, PowerUpType};
use macroquad::prelude::*;

pub fn render_game(state: &GameState) {
    // Render bricks
    for brick in &state.bricks {
        if brick.active {
            draw_rectangle(brick.x, brick.y, brick.width, brick.height, brick.color);
        }
    }

    // Render balls
    for ball in &state.balls {
        if ball.active {
            draw_circle(ball.x, ball.y, ball.radius, CYAN);
        }
    }

    // Render paddle
    draw_rectangle(state.paddle.x, state.paddle.y, state.paddle.width, state.paddle.height, WHITE);

    // Render power-ups
    for powerup in &state.powerups {
        if powerup.active {
            let color = match powerup.power_type {
                PowerUpType::MultiBall => GOLD,
                PowerUpType::PaddleExtend => GREEN,
                PowerUpType::SlowTime => DARK_PURPLE,
            };
            draw_rectangle(powerup.x - POWERUP_WIDTH / 2.0, powerup.y, POWERUP_WIDTH, POWERUP_HEIGHT, color);

            // Draw letter indicator
            let letter = match powerup.power_type {
                PowerUpType::MultiBall => "M",
                PowerUpType::PaddleExtend => "P",
                PowerUpType::SlowTime => "S",
            };
            draw_text(letter, powerup.x - 3.0, powerup.y + 14.0, 16.0, BLACK);
        }
    }

    // Render HUD
    let hud_text = format!("Lives: {}", state.lives);
    draw_text(&hud_text, 10.0, 20.0, 24.0, WHITE);

    let score_text = format!("Score: {}", state.score);
    let score_width = measure_text(&score_text, None, 24, 1.0).width;
    draw_text(&score_text, SCREEN_WIDTH / 2.0 - score_width / 2.0, 20.0, 24.0, WHITE);

    let level_text = format!("Level: {}/{}", state.level, NUM_LEVELS);
    let level_width = measure_text(&level_text, None, 24, 1.0).width;
    draw_text(&level_text, SCREEN_WIDTH - level_width - 10.0, 20.0, 24.0, WHITE);

    // Render active power-ups
    let mut powerup_y = SCREEN_HEIGHT - 30.0;
    for (i, active) in state.active_powerups.iter().enumerate() {
        let power_str = match active.power_type {
            PowerUpType::MultiBall => "M",
            PowerUpType::PaddleExtend => "P",
            PowerUpType::SlowTime => "S",
        };
        let text = format!("{}({})", power_str, active.remaining_frames);
        draw_text(&text, 10.0, powerup_y - (i as f32 * 25.0), 20.0, WHITE);
    }
}

pub fn render_main_menu(state: &GameState) {
    let title = "BREAKOUT: CLASSIC REVIVAL";
    let title_width = measure_text(title, None, 48, 1.0).width;
    draw_text(title, SCREEN_WIDTH / 2.0 - title_width / 2.0, SCREEN_HEIGHT / 2.0 - 100.0, 48.0, WHITE);

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(&high_score_text, SCREEN_WIDTH / 2.0 - hs_width / 2.0, SCREEN_HEIGHT / 2.0 - 20.0, 32.0, WHITE);

    let play_text = "Press SPACE to Play";
    let play_width = measure_text(play_text, None, 28, 1.0).width;
    draw_text(play_text, SCREEN_WIDTH / 2.0 - play_width / 2.0, SCREEN_HEIGHT / 2.0 + 50.0, 28.0, YELLOW);

    let quit_text = "Press ESC to Quit";
    let quit_width = measure_text(quit_text, None, 20, 1.0).width;
    draw_text(quit_text, SCREEN_WIDTH / 2.0 - quit_width / 2.0, SCREEN_HEIGHT / 2.0 + 100.0, 20.0, WHITE);
}

pub fn render_level_complete(state: &GameState) {
    let level_text = format!("LEVEL {} COMPLETE!", state.level);
    let level_width = measure_text(&level_text, None, 40, 1.0).width;
    draw_text(&level_text, SCREEN_WIDTH / 2.0 - level_width / 2.0, SCREEN_HEIGHT / 2.0 - 50.0, 40.0, GREEN);

    let score_text = format!("Score: {}", state.score);
    let score_width = measure_text(&score_text, None, 32, 1.0).width;
    draw_text(&score_text, SCREEN_WIDTH / 2.0 - score_width / 2.0, SCREEN_HEIGHT / 2.0 + 20.0, 32.0, WHITE);
}

pub fn render_game_over(state: &GameState) {
    let game_over_text = "GAME OVER";
    let go_width = measure_text(game_over_text, None, 48, 1.0).width;
    draw_text(game_over_text, SCREEN_WIDTH / 2.0 - go_width / 2.0, SCREEN_HEIGHT / 2.0 - 80.0, 48.0, RED);

    let final_score_text = format!("Final Score: {}", state.score);
    let fs_width = measure_text(&final_score_text, None, 32, 1.0).width;
    draw_text(&final_score_text, SCREEN_WIDTH / 2.0 - fs_width / 2.0, SCREEN_HEIGHT / 2.0 - 10.0, 32.0, WHITE);

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(&high_score_text, SCREEN_WIDTH / 2.0 - hs_width / 2.0, SCREEN_HEIGHT / 2.0 + 40.0, 32.0, WHITE);

    let restart_text = "Press SPACE to Play Again";
    let restart_width = measure_text(restart_text, None, 24, 1.0).width;
    draw_text(restart_text, SCREEN_WIDTH / 2.0 - restart_width / 2.0, SCREEN_HEIGHT / 2.0 + 100.0, 24.0, YELLOW);
}

pub fn render_victory(state: &GameState) {
    let victory_text = "VICTORY!";
    let victory_width = measure_text(victory_text, None, 48, 1.0).width;
    draw_text(victory_text, SCREEN_WIDTH / 2.0 - victory_width / 2.0, SCREEN_HEIGHT / 2.0 - 80.0, 48.0, GREEN);

    let final_score_text = format!("Final Score: {}", state.score);
    let fs_width = measure_text(&final_score_text, None, 32, 1.0).width;
    draw_text(&final_score_text, SCREEN_WIDTH / 2.0 - fs_width / 2.0, SCREEN_HEIGHT / 2.0 - 10.0, 32.0, WHITE);

    let high_score_text = format!("High Score: {}", state.high_score);
    let hs_width = measure_text(&high_score_text, None, 32, 1.0).width;
    draw_text(&high_score_text, SCREEN_WIDTH / 2.0 - hs_width / 2.0, SCREEN_HEIGHT / 2.0 + 40.0, 32.0, WHITE);

    let restart_text = "Press SPACE to Play Again";
    let restart_width = measure_text(restart_text, None, 24, 1.0).width;
    draw_text(restart_text, SCREEN_WIDTH / 2.0 - restart_width / 2.0, SCREEN_HEIGHT / 2.0 + 100.0, 24.0, YELLOW);
}
