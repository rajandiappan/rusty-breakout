#[cfg(windows)]
mod console_hide {
    #[link(name = "kernel32")]
    extern "system" {
        fn FreeConsole() -> i32;
    }

    pub fn hide_console() {
        unsafe {
            FreeConsole();
        }
    }
}

mod achievements;
mod audio;
mod ball;
mod brick;
mod constants;
mod effects;
mod game;
mod gamepad;
mod level;
mod paddle;
mod persistence;
mod physics;
mod powerup;
mod settings;
mod themes;
mod types;
mod ui;

#[cfg(windows)]
use console_hide::hide_console;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Breakout")]
async fn main() {
    #[cfg(windows)]
    hide_console();

    let mut game = Game::new();

    loop {
        // Handle input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Update game state
        game.update().await;

        // Render
        game.render();

        // Frame timing (60 FPS)
        next_frame().await;
    }
}
