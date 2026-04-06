// Game window and layout constants
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
#[allow(dead_code)]
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
pub const NUM_LEVELS: usize = 10;
pub const LEVEL_COMPLETE_BONUS: u32 = 1000;
pub const ALL_LEVELS_BONUS: u32 = 5000;

// [NEW] Phase 5: Advanced brick type constants
pub const FROZEN_SPEED_REDUCTION: f32 = 0.6; // 40% speed reduction
pub const FROZEN_DURATION: u32 = 120; // 120 frames (2 seconds)
pub const EXPLODING_RADIUS: f32 = 80.0; // Chain reaction radius in pixels
pub const STEEL_BRICK_HEALTH: u8 = 3; // Hits required to destroy
pub const REGENERATING_DURATION: u32 = 300; // 300 frames (5 seconds)

// Power-up constants
pub const POWERUP_WIDTH: f32 = 20.0;
pub const POWERUP_HEIGHT: f32 = 20.0;
pub const POWERUP_FALL_SPEED: f32 = 3.0;
pub const POWERUP_SPAWN_CHANCE: f32 = 0.15; // 15% chance
pub const POWERUP_DURATION: usize = 60; // frames (1 second at 60 FPS)

// [NEW] Phase 2 extended power-up durations
pub const POWERUP_LASER_DURATION: usize = 120; // 2 seconds of laser firing
#[allow(dead_code)]
pub const POWERUP_SHIELD_DURATION: usize = 1; // Single-use (consumed immediately on impact)
#[allow(dead_code)]
pub const POWERUP_BOMB_DURATION: usize = 1; // Single-use (triggered once)
pub const POWERUP_MAGNETIZE_DURATION: usize = 180; // 3 seconds stuck mode

// [NEW] Laser constants
pub const LASER_WIDTH: f32 = 8.0;
pub const LASER_HEIGHT: f32 = 15.0;
pub const LASER_SPEED: f32 = 8.0;

// Power-up symbols and descriptions
pub const POWERUP_MULTIBALL_SYMBOL: &str = "⊕"; // Circle with cross (multiple balls)
pub const POWERUP_MULTIBALL_LABEL: &str = "M"; // Fallback letter
pub const POWERUP_EXTEND_SYMBOL: &str = "▬"; // Horizontal bar (paddle extension)
pub const POWERUP_EXTEND_LABEL: &str = "P"; // Fallback letter
pub const POWERUP_SLOWTIME_SYMBOL: &str = "◐"; // Half circle (time/slow effect)
pub const POWERUP_SLOWTIME_LABEL: &str = "S"; // Fallback letter

// [NEW] Phase 2 extended power-up symbols
pub const POWERUP_LASER_SYMBOL: &str = "↑"; // Up arrow (laser fire)
pub const POWERUP_LASER_LABEL: &str = "L";
pub const POWERUP_SHIELD_SYMBOL: &str = "◇"; // Diamond (protection)
pub const POWERUP_SHIELD_LABEL: &str = "X";
pub const POWERUP_BOMB_SYMBOL: &str = "◈"; // Diamond with center (explosion)
pub const POWERUP_BOMB_LABEL: &str = "B";
pub const POWERUP_MAGNETIZE_SYMBOL: &str = "●"; // Filled circle (magnetic)
pub const POWERUP_MAGNETIZE_LABEL: &str = "Z";
pub const POWERUP_SHRINK_SYMBOL: &str = "◈"; // Circle bomb (shrink - power-down)
pub const POWERUP_SHRINK_LABEL: &str = "S"; // Fallback letter

// Game constants
pub const STARTING_LIVES: u8 = 3;
pub const MAX_BALLS: usize = 3;

// Colors (RGB)
pub use macroquad::color::{GREEN, MAGENTA, ORANGE, RED, YELLOW};

pub const CYAN: macroquad::color::Color = macroquad::color::Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const DARK_PURPLE: macroquad::color::Color = macroquad::color::Color {
    r: 0.6,
    g: 0.2,
    b: 1.0,
    a: 1.0,
};

#[allow(dead_code)]
pub const GOLD: macroquad::color::Color = macroquad::color::Color {
    r: 1.0,
    g: 0.84,
    b: 0.0,
    a: 1.0,
};

// Brick colors (rainbow pattern)
pub const BRICK_COLORS: &[macroquad::color::Color] = &[
    RED,     // Row 1
    ORANGE,  // Row 2
    YELLOW,  // Row 3
    GREEN,   // Row 4
    CYAN,    // Row 5
    MAGENTA, // Row 6
];
