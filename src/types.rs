use crate::achievements::AchievementManager;
use crate::effects::ParticleSystem;
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
    MultiBall,
    PaddleExtend,
    SlowTime,
}

#[derive(Clone, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub active: bool,
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
}

#[derive(Clone, Debug)]
pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub active: bool,
    pub color: Color,
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
            lives: 3,
            phase: GamePhase::MainMenu,
            balls: Vec::new(),
            paddle: Paddle {
                x: 350.0,
                y: 550.0,
                width: 100.0,
                height: 15.0,
                normal_width: 100.0,
                extended_width: 150.0,
                is_extended: false,
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
        }
    }
}
