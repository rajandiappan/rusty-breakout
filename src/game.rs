use crate::achievements::{AchievementId, AchievementManager};
use crate::audio::MusicTier;
use crate::constants::*;
use crate::level;
use crate::settings::{Difficulty, GameSettings};
use crate::types::{Ball, BrickType, GamePhase, GameState, PowerUp, PowerUpType};
use macroquad::prelude::*;

const SETTINGS_PATH: &str = "settings/game.json";
const HIGH_SCORE_PATH: &str = "settings/high_score.txt";
const ACHIEVEMENTS_PATH: &str = "settings/achievements.json";

pub struct Game {
    pub state: GameState,
    pub settings: GameSettings,
}

impl Game {
    const DEV_MENU_ROW_COUNT: usize = 9;

    pub fn new() -> Self {
        let settings = GameSettings::load_from_file(SETTINGS_PATH).unwrap_or_default();
        let mut game = Game {
            state: GameState::new(),
            settings,
        };
        game.load_progress();
        game.apply_settings();
        game.track_theme_unlock();
        game.start_menu();
        game
    }

    pub fn apply_settings(&mut self) {
        self.state.difficulty = self.settings.difficulty;
        self.state.current_theme = self.settings.theme;
        self.state.theme_colors = crate::themes::get_theme_colors(self.state.current_theme);
        self.state.audio.volume = self.settings.sfx_volume;
        self.state
            .audio
            .set_music_volume(self.settings.music_volume);
        self.state.seen_themes = self.settings.seen_themes.iter().copied().collect();
        self.state.seen_themes.insert(self.state.current_theme);
    }

    pub fn save_settings(&mut self) {
        self.settings.clamp_volumes();
        let _ = self.settings.save_to_file(SETTINGS_PATH);
    }

    fn load_progress(&mut self) {
        self.state.high_score = crate::persistence::load_high_score(HIGH_SCORE_PATH).unwrap_or(0);
        self.state.achievements = AchievementManager::load_from_file(ACHIEVEMENTS_PATH)
            .unwrap_or_else(|_| AchievementManager::new());
    }

    fn save_achievements(&self) {
        let _ = self.state.achievements.save_to_file(ACHIEVEMENTS_PATH);
    }

    fn refresh_music_state(&mut self) {
        let tier = MusicTier::from(self.state.level);
        let active_balls = self.state.balls.iter().filter(|ball| ball.active).count();
        let active_bricks = self
            .state
            .bricks
            .iter()
            .filter(|brick| brick.active)
            .count();
        let danger = self.state.lives <= 1
            || active_balls >= MAX_BALLS
            || (self.state.level >= 8 && active_bricks > 0 && active_bricks <= 10);
        self.state.audio.set_music_state(tier, danger);
    }

    fn record_high_score(&mut self) {
        if self.state.score > self.state.high_score {
            self.state.high_score = self.state.score;
            let _ = crate::persistence::save_high_score(HIGH_SCORE_PATH, self.state.high_score);
        }
    }

    fn add_score(&mut self, amount: u32) {
        self.state.score += amount;
        self.record_high_score();
    }

    fn increment_achievement_progress(&mut self, id: AchievementId, amount: u32) {
        self.state.achievements.increment_progress(id, amount);
        self.save_achievements();
    }

    fn unlock_achievement(&mut self, id: AchievementId) {
        self.state.achievements.unlock(id);
        self.save_achievements();
    }

    fn track_theme_unlock(&mut self) {
        self.state.seen_themes.insert(self.state.current_theme);
        if !self
            .settings
            .seen_themes
            .contains(&self.state.current_theme)
        {
            self.settings.seen_themes.push(self.state.current_theme);
            self.save_settings();
        }
        if self.state.seen_themes.len() == crate::settings::ThemeType::all_themes().len() {
            self.unlock_achievement(AchievementId::ThemeCollector);
        }
    }

    fn track_powerup_pickup_achievements(&mut self) {
        self.state.powerups_collected_this_level += 1;
        if self.state.powerups_collected_this_level >= 5 {
            self.unlock_achievement(AchievementId::PowerUpHoarder);
        }

        let now = self.state.frame_count;
        let window_start = now.saturating_sub(5 * FPS as usize);
        self.state.recent_powerup_frames.push_back(now);
        while self
            .state
            .recent_powerup_frames
            .front()
            .is_some_and(|frame| *frame < window_start)
        {
            self.state.recent_powerup_frames.pop_front();
        }
        if self.state.recent_powerup_frames.len() >= 3 {
            self.unlock_achievement(AchievementId::LuckyBreak);
        }
    }

    fn track_brick_destroy_achievement(&mut self) {
        self.state.bricks_destroyed_this_level += 1;
        if self.state.bricks_destroyed_this_level >= 100 {
            self.unlock_achievement(AchievementId::Sharpshooter);
        }
    }

    fn track_brick_destruction_batch(&mut self, count: u32) {
        for _ in 0..count {
            self.track_brick_destroy_achievement();
        }
    }

    fn resolve_auxiliary_brick_hit(&mut self, brick_index: usize) -> bool {
        let brick = &mut self.state.bricks[brick_index];
        if !brick.active {
            return false;
        }

        match brick.brick_type {
            BrickType::Normal | BrickType::Frozen => {
                brick.active = false;
                true
            }
            BrickType::Exploding => {
                brick.active = false;
                true
            }
            BrickType::Steel => {
                if brick.health > 0 {
                    brick.health -= 1;
                    self.state.audio.play_steel_hit();
                    self.state
                        .particle_system
                        .steel_impact(brick.x + brick.width / 2.0, brick.y + brick.height / 2.0);
                    if brick.health == 0 {
                        brick.active = false;
                        return true;
                    }
                }
                false
            }
            BrickType::Regenerating => {
                brick.active = false;
                brick.is_hit = true;
                brick.regen_timer = REGENERATING_DURATION;
                true
            }
        }
    }

