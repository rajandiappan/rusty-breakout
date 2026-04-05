mod constants;
mod types;
mod game;
mod ball;
mod paddle;
mod brick;
mod powerup;
mod physics;
mod level;
mod ui;
mod settings;
mod themes;
mod achievements;

use macroquad::prelude::*;
use game::Game;

#[macroquad::main("Breakout")]
async fn main() {
    let mut game = Game::new();

    loop {
        // Handle input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Update game state
        game.update().await;

        // Render
        clear_background(BLACK);
        game.render();

        // Frame timing (60 FPS)
        next_frame().await;
    }
}
