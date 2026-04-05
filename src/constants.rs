// Game window and layout constants
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const FPS: u32 = 60;

// Ball constants
pub const BALL_RADIUS: f32 = 5.0;
pub const BALL_BASE_SPEED: f32 = 4.0;
pub const BALL_MAX_SPEED: f32 = 6.0;

// Paddle constants
pub const PADDLE_WIDTH: f32 = 100.0;
pub const PADDLE_EXTENDED_WIDTH: f32 = 150.0;
pub const PADDLE_HEIGHT: f32 = 15.0;
pub const PADDLE_Y: f32 = 550.0;
pub const PADDLE_SPEED: f32 = 7.0;

// Brick constants
pub const BRICK_WIDTH: f32 = 60.0;
pub const BRICK_HEIGHT: f32 = 20.0;
pub const BRICK_COLS: usize = 12;
pub const BRICK_ROWS: usize = 6;
pub const BRICK_START_X: f32 = 20.0;
pub const BRICK_START_Y: f32 = 60.0;
pub const BRICK_SPACING: f32 = 8.0;
pub const BRICK_POINTS: u32 = 10;

// Level constants
pub const NUM_LEVELS: usize = 5;
pub const LEVEL_COMPLETE_BONUS: u32 = 1000;
pub const ALL_LEVELS_BONUS: u32 = 5000;

// Power-up constants
pub const POWERUP_WIDTH: f32 = 20.0;
pub const POWERUP_HEIGHT: f32 = 20.0;
pub const POWERUP_FALL_SPEED: f32 = 3.0;
pub const POWERUP_SPAWN_CHANCE: f32 = 0.15; // 15% chance
pub const POWERUP_DURATION: usize = 60; // frames (1 second at 60 FPS)

// Game constants
pub const STARTING_LIVES: u8 = 3;
pub const MAX_BALLS: usize = 3;

// Colors (RGB)
pub use macroquad::color::{BLACK, WHITE, RED, ORANGE, YELLOW, GREEN, CYAN, MAGENTA};

pub const DARK_PURPLE: macroquad::color::Color = macroquad::color::Color {
    r: 0.6,
    g: 0.2,
    b: 1.0,
    a: 1.0,
};

pub const GOLD: macroquad::color::Color = macroquad::color::Color {
    r: 1.0,
    g: 0.84,
    b: 0.0,
    a: 1.0,
};

// Brick colors (rainbow pattern)
pub const BRICK_COLORS: &[macroquad::color::Color] = &[
    RED,      // Row 1
    ORANGE,   // Row 2
    YELLOW,   // Row 3
    GREEN,    // Row 4
    CYAN,     // Row 5
    MAGENTA,  // Row 6
];
