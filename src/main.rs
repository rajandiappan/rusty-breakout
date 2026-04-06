#[cfg(windows)]
mod console_hide {
    #[link(name = "kernel32")]
    extern "system" {
        fn FreeConsole() -> i32;
    }

    pub fn hide_console() {
        unsafe { FreeConsole(); }
    }
}

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
mod effects;
mod audio;
mod gamepad;

#[cfg(windows)]
use console_hide::hide_console;

use macroquad::prelude::*;
use game::Game;

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