    fn trigger_exploding_brick_chain(&mut self, exploding_idx: usize) -> (u32, u32) {
        let bx = self.state.bricks[exploding_idx].x + BRICK_WIDTH / 2.0;
        let by = self.state.bricks[exploding_idx].y + BRICK_HEIGHT / 2.0;
        let mut chain_count = 0;
        let mut gained_score = 0;
        let mut destroyed_bricks = 0;
        let mut chain_targets = Vec::new();

        for (other_idx, other_brick) in self.state.bricks.iter().enumerate() {
            if other_idx == exploding_idx || !other_brick.active {
                continue;
            }

            let ox = other_brick.x + BRICK_WIDTH / 2.0;
            let oy = other_brick.y + BRICK_HEIGHT / 2.0;
            let dist = ((bx - ox).powi(2) + (by - oy).powi(2)).sqrt();
            if dist < EXPLODING_RADIUS {
                chain_targets.push(other_idx);
            }
        }

        for other_idx in chain_targets {
            let was_exploding = self.state.bricks[other_idx].brick_type == BrickType::Exploding;
            let was_active = self.state.bricks[other_idx].active;
            let destroyed = self.resolve_auxiliary_brick_hit(other_idx);
            if destroyed {
                chain_count += 1;
                gained_score += BRICK_POINTS;
                destroyed_bricks += 1;
                let brick = &self.state.bricks[other_idx];
                self.emit_destroyed_brick_feedback(
                    brick.brick_type,
                    brick.x + BRICK_WIDTH / 2.0,
                    brick.y + BRICK_HEIGHT / 2.0,
                    brick.color,
                );

                if was_exploding && was_active {
                    let (nested_score, nested_destroyed) =
                        self.trigger_exploding_brick_chain(other_idx);
                    gained_score += nested_score;
                    destroyed_bricks += nested_destroyed;
                }
            }
        }

        if chain_count > 2 {
            self.state.screen_flash = 0.3;
        }

        (gained_score, destroyed_bricks)
    }

    fn emit_destroyed_brick_feedback(
        &mut self,
        brick_type: BrickType,
        center_x: f32,
        center_y: f32,
        color: Color,
    ) {
        match brick_type {
            BrickType::Frozen => {
                self.state
                    .particle_system
                    .frozen_shatter(center_x, center_y);
            }
            BrickType::Exploding => {
                self.state
                    .particle_system
                    .explosion_burst(center_x, center_y);
                self.state.screen_flash = self.state.screen_flash.max(0.22);
            }
            _ => {
                self.state
                    .particle_system
                    .brick_destruction(center_x, center_y, color);
            }
        }
    }

    fn play_brick_feedback_sound(&self, brick_type: BrickType, destroyed: bool) {
        match brick_type {
            BrickType::Frozen if destroyed => self.state.audio.play_frozen_shatter(),
            BrickType::Exploding if destroyed => self.state.audio.play_exploding_burst(),
            BrickType::Steel => self.state.audio.play_steel_hit(),
            BrickType::Regenerating if destroyed => self.state.audio.play_regenerating_break(),
            _ if destroyed => self.state.audio.play_brick_destroy(),
            _ => {}
        }
    }

    fn track_multiball_achievement(&mut self) {
        let active_balls = self.state.balls.iter().filter(|ball| ball.active).count();
        if active_balls >= MAX_BALLS {
            self.state.three_ball_streak_frames += 1;
            if self.state.three_ball_streak_frames >= 30 * FPS as usize {
                self.unlock_achievement(AchievementId::MultiBallMaster);
            }
        } else {
            self.state.three_ball_streak_frames = 0;
        }
    }

    fn reset_level_tracking(&mut self) {
        self.state.level_start_frame = self.state.frame_count;
        self.state.bricks_destroyed_this_level = 0;
        self.state.powerups_collected_this_level = 0;
        self.state.three_ball_streak_frames = 0;
        self.state.recent_powerup_frames.clear();
    }

    fn check_level_completion_achievements(&mut self) {
        let level_frames = self
            .state
            .frame_count
            .saturating_sub(self.state.level_start_frame);
        if level_frames <= 60 * FPS as usize {
            self.unlock_achievement(AchievementId::RapidFire);
        }
    }

    fn check_victory_achievements(&mut self) {
        if self.state.lives == self.state.run_starting_lives {
            self.unlock_achievement(AchievementId::PerfectClear);
        }

        let run_frames = self
            .state
            .frame_count
            .saturating_sub(self.state.game_start_frame);
        if run_frames <= 5 * 60 * FPS as usize {
            self.unlock_achievement(AchievementId::Speedrunner);
        }

        if self.state.difficulty == Difficulty::Hard {
            self.unlock_achievement(AchievementId::HardcoreChampion);
        }
    }

    fn dev_powerup_catalog() -> [PowerUpType; 8] {
        [
            PowerUpType::MultiBall,
            PowerUpType::PaddleExtend,
            PowerUpType::SlowTime,
            PowerUpType::Laser,
            PowerUpType::Shield,
            PowerUpType::Bomb,
            PowerUpType::Magnetize,
            PowerUpType::PaddleShrink,
        ]
    }

    fn toggle_dev_menu(&mut self) {
        if self.state.dev_tools.enabled {
            self.state.dev_tools.open = !self.state.dev_tools.open;
            self.state.dev_tools.selected_row = 0;
        }
    }

    fn can_open_dev_menu(&self) -> bool {
        self.state.dev_tools.enabled
            && matches!(
                self.state.phase,
                GamePhase::MainMenu
                    | GamePhase::LevelComplete
                    | GamePhase::GameOver
                    | GamePhase::Victory
            )
            || (self.state.dev_tools.enabled
                && self.state.phase == GamePhase::Playing
                && self.state.is_paused)
    }

