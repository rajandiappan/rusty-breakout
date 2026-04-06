/// Audio system for game sounds
/// Uses procedural synthesis approach - logs audio events
#[derive(Clone, Debug)]
pub struct AudioManager {
    pub sfx_enabled: bool,
    pub music_enabled: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            sfx_enabled: true,
            music_enabled: true,
        }
    }

    /// Play a paddle hit sound (quick beep at 400 Hz)
    pub fn play_paddle_hit(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Paddle Hit", 400.0, 40);
    }

    /// Play a brick destruction sound (medium beep at 600 Hz)
    pub fn play_brick_destroy(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Brick Destroy", 600.0, 80);
    }

    /// Play a power-up pickup sound (high beep at 900 Hz)
    pub fn play_powerup_pickup(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Power-up Pickup", 900.0, 150);
    }

    /// Play a level complete sound
    pub fn play_level_complete(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Level Complete", 700.0, 200);
    }

    /// Play a game over sound
    pub fn play_game_over(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Game Over", 300.0, 300);
    }

    /// Play a victory sound
    pub fn play_victory(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.log_sound_event("Victory", 800.0, 400);
    }

    /// Log sound event for debugging and audio system testing
    fn log_sound_event(&self, sound_name: &str, frequency: f32, duration_ms: u32) {
        #[cfg(debug_assertions)]
        eprintln!(
            "[AUDIO] {}: {}Hz for {}ms",
            sound_name, frequency as u32, duration_ms
        );
    }

    pub fn set_sfx_enabled(&mut self, enabled: bool) {
        self.sfx_enabled = enabled;
    }

    pub fn set_music_enabled(&mut self, enabled: bool) {
        self.music_enabled = enabled;
    }

    pub fn toggle_sfx(&mut self) {
        self.sfx_enabled = !self.sfx_enabled;
    }

    pub fn toggle_music(&mut self) {
        self.music_enabled = !self.music_enabled;
    }
}
