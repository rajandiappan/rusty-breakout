use crate::constants::POWERUP_SPAWN_CHANCE;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn ball_speed_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.3,
        }
    }

    pub fn paddle_width_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.3,   // 130px
            Difficulty::Normal => 1.0, // 100px
            Difficulty::Hard => 0.7,   // 70px
        }
    }

    pub fn starting_lives(&self) -> u8 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Normal => 3,
            Difficulty::Hard => 2,
        }
    }

    pub fn powerup_spawn_chance(&self) -> f32 {
        match self {
            Difficulty::Easy => POWERUP_SPAWN_CHANCE * 1.5, // Higher chance for easy
            Difficulty::Normal => POWERUP_SPAWN_CHANCE,     // 15%
            Difficulty::Hard => POWERUP_SPAWN_CHANCE * 0.67, // ~10%
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThemeType {
    Classic,
    Dark,
    Neon,
    Crt,
    Minimalist,
}

impl ThemeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeType::Classic => "classic",
            ThemeType::Dark => "dark",
            ThemeType::Neon => "neon",
            ThemeType::Crt => "crt",
            ThemeType::Minimalist => "minimalist",
        }
    }

    #[allow(dead_code)]
    pub fn all_themes() -> &'static [ThemeType] {
        &[
            ThemeType::Classic,
            ThemeType::Dark,
            ThemeType::Neon,
            ThemeType::Crt,
            ThemeType::Minimalist,
        ]
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeType::Classic => ThemeType::Dark,
            ThemeType::Dark => ThemeType::Neon,
            ThemeType::Neon => ThemeType::Crt,
            ThemeType::Crt => ThemeType::Minimalist,
            ThemeType::Minimalist => ThemeType::Classic,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GameSettings {
    pub difficulty: Difficulty,
    pub theme: ThemeType,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub particle_effects: bool,
    pub screen_shake: bool,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            difficulty: Difficulty::Normal,
            theme: ThemeType::Classic,
            music_volume: 0.7,
            sfx_volume: 0.8,
            particle_effects: true,
            screen_shake: true,
            fullscreen: false,
        }
    }
}

#[allow(dead_code)]
impl GameSettings {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            let contents = fs::read_to_string(path)?;
            let settings: GameSettings = serde_json::from_str(&contents)?;
            Ok(settings)
        } else {
            Ok(GameSettings::default())
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn clamp_volumes(&mut self) {
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_multipliers() {
        assert_eq!(Difficulty::Easy.ball_speed_multiplier(), 0.8);
        assert_eq!(Difficulty::Normal.ball_speed_multiplier(), 1.0);
        assert_eq!(Difficulty::Hard.ball_speed_multiplier(), 1.3);
    }

    #[test]
    fn test_difficulty_lives() {
        assert_eq!(Difficulty::Easy.starting_lives(), 5);
        assert_eq!(Difficulty::Normal.starting_lives(), 3);
        assert_eq!(Difficulty::Hard.starting_lives(), 2);
    }

    #[test]
    fn test_theme_cycling() {
        let mut theme = ThemeType::Classic;
        theme = theme.next();
        assert_eq!(theme, ThemeType::Dark);
        theme = theme.next();
        assert_eq!(theme, ThemeType::Neon);
    }

    #[test]
    fn test_default_settings() {
        let settings = GameSettings::default();
        assert_eq!(settings.difficulty, Difficulty::Normal);
        assert_eq!(settings.theme, ThemeType::Classic);
        assert!(settings.particle_effects);
    }
}