    fn activate_selected_level(&mut self) {
        let level = self.state.dev_tools.selected_level;
        match self.state.phase {
            GamePhase::MainMenu | GamePhase::GameOver | GamePhase::Victory => {
                self.start_fresh_run_at_level(level);
            }
            GamePhase::Playing | GamePhase::LevelComplete => {
                self.jump_to_level(level);
            }
        }
    }

    fn reset_ball_and_paddle_state(&mut self) {
        let speed_multiplier = self.state.difficulty.ball_speed_multiplier();
        self.state.paddle.width = self.state.paddle.normal_width;
        self.state.paddle.is_extended = false;
        self.state.paddle.is_shrunk = false;
        self.state.paddle.shield_count = 0;
        self.state.paddle.magnetized_ball = None;
        self.state.paddle.x = (SCREEN_WIDTH - self.state.paddle.width) / 2.0;
        self.state.laser_shots.clear();
        self.state.active_powerups.clear();
        self.state.balls = vec![Ball {
            x: SCREEN_WIDTH / 2.0,
            y: PADDLE_Y - PADDLE_HEIGHT * 2.0,
            vx: 2.0 * speed_multiplier,
            vy: -BALL_BASE_SPEED * speed_multiplier,
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        }];
    }

    fn clear_active_powerup_effects(&mut self) {
        self.state.active_powerups.clear();
        self.state.powerups.clear();
        self.state.laser_shots.clear();
        self.state.paddle.width = self.state.paddle.normal_width;
        self.state.paddle.is_extended = false;
        self.state.paddle.is_shrunk = false;
        self.state.paddle.shield_count = 0;
        self.state.paddle.magnetized_ball = None;
        for ball in &mut self.state.balls {
            ball.is_magnetized = false;
            ball.frozen_timer = 0;
            ball.speed_multiplier = 1.0;
        }
    }

    fn start_fresh_run_at_level(&mut self, level: usize) {
        self.state.score = 0;
        self.state.lives = self.state.difficulty.starting_lives();
        self.state.phase = GamePhase::Playing;
        self.state.is_paused = false;
        self.state.dev_tools.open = false;
        self.state.game_start_frame = self.state.frame_count;
        self.state.run_starting_lives = self.state.lives;
        self.state.audio.start_music();
        self.load_level(level);
    }

    fn jump_to_level(&mut self, level: usize) {
        self.state.phase = GamePhase::Playing;
        self.state.is_paused = false;
        self.state.dev_tools.open = false;
        self.state.audio.start_music();
        self.load_level(level);
    }

    fn apply_dev_menu_action(&mut self) {
        match self.state.dev_tools.selected_row {
            0 => self.activate_selected_level(),
            3 => self.jump_to_level(self.state.dev_tools.selected_level),
            4 => self.start_fresh_run_at_level(self.state.dev_tools.selected_level),
            5 => {
                let current_level = self.state.level.max(1);
                self.jump_to_level(current_level);
            }
            6 => {
                if self.state.phase == GamePhase::Playing {
                    let powerup =
                        Self::dev_powerup_catalog()[self.state.dev_tools.selected_powerup_index];
                    self.apply_powerup(powerup);
                }
            }
            7 => {
                if self.state.phase == GamePhase::Playing {
                    self.clear_active_powerup_effects();
                }
            }
            8 => {
                if self.state.phase == GamePhase::LevelComplete {
                    self.state.level_complete_timer = 0;
                    self.state.dev_tools.open = false;
                } else if self.state.phase == GamePhase::Playing {
                    self.reset_ball_and_paddle_state();
                }
            }
            _ => {}
        }
    }

    fn is_dev_menu_action_available(&self, row: usize) -> bool {
        match row {
            0 => true,
            1 => true,
            2 => true,
            3 => matches!(
                self.state.phase,
                GamePhase::Playing | GamePhase::LevelComplete
            ),
            4 => matches!(
                self.state.phase,
                GamePhase::MainMenu
                    | GamePhase::Playing
                    | GamePhase::LevelComplete
                    | GamePhase::GameOver
                    | GamePhase::Victory
            ),
            5 => self.state.phase == GamePhase::Playing,
            6 => self.state.phase == GamePhase::Playing,
            7 => self.state.phase == GamePhase::Playing,
            8 => {
                self.state.phase == GamePhase::Playing
                    || self.state.phase == GamePhase::LevelComplete
            }
            _ => false,
        }
    }

    fn update_dev_menu(&mut self) {
        if !self.can_open_dev_menu() {
            self.state.dev_tools.open = false;
            return;
        }

        if is_key_pressed(KeyCode::F1) {
            self.toggle_dev_menu();
        }

        if !self.state.dev_tools.open {
            return;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.state.dev_tools.open = false;
            return;
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.state.dev_tools.selected_row = self.state.dev_tools.selected_row.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.state.dev_tools.selected_row =
                (self.state.dev_tools.selected_row + 1).min(Self::DEV_MENU_ROW_COUNT - 1);
        }

        let adjust_left = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        let adjust_right = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);
        if adjust_left || adjust_right {
            let delta = if adjust_right { 1isize } else { -1isize };
            match self.state.dev_tools.selected_row {
                0 => {
                    let level = self.state.dev_tools.selected_level as isize + delta;
                    self.state.dev_tools.selected_level =
                        level.clamp(1, NUM_LEVELS as isize) as usize;
                }
                1 => {
                    let max = Self::dev_powerup_catalog().len() as isize - 1;
                    let index = self.state.dev_tools.selected_powerup_index as isize + delta;
                    self.state.dev_tools.selected_powerup_index = index.clamp(0, max) as usize;
                }
                2 => {
                    self.state.dev_tools.infinite_lives = !self.state.dev_tools.infinite_lives;
                }
                _ => {}
            }
        }

