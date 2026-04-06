use crate::achievements::AchievementManager;
use crate::audio::AudioManager;
use crate::constants::*;
use crate::effects::ParticleSystem;
use crate::gamepad::GamepadInput;
use crate::settings::{Difficulty, ThemeType};
use crate::themes::ThemeColors;
use macroquad::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePhase {
    MainMenu,
    Playing,
    LevelComplete,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerUpType {
    MultiBall,    // ⊕ Gold: Spawn 2 extra balls
    PaddleExtend, // ▬ Green: Widen paddle to 150px (permanent until next level or shrink)
    SlowTime,     // ◐ Purple: Reduce ball velocity 50%
    Laser,        // ↑ Cyan: Fire projectiles upward
    Shield,       // ◇ Orange: Catch 1 lost ball
    Bomb,         // ◈ Red: Destroy bricks in 3x3 area
    Magnetize,    // ● Magenta: Ball sticks to paddle
    PaddleShrink, // ◈ Red/Dark: Shrink paddle to 60px (60% of normal)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrickType {
    Normal,
    Frozen,       // Slows ball on hit (40% reduction, 120 frames)
    Exploding,    // Chain destruction (80px radius)
    Steel,        // Multi-hit (3 hits to destroy)
    Regenerating, // Respawns after delay (300 frames)
}

#[derive(Clone, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub active: bool,
    pub is_magnetized: bool, // [NEW] Stuck to paddle via Magnetize
    // Phase 5: Frozen brick effect
    pub speed_multiplier: f32, // 0.6 = 40% speed reduction
    pub frozen_timer: u32,     // Duration of slow effect
}

#[derive(Clone, Debug)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub normal_width: f32,
    pub extended_width: f32,
    pub is_extended: bool,
    pub is_shrunk: bool,                // [NEW] Shrink status
    pub shrunk_width: f32,              // [NEW] Width when shrunk
    pub shield_count: u8,               // [NEW] Shield count (for stacking)
    pub magnetized_ball: Option<usize>, // [NEW] Index of stuck ball, if any
}

#[derive(Clone, Debug)]
pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
    pub color: Color,
    // Phase 5: Advanced brick types
    pub brick_type: BrickType,
    pub health: u8,       // For Steel bricks (0-3)
    pub regen_timer: u32, // For Regenerating bricks (300 frames)
    pub is_hit: bool,     // For Regenerating combo tracking
}

#[derive(Clone, Debug)]
pub struct PowerUp {
    pub x: f32,
    pub y: f32,
    pub power_type: PowerUpType,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct ActivePowerUp {
    pub power_type: PowerUpType,
    pub remaining_frames: usize,
}

#[derive(Clone, Debug)]
pub struct LaserShot {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
}

#[derive(Debug)]
pub struct GameState {
    pub level: usize,
    pub score: u32,
    pub high_score: u32,
    pub lives: u8,
    pub phase: GamePhase,
    pub balls: Vec<Ball>,
    pub paddle: Paddle,
    pub bricks: Vec<Brick>,
    pub powerups: Vec<PowerUp>,
    pub active_powerups: Vec<ActivePowerUp>,
    pub frame_count: usize,
    pub level_complete_timer: usize,
    // Phase 2 additions
    pub difficulty: Difficulty,
    pub current_theme: ThemeType,
    pub theme_colors: ThemeColors,
    pub achievements: AchievementManager,
    pub is_paused: bool,
    pub particle_system: ParticleSystem,
    // Phase 2 extensions - new power-ups
    pub laser_shots: Vec<LaserShot>, // [NEW] Active laser projectiles
    pub audio: AudioManager,         // [NEW] Audio system
    pub gamepad: GamepadInput,       // [NEW] Gamepad/Controller input
}

impl GameState {
    pub fn new() -> Self {
        use crate::themes::get_theme_colors;

        let theme = ThemeType::Classic;
        let theme_colors = get_theme_colors(theme);

        Self {
            level: 1,
            score: 0,
            high_score: 0,
            lives: STARTING_LIVES,
            phase: GamePhase::MainMenu,
            balls: Vec::new(),
            paddle: Paddle {
                x: (SCREEN_WIDTH - PADDLE_WIDTH) / 2.0,
                y: PADDLE_Y,
                width: PADDLE_WIDTH,
                height: PADDLE_HEIGHT,
                normal_width: PADDLE_WIDTH,
                extended_width: PADDLE_EXTENDED_WIDTH,
                is_extended: false,
                is_shrunk: false,
                shrunk_width: PADDLE_WIDTH * 0.6,
                shield_count: 0,
                magnetized_ball: None,
            },
            bricks: Vec::new(),
            powerups: Vec::new(),
            active_powerups: Vec::new(),
            frame_count: 0,
            level_complete_timer: 0,
            difficulty: Difficulty::Normal,
            current_theme: theme,
            theme_colors,
            achievements: AchievementManager::new(),
            is_paused: false,
            particle_system: ParticleSystem::new(),
            laser_shots: Vec::new(),
            audio: AudioManager::new(),
            gamepad: GamepadInput::new(),
        }
    }
}