        if (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space))
            && self.is_dev_menu_action_available(self.state.dev_tools.selected_row)
        {
            self.apply_dev_menu_action();
        }
    }

    pub fn start_menu(&mut self) {
        self.state.phase = GamePhase::MainMenu;
        self.state.score = 0;
        self.state.lives = self.state.difficulty.starting_lives();
        self.state.level = 1;
        self.state.dev_tools.selected_level = 1;
        self.refresh_music_state();
        self.state.audio.stop_music();
    }

    pub fn start_game(&mut self) {
        self.state.phase = GamePhase::Playing;
        self.refresh_music_state();
        self.state.audio.start_music();
        self.state.game_start_frame = self.state.frame_count;
        self.state.run_starting_lives = self.state.difficulty.starting_lives();
        self.load_level(self.state.level);
    }

    pub fn load_level(&mut self, level_num: usize) {
        self.state.level = level_num.min(NUM_LEVELS);
        self.state.dev_tools.selected_level = self.state.level;
        self.state.bricks = level::create_level_bricks(level_num);
        self.state.powerups.clear();
        self.state.active_powerups.clear();
        self.state.laser_shots.clear();

        // Apply difficulty multiplier to ball speed
        let speed_multiplier = self.state.difficulty.ball_speed_multiplier();
        let initial_ball = Ball {
            x: SCREEN_WIDTH / 2.0,
            y: PADDLE_Y - PADDLE_HEIGHT * 2.0,
            vx: 2.0 * speed_multiplier,
            vy: -BALL_BASE_SPEED * speed_multiplier,
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        };
        self.state.balls = vec![initial_ball];

        // Reset paddle with difficulty-adjusted width
        let paddle_width = PADDLE_WIDTH * self.state.difficulty.paddle_width_multiplier();
        self.state.paddle.x = (SCREEN_WIDTH - paddle_width) / 2.0;
        self.state.paddle.width = paddle_width;
        self.state.paddle.normal_width = paddle_width;
        self.state.paddle.extended_width = paddle_width * 1.5;
        self.state.paddle.shrunk_width = paddle_width * 0.6;
        self.state.paddle.is_extended = false;
        self.state.paddle.is_shrunk = false;
        self.state.paddle.shield_count = 0;
        self.state.paddle.magnetized_ball = None;
        self.reset_level_tracking();
        self.refresh_music_state();
    }

    pub async fn update(&mut self) {
        // Update gamepad state first (captures all input events)
        self.state.gamepad.update();

        self.state.frame_count += 1;

        match self.state.phase {
            GamePhase::MainMenu => {
                self.update_menu();
            }
            GamePhase::Playing => {
                self.update_playing();
            }
            GamePhase::LevelComplete => {
                self.update_level_complete();
            }
            GamePhase::GameOver | GamePhase::Victory => {
                self.update_game_over();
            }
        }
    }

    fn update_menu(&mut self) {
        self.update_dev_menu();
        if self.state.dev_tools.open {
            return;
        }

        // Keyboard: Spacebar to start
        if is_key_pressed(KeyCode::Space) {
            self.start_game();
        }

        // Gamepad: A button (South) or Start button to start
        if self.state.gamepad.is_south_pressed() || self.state.gamepad.is_start_pressed() {
            self.start_game();
        }
    }

    fn update_playing(&mut self) {
        // KEYBOARD CONTROLS
        // Handle pause toggle (P key)
        if is_key_pressed(KeyCode::P) {
            self.state.is_paused = !self.state.is_paused;
        }

        // Handle theme switching (T key)
        if is_key_pressed(KeyCode::T) {
            self.state.current_theme = self.state.current_theme.next();
            self.state.theme_colors = crate::themes::get_theme_colors(self.state.current_theme);
            self.track_theme_unlock();
            self.settings.theme = self.state.current_theme;
            self.save_settings();
        }

        // Handle difficulty switching (D key)
        if is_key_pressed(KeyCode::D) {
            self.state.difficulty = match self.state.difficulty {
                Difficulty::Easy => Difficulty::Normal,
                Difficulty::Normal => Difficulty::Hard,
                Difficulty::Hard => Difficulty::Easy,
            };
            self.settings.difficulty = self.state.difficulty;
            self.save_settings();
        }

        // Handle volume control (+ and - keys)
        if is_key_pressed(KeyCode::Equal) {
            self.state.audio.increase_volume();
            self.settings.sfx_volume = self.state.audio.volume;
            self.save_settings();
        }
        if is_key_pressed(KeyCode::Minus) {
            self.state.audio.decrease_volume();
            self.settings.sfx_volume = self.state.audio.volume;
            self.save_settings();
        }

        // Handle music toggle (M key)
        if is_key_pressed(KeyCode::M) {
            self.state.audio.toggle_music();
        }

        // GAMEPAD CONTROLS
        // Pause toggle (Start button)
        if self.state.gamepad.is_start_pressed() {
            self.state.is_paused = !self.state.is_paused;
        }

        // Volume control (LB/RB shoulder buttons)
        if self.state.gamepad.is_lb_pressed() {
            self.state.audio.decrease_volume();
            self.settings.sfx_volume = self.state.audio.volume;
            self.save_settings();
        }
        if self.state.gamepad.is_rb_pressed() {
            self.state.audio.increase_volume();
            self.settings.sfx_volume = self.state.audio.volume;
            self.save_settings();
        }

        // Music toggle (Back/Select button on gamepad)
        if self.state.gamepad.is_select_pressed() {
            self.state.audio.toggle_music();
        }

        // Update particle system regardless of pause
        self.state.particle_system.update(1.0 / 60.0);

        // Update score popups
        for popup in &mut self.state.score_popups {
            popup.y -= 30.0 * (1.0 / 60.0); // Float upward
            popup.lifetime -= 1.0 / 60.0;
        }
        self.state.score_popups.retain(|p| p.lifetime > 0.0);

        // Update screen flash (decay)
        if self.state.screen_flash > 0.0 {
            self.state.screen_flash -= 0.02;
            if self.state.screen_flash < 0.0 {
                self.state.screen_flash = 0.0;
            }
        }

        // Skip game updates if paused
        if self.state.is_paused {
            self.update_dev_menu();
            return;
        }

        // Update paddle
        self.update_paddle();

        // Update balls
        self.update_balls();

        // Update power-ups
        self.update_powerups();

        // Check collisions
        self.check_collisions();

        // Check win/lose conditions
        self.check_game_conditions();

        self.track_multiball_achievement();
        self.refresh_music_state();
    }

    fn update_paddle(&mut self) {
        // Keyboard input
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.state.paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.state.paddle.x += PADDLE_SPEED;
        }

        // Gamepad input - Analog stick for smooth movement
        let stick_x = self.state.gamepad.get_left_stick_x(0.2);
        if stick_x.abs() > 0.0 {
            self.state.paddle.x += stick_x * PADDLE_SPEED * 0.8; // 0.8x speed for gamepad (smoother)
        }

        // Gamepad D-Pad as digital alternative
        if self.state.gamepad.is_dpad_left_pressed() {
            self.state.paddle.x -= PADDLE_SPEED;
        }
        if self.state.gamepad.is_dpad_right_pressed() {
            self.state.paddle.x += PADDLE_SPEED;
        }

        // Clamp paddle to screen bounds
        let max_x = SCREEN_WIDTH - self.state.paddle.width;
        self.state.paddle.x = self.state.paddle.x.max(0.0).min(max_x);
    }

    fn update_balls(&mut self) {
        for ball in &mut self.state.balls {
            if !ball.active {
                continue;
            }

            // [NEW] Handle magnetized balls (stick to paddle)
            if ball.is_magnetized {
                let paddle_center_x = self.state.paddle.x + self.state.paddle.width / 2.0;
                ball.x = paddle_center_x;
                ball.y = PADDLE_Y - PADDLE_HEIGHT;
                ball.vx = 0.0;
                ball.vy = 0.0;

                // Release on spacebar or gamepad A button
                let release =
                    is_key_pressed(KeyCode::Space) || self.state.gamepad.is_south_pressed();

                if release {
                    ball.is_magnetized = false;
                    ball.vx = 2.0;
                    ball.vy = -4.0;
                }
                continue; // Skip normal physics for magnetized ball
            }

            // Apply slow time power-up effect
            let mut slow_time_multiplier = if self
                .state
                .active_powerups
                .iter()
                .any(|p| p.power_type == PowerUpType::SlowTime)
            {
                0.5
            } else {
                1.0
            };

            // Apply frozen brick effect
            if ball.frozen_timer > 0 {
                ball.frozen_timer -= 1;
                slow_time_multiplier *= FROZEN_SPEED_REDUCTION;
            }

            // Update position
            ball.x += ball.vx * slow_time_multiplier;
            ball.y += ball.vy * slow_time_multiplier;

            // Emit ball trail particles
            if self.state.frame_count.is_multiple_of(3) {
                self.state
                    .particle_system
                    .ball_trail(ball.x, ball.y, self.state.theme_colors.ball);
            }

            // Wall collisions
            if ball.x <= BALL_RADIUS || ball.x >= SCREEN_WIDTH - BALL_RADIUS {
                ball.vx = -ball.vx;
                ball.x = ball.x.clamp(BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS);
            }

            if ball.y <= BALL_RADIUS {
                ball.vy = -ball.vy;
                ball.y = BALL_RADIUS;
            }

            // Bottom (mark as lost, but don't lose life yet)
            if ball.y > SCREEN_HEIGHT {
                ball.active = false;
            }
        }

        // [NEW] Handle laser firing
        if self
            .state
            .active_powerups
            .iter()
            .any(|p| p.power_type == PowerUpType::Laser)
        {
            // Fire laser from paddle every frame when active
            if self.state.frame_count.is_multiple_of(6) {
                // Fire every 6 frames (10 shots/sec)
                self.state.laser_shots.push(crate::types::LaserShot {
                    x: self.state.paddle.x + self.state.paddle.width / 2.0 - LASER_WIDTH / 2.0,
                    y: PADDLE_Y - LASER_HEIGHT,
                    width: LASER_WIDTH,
                    height: LASER_HEIGHT,
                    active: true,
                });
            }
        }

        // Remove inactive balls
        self.state.balls.retain(|b| b.active);

        // Only lose 1 life when ALL balls are gone
        if self.state.balls.is_empty() {
            if !self.state.dev_tools.infinite_lives {
                self.state.lives -= 1;
            }

            // Reset paddle state on life lost
            self.state.paddle.width = self.state.paddle.normal_width;
            self.state.paddle.is_extended = false;
            self.state.paddle.is_shrunk = false;
            self.state.paddle.shield_count = 0;

            if self.state.lives == 0 {
                self.record_high_score();
                self.save_achievements();
                self.state.phase = GamePhase::GameOver;
            } else {
                // Reset with 1 ball
                let speed_multiplier = self.state.difficulty.ball_speed_multiplier();
                let initial_ball = Ball {
                    x: SCREEN_WIDTH / 2.0,
                    y: PADDLE_Y - PADDLE_HEIGHT * 2.0,
                    vx: 2.0 * speed_multiplier,
                    vy: -BALL_BASE_SPEED * speed_multiplier,
                    radius: BALL_RADIUS,
                    active: true,
                    is_magnetized: false,
                    speed_multiplier: 1.0,
                    frozen_timer: 0,
                };
                self.state.balls = vec![initial_ball];
            }
        }
    }

    fn update_powerups(&mut self) {
        // Update falling power-ups
        for powerup in &mut self.state.powerups {
            if powerup.active {
                powerup.y += POWERUP_FALL_SPEED;
                if powerup.y > SCREEN_HEIGHT {
                    powerup.active = false;
                }
            }
        }

        // Update active power-up timers
        for powerup in &mut self.state.active_powerups {
            if powerup.remaining_frames > 0 {
                powerup.remaining_frames -= 1;
            }
        }

        // Magnetize expiration: release magnetized balls when duration ends
        if self
            .state
            .active_powerups
            .iter()
            .any(|p| p.power_type == PowerUpType::Magnetize && p.remaining_frames == 0)
        {
            for ball in &mut self.state.balls {
                if ball.is_magnetized {
                    ball.is_magnetized = false;
                    // Release with a small nudge to avoid sticking
                    ball.vx = 2.0;
                    ball.vy = -4.0;
                }
            }
        }

        // Remove expired power-ups
        self.state
            .active_powerups
            .retain(|p| p.remaining_frames > 0);

        // Handle paddle state - extend is permanent, shrink is also permanent (until extend is collected)
        // Do not auto-reset paddle width here anymore - only happens on level change or power-up collection

        self.state.powerups.retain(|p| p.active);
    }

    fn check_collisions(&mut self) {
        let mut pending_score = 0;
        let mut destroyed_bricks = 0;

        // Check ball-paddle collisions
        for ball in &mut self.state.balls {
            if crate::physics::check_ball_paddle_collision(ball, &self.state.paddle) {
                // [NEW] Play paddle hit sound
                self.state.audio.play_paddle_hit();

                // Emit particles on paddle hit
                self.state.particle_system.paddle_hit(
                    ball.x,
                    ball.y,
                    self.state.theme_colors.paddle,
                );
            }
        }

        // Check ball-brick collisions
        for idx in 0..self.state.bricks.len() {
            if !self.state.bricks[idx].active {
                continue;
            }
            for ball_idx in 0..self.state.balls.len() {
                let (
                    collided,
                    destroyed_now,
                    was_exploding,
                    brick_x,
                    brick_y,
                    brick_color,
                    brick_type,
                ) = {
                    let brick = &mut self.state.bricks[idx];
                    let ball = &mut self.state.balls[ball_idx];
                    let was_active = brick.active;
                    let was_exploding = brick.brick_type == BrickType::Exploding;
                    let collided = crate::physics::check_ball_brick_collision(ball, brick);
                    (
                        collided,
                        was_active && !brick.active,
                        was_exploding,
                        brick.x,
                        brick.y,
                        brick.color,
                        brick.brick_type,
                    )
                };

                if collided {
                    if brick_type == BrickType::Steel && !destroyed_now {
                        self.play_brick_feedback_sound(brick_type, false);
                    }
                    if destroyed_now {
                        pending_score += BRICK_POINTS;
                        destroyed_bricks += 1;

                        self.state.score_popups.push(crate::types::ScorePopup {
                            x: brick_x + BRICK_WIDTH / 2.0,
                            y: brick_y + BRICK_HEIGHT / 2.0,
                            value: BRICK_POINTS as i32,
                            lifetime: 1.0,
                            max_lifetime: 1.0,
                        });

                        self.play_brick_feedback_sound(brick_type, true);
                        self.emit_destroyed_brick_feedback(
                            brick_type,
                            brick_x + BRICK_WIDTH / 2.0,
                            brick_y + BRICK_HEIGHT / 2.0,
                            brick_color,
                        );

                        if was_exploding {
                            let (chain_score, chain_destroyed) =
                                self.trigger_exploding_brick_chain(idx);
                            pending_score += chain_score;
                            destroyed_bricks += chain_destroyed;
                        }
                    }

                    // Spawn power-up with difficulty-adjusted chance
                    let spawn_chance = self.state.difficulty.powerup_spawn_chance();
                    let spawn_rand =
                        ((self.state.frame_count as f32 * 12.347 + idx as f32 * 53.891) % 100.0)
                            / 100.0;
                    if spawn_rand < spawn_chance {
                        let power_type = match (self.state.frame_count + idx) % 8 {
                            0 => PowerUpType::MultiBall,
                            1 => PowerUpType::PaddleExtend,
                            2 => PowerUpType::SlowTime,
                            3 => PowerUpType::Laser,        // [NEW]
                            4 => PowerUpType::Shield,       // [NEW]
                            5 => PowerUpType::Bomb,         // [NEW]
                            6 => PowerUpType::Magnetize,    // [NEW]
                            _ => PowerUpType::PaddleShrink, // [NEW] Power-down
                        };

                        // Emit particles for power-up spawn
                        self.state.particle_system.power_up_spawn(
                            brick_x + BRICK_WIDTH / 2.0,
                            brick_y,
                            self.state.theme_colors.accent,
                        );

                        self.state.powerups.push(PowerUp {
                            x: brick_x + BRICK_WIDTH / 2.0,
                            y: brick_y,
                            power_type,
                            active: true,
                        });
                    }
                    break;
                }
            }
        }

        // Update Regenerating bricks (respawn after delay)
        for brick in &mut self.state.bricks {
            if !brick.active && brick.brick_type == BrickType::Regenerating && brick.regen_timer > 0
            {
                brick.regen_timer -= 1;
                if brick.regen_timer == 0 {
                    brick.active = true;
                    brick.is_hit = false;
                    self.state.audio.play_regenerating_respawn();
                    self.state.particle_system.regenerating_respawn(
                        brick.x + brick.width / 2.0,
                        brick.y + brick.height / 2.0,
                    );
                }
            }
        }

        // Check power-up pickups
        let mut powerups_to_apply = Vec::new();
        for powerup in &mut self.state.powerups {
            if !powerup.active {
                continue;
            }
            if crate::physics::check_powerup_pickup(powerup, &self.state.paddle) {
                powerup.active = false;

                // [UPDATED] Play power-up or power-down sound based on type
                match powerup.power_type {
                    PowerUpType::PaddleShrink => {
                        self.state.audio.play_paddle_shrink(); // [NEW] Special sound for power-down
                    }
                    _ => {
                        self.state.audio.play_powerup_pickup(); // Regular power-up sound
                    }
                }

                // Emit particles for power-up pickup
                self.state.particle_system.power_up_pickup(
                    powerup.x,
                    powerup.y,
                    self.state.theme_colors.primary,
                );

                powerups_to_apply.push(powerup.power_type);
            }
        }

        for power_type in powerups_to_apply {
            self.track_powerup_pickup_achievements();
            self.apply_powerup(power_type);
        }

        // [NEW] Handle laser collisions with bricks
        let mut lasers_to_remove = Vec::new();
        for laser_idx in 0..self.state.laser_shots.len() {
            if !self.state.laser_shots[laser_idx].active {
                continue;
            }

            // Move laser up
            self.state.laser_shots[laser_idx].y -= LASER_SPEED;

            // Remove if off-screen
            if self.state.laser_shots[laser_idx].y < 0.0 {
                self.state.laser_shots[laser_idx].active = false;
                continue;
            }

            // Check collisions with bricks
            for brick_idx in 0..self.state.bricks.len() {
                let brick = &self.state.bricks[brick_idx];
                if !brick.active {
                    continue;
                }

                let laser = &self.state.laser_shots[laser_idx];
                // Simple rect-rect collision
                if laser.x < brick.x + brick.width
                    && laser.x + laser.width > brick.x
                    && laser.y < brick.y + brick.height
                    && laser.y + laser.height > brick.y
                {
                    let was_exploding =
                        self.state.bricks[brick_idx].brick_type == BrickType::Exploding;
                    let destroyed = self.resolve_auxiliary_brick_hit(brick_idx);
                    self.state.laser_shots[laser_idx].active = false;
                    lasers_to_remove.push(laser_idx);

                    if destroyed {
                        let brick = &self.state.bricks[brick_idx];
                        pending_score += BRICK_POINTS;
                        destroyed_bricks += 1;
                        self.play_brick_feedback_sound(brick.brick_type, true);
                        self.emit_destroyed_brick_feedback(
                            brick.brick_type,
                            brick.x + BRICK_WIDTH / 2.0,
                            brick.y + BRICK_HEIGHT / 2.0,
                            brick.color,
                        );

                        if was_exploding {
                            let (chain_score, chain_destroyed) =
                                self.trigger_exploding_brick_chain(brick_idx);
                            pending_score += chain_score;
                            destroyed_bricks += chain_destroyed;
                        }
                    }
                    break;
                }
            }
        }

        self.state.laser_shots.retain(|l| l.active);

        // [NEW] Handle shield - catch falling balls (stacking supported)
        if self.state.paddle.shield_count > 0 {
            for ball in &mut self.state.balls {
                if ball.y > PADDLE_Y && ball.y < SCREEN_HEIGHT {
                    // Ball hit the shield - restore it and decrement shield count
                    ball.active = true;
                    ball.vy = -4.0;
                    self.state.paddle.shield_count =
                        self.state.paddle.shield_count.saturating_sub(1);

                    // Emit particles
                    self.state.particle_system.power_up_pickup(
                        ball.x,
                        ball.y,
                        self.state.theme_colors.accent,
                    );
                    break;
                }
            }
        }

        if pending_score > 0 {
            self.add_score(pending_score);
        }
        if destroyed_bricks > 0 {
            self.track_brick_destruction_batch(destroyed_bricks);
        }
    }

    fn apply_powerup(&mut self, power_type: PowerUpType) {
        match power_type {
            PowerUpType::MultiBall => {
                if self.state.balls.len() < MAX_BALLS {
                    let ball = self.state.balls[0].clone();
                    let mut ball1 = ball.clone();
                    let mut ball2 = ball.clone();
                    ball1.vx = ball.vx * 0.866 - ball.vy * 0.5; // -30°
                    ball1.vy = ball.vy * 0.866 + ball.vx * 0.5;
                    ball2.vx = ball.vx * 0.866 + ball.vy * 0.5; // +30°
                    ball2.vy = ball.vy * 0.866 - ball.vx * 0.5;
                    self.state.balls.push(ball1);
                    if self.state.balls.len() < MAX_BALLS {
                        self.state.balls.push(ball2);
                    }
                }
            }
            PowerUpType::PaddleExtend => {
                self.state.paddle.is_extended = true;
                // Extended width is 1.5x the normal width for this difficulty
                self.state.paddle.width = self.state.paddle.extended_width;
                self.state.paddle.is_shrunk = false; // Cancel shrink if active
                                                     // [CHANGED] No longer add to active_powerups - extend is PERMANENT until next level or shrink
            }
            PowerUpType::SlowTime => {
                // Stacking policy: Extend existing slow time duration if active; otherwise start a new one.
                if let Some(active) = self
                    .state
                    .active_powerups
                    .iter_mut()
                    .find(|p| p.power_type == PowerUpType::SlowTime)
                {
                    active.remaining_frames =
                        active.remaining_frames.saturating_add(POWERUP_DURATION);
                } else {
                    self.state
                        .active_powerups
                        .push(crate::types::ActivePowerUp {
                            power_type,
                            remaining_frames: POWERUP_DURATION,
                        });
                }
                self.increment_achievement_progress(AchievementId::TimeBender, 1);
            }
            PowerUpType::Laser => {
                // [NEW] Activate laser mode
                // Stacking policy: Refresh existing laser duration if active; otherwise start a new one.
                if let Some(active) = self
                    .state
                    .active_powerups
                    .iter_mut()
                    .find(|p| p.power_type == PowerUpType::Laser)
                {
                    active.remaining_frames = POWERUP_LASER_DURATION;
                } else {
                    self.state
                        .active_powerups
                        .push(crate::types::ActivePowerUp {
                            power_type,
                            remaining_frames: POWERUP_LASER_DURATION,
                        });
                }
            }
            PowerUpType::Shield => {
                // [NEW] Grant shield to paddle (stacking allowed)
                self.state.paddle.shield_count = self.state.paddle.shield_count.saturating_add(1);
            }
            PowerUpType::Bomb => {
                // [NEW] Trigger bomb explosion at paddle position
                self.trigger_bomb_explosion();
            }
            PowerUpType::Magnetize => {
                // [NEW] Magnetize the first ball to the paddle
                // Stacking policy: Extend existing magnetize duration if active; otherwise start a new one.
                if !self.state.balls.is_empty() {
                    self.state.balls[0].is_magnetized = true;
                    if let Some(active) = self
                        .state
                        .active_powerups
                        .iter_mut()
                        .find(|p| p.power_type == PowerUpType::Magnetize)
                    {
                        active.remaining_frames = active
                            .remaining_frames
                            .saturating_add(POWERUP_MAGNETIZE_DURATION);
                    } else {
                        self.state
                            .active_powerups
                            .push(crate::types::ActivePowerUp {
                                power_type,
                                remaining_frames: POWERUP_MAGNETIZE_DURATION,
                            });
                    }
                }
            }
            PowerUpType::PaddleShrink => {
                // [NEW] Shrink paddle and play audio
                self.state.paddle.is_shrunk = true;
                self.state.paddle.width = self.state.paddle.shrunk_width;
                self.state.paddle.is_extended = false; // Cancel extend if active
                self.state.audio.play_paddle_shrink(); // [NEW] Play shrink sound
            }
        }
    }

    fn trigger_bomb_explosion(&mut self) {
        // Destroy all bricks in 3x3 area around paddle position
        let paddle_center_x = self.state.paddle.x + self.state.paddle.width / 2.0;
        let bomb_radius = 90.0; // 3 brick widths
        let mut destroyed_score = 0;
        let mut destroyed_bricks = 0;

        let mut targets = Vec::new();
        for (brick_idx, brick) in self.state.bricks.iter().enumerate() {
            if !brick.active {
                continue;
            }
            let brick_center_x = brick.x + BRICK_WIDTH / 2.0;
            let brick_center_y = brick.y + BRICK_HEIGHT / 2.0;
            let dx = (brick_center_x - paddle_center_x).abs();
            let dy = (brick_center_y - PADDLE_Y).abs();

            // Check if brick is in explosion radius
            if dx < bomb_radius && dy < bomb_radius * 1.5 {
                targets.push(brick_idx);
            }
        }

        for brick_idx in targets {
            let was_exploding = self.state.bricks[brick_idx].brick_type == BrickType::Exploding;
            let destroyed = self.resolve_auxiliary_brick_hit(brick_idx);
            if destroyed {
                let brick = &self.state.bricks[brick_idx];
                destroyed_score += BRICK_POINTS;
                destroyed_bricks += 1;
                self.play_brick_feedback_sound(brick.brick_type, true);
                self.emit_destroyed_brick_feedback(
                    brick.brick_type,
                    brick.x + BRICK_WIDTH / 2.0,
                    brick.y + BRICK_HEIGHT / 2.0,
                    brick.color,
                );

                if was_exploding {
                    let (chain_score, chain_destroyed) =
                        self.trigger_exploding_brick_chain(brick_idx);
                    destroyed_score += chain_score;
                    destroyed_bricks += chain_destroyed;
                }
            }
        }

        if destroyed_score > 0 {
            self.add_score(destroyed_score);
        }
        if destroyed_bricks > 0 {
            self.track_brick_destruction_batch(destroyed_bricks);
        }
    }

    fn check_game_conditions(&mut self) {
        // Check if all bricks destroyed
        let active_bricks = self.state.bricks.iter().filter(|b| b.active).count();
        if active_bricks == 0 {
            self.state.phase = GamePhase::LevelComplete;
            self.add_score(LEVEL_COMPLETE_BONUS);
            self.state.level_complete_timer = 120; // 2 seconds
            self.check_level_completion_achievements();

            // [NEW] Play level complete sound
            self.state.audio.play_level_complete();
        }

        // Check if no balls left
        if self.state.balls.is_empty() && self.state.phase == GamePhase::Playing {
            self.state.lives = 0;
            self.state.phase = GamePhase::GameOver;
            self.record_high_score();
            self.save_achievements();

            // Play game over sound and stop music
            self.state.audio.play_game_over();
            self.state.audio.stop_music();
        }
    }

    fn update_level_complete(&mut self) {
        self.update_dev_menu();
        if self.state.dev_tools.open {
            return;
        }

        if self.state.level_complete_timer > 0 {
            self.state.level_complete_timer -= 1;
        } else {
            if self.state.level < NUM_LEVELS {
                self.load_level(self.state.level + 1);
                self.state.phase = GamePhase::Playing;
            } else {
                self.state.phase = GamePhase::Victory;
                self.add_score(ALL_LEVELS_BONUS);

                // Play victory sound and stop music
                self.state.audio.play_victory();
                self.state.audio.stop_music();

                self.check_victory_achievements();
                self.record_high_score();
                self.save_achievements();
            }
        }
    }

    fn update_game_over(&mut self) {
        self.update_dev_menu();
        if self.state.dev_tools.open {
            return;
        }
        if is_key_pressed(KeyCode::Space) {
            self.start_menu();
        }
    }

    pub fn render(&self) {
        match self.state.phase {
            GamePhase::MainMenu => {
                crate::ui::render_main_menu(&self.state);
            }
            GamePhase::Playing => {
                crate::ui::render_game(&self.state);
            }
            GamePhase::LevelComplete => {
                crate::ui::render_game(&self.state);
                crate::ui::render_level_complete(&self.state);
            }
            GamePhase::GameOver => {
                crate::ui::render_game_over(&self.state);
            }
            GamePhase::Victory => {
                crate::ui::render_victory(&self.state);
            }
        }
    }
}
