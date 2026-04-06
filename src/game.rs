use crate::constants::*;
use crate::types::{Ball, PowerUp, PowerUpType, GameState, GamePhase};
use crate::level;
use macroquad::prelude::*;

pub struct Game {
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            state: GameState::new(),
        };
        game.start_menu();
        game
    }

    pub fn start_menu(&mut self) {
        self.state.phase = GamePhase::MainMenu;
        self.state.score = 0;
        self.state.lives = self.state.difficulty.starting_lives();
        self.state.level = 1;
        self.state.audio.stop_music();
    }

    pub fn start_game(&mut self) {
        self.state.phase = GamePhase::Playing;
        self.state.audio.start_music();
        self.load_level(self.state.level);
    }

    pub fn load_level(&mut self, level_num: usize) {
        self.state.level = level_num.min(NUM_LEVELS);
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
        self.state.paddle.has_shield = false;
        self.state.paddle.magnetized_ball = None;
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
        }

        // Handle volume control (+ and - keys)
        if is_key_pressed(KeyCode::Equal) {
            self.state.audio.increase_volume();
        }
        if is_key_pressed(KeyCode::Minus) {
            self.state.audio.decrease_volume();
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
        }
        if self.state.gamepad.is_rb_pressed() {
            self.state.audio.increase_volume();
        }

        // Music toggle (Back/Select button on gamepad)
        if self.state.gamepad.is_select_pressed() {
            self.state.audio.toggle_music();
        }

        // Update particle system regardless of pause
        self.state.particle_system.update(1.0 / 60.0);

        // Skip game updates if paused
        if self.state.is_paused {
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
                let release = is_key_pressed(KeyCode::Space) || self.state.gamepad.is_south_pressed();
                
                if release {
                    ball.is_magnetized = false;
                    ball.vx = 2.0;
                    ball.vy = -4.0;
                }
                continue; // Skip normal physics for magnetized ball
            }

            // Apply slow time power-up effect
            let speed_multiplier = if self.state.active_powerups.iter().any(|p| p.power_type == PowerUpType::SlowTime) {
                0.5
            } else {
                1.0
            };

            // Update position
            ball.x += ball.vx * speed_multiplier;
            ball.y += ball.vy * speed_multiplier;

            // Wall collisions
            if ball.x <= BALL_RADIUS || ball.x >= SCREEN_WIDTH - BALL_RADIUS {
                ball.vx = -ball.vx;
                ball.x = ball.x.max(BALL_RADIUS).min(SCREEN_WIDTH - BALL_RADIUS);
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
        if self.state.active_powerups.iter().any(|p| p.power_type == PowerUpType::Laser) {
            // Fire laser from paddle every frame when active
            if self.state.frame_count % 6 == 0 {  // Fire every 6 frames (10 shots/sec)
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
            self.state.lives -= 1;

            if self.state.lives == 0 {
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

        // Remove expired power-ups
        self.state.active_powerups.retain(|p| p.remaining_frames > 0);

        // Handle paddle state - extend is permanent, shrink is also permanent (until extend is collected)
        // Do not auto-reset paddle width here anymore - only happens on level change or power-up collection

        self.state.powerups.retain(|p| p.active);
    }

    fn check_collisions(&mut self) {
        // Check ball-paddle collisions
        for ball in &mut self.state.balls {
            if crate::physics::check_ball_paddle_collision(ball, &mut self.state.paddle) {
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
        let mut bricks_to_destroy = Vec::new();
        for (idx, brick) in self.state.bricks.iter_mut().enumerate() {
            if !brick.active {
                continue;
            }
            for ball in &mut self.state.balls {
                if crate::physics::check_ball_brick_collision(ball, brick) {
                    bricks_to_destroy.push(idx);
                    self.state.score += BRICK_POINTS;

                    // [NEW] Play brick destroy sound
                    self.state.audio.play_brick_destroy();

                    // Emit particles on brick destruction
                    self.state.particle_system.brick_destruction(
                        brick.x + BRICK_WIDTH / 2.0,
                        brick.y + BRICK_HEIGHT / 2.0,
                        brick.color,
                    );

                    // Spawn power-up with difficulty-adjusted chance
                    let spawn_chance = self.state.difficulty.powerup_spawn_chance();
                    let spawn_rand = ((self.state.frame_count as f32 * 12.347 + idx as f32 * 53.891) % 100.0) / 100.0;
                    if spawn_rand < spawn_chance {
                        let power_type = match (self.state.frame_count + idx) % 8 {
                            0 => PowerUpType::MultiBall,
                            1 => PowerUpType::PaddleExtend,
                            2 => PowerUpType::SlowTime,
                            3 => PowerUpType::Laser,      // [NEW]
                            4 => PowerUpType::Shield,     // [NEW]
                            5 => PowerUpType::Bomb,       // [NEW]
                            6 => PowerUpType::Magnetize,  // [NEW]
                            _ => PowerUpType::PaddleShrink, // [NEW] Power-down
                        };
                        
                        // Emit particles for power-up spawn
                        self.state.particle_system.power_up_spawn(
                            brick.x + BRICK_WIDTH / 2.0,
                            brick.y,
                            self.state.theme_colors.accent,
                        );
                        
                        self.state.powerups.push(PowerUp {
                            x: brick.x + BRICK_WIDTH / 2.0,
                            y: brick.y,
                            power_type,
                            active: true,
                        });
                    }
                    break;
                }
            }
        }

        for idx in bricks_to_destroy {
            self.state.bricks[idx].active = false;
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
            self.apply_powerup(power_type);
        }

        // [NEW] Handle laser collisions with bricks
        let mut lasers_to_remove = Vec::new();
        for (laser_idx, laser) in self.state.laser_shots.iter_mut().enumerate() {
            if !laser.active {
                continue;
            }

            // Move laser up
            laser.y -= LASER_SPEED;

            // Remove if off-screen
            if laser.y < 0.0 {
                laser.active = false;
                continue;
            }

            // Check collisions with bricks
            for brick in self.state.bricks.iter_mut() {
                if !brick.active {
                    continue;
                }

                // Simple rect-rect collision
                if laser.x < brick.x + brick.width
                    && laser.x + laser.width > brick.x
                    && laser.y < brick.y + brick.height
                    && laser.y + laser.height > brick.y
                {
                    brick.active = false;
                    self.state.score += BRICK_POINTS;
                    laser.active = false;
                    lasers_to_remove.push(laser_idx);

                    // Emit particles
                    self.state.particle_system.brick_destruction(
                        brick.x + BRICK_WIDTH / 2.0,
                        brick.y + BRICK_HEIGHT / 2.0,
                        brick.color,
                    );
                    break;
                }
            }
        }

        self.state.laser_shots.retain(|l| l.active);

        // [NEW] Handle shield - catch falling balls
        if self.state.paddle.has_shield {
            for ball in &mut self.state.balls {
                if ball.y > PADDLE_Y && ball.y < SCREEN_HEIGHT {
                    // Ball hit the shield - restore it and remove shield
                    ball.active = true;
                    ball.vy = -4.0;
                    self.state.paddle.has_shield = false;
                    
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
                self.state.achievements.increment_progress(crate::achievements::AchievementId::MultiBallMaster, 1);
            }
            PowerUpType::PaddleExtend => {
                self.state.paddle.is_extended = true;
                // Extended width is 1.5x the normal width for this difficulty
                self.state.paddle.width = self.state.paddle.extended_width;
                self.state.paddle.is_shrunk = false; // Cancel shrink if active
                // [CHANGED] No longer add to active_powerups - extend is PERMANENT until next level or shrink
                self.state.achievements.increment_progress(crate::achievements::AchievementId::PowerUpHoarder, 1);
            }
            PowerUpType::SlowTime => {
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_DURATION,
                });
                self.state.achievements.increment_progress(crate::achievements::AchievementId::TimeBender, 1);
            }
            PowerUpType::Laser => {
                // [NEW] Activate laser mode
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_LASER_DURATION,
                });
            }
            PowerUpType::Shield => {
                // [NEW] Grant shield to paddle
                self.state.paddle.has_shield = true;
            }
            PowerUpType::Bomb => {
                // [NEW] Trigger bomb explosion at paddle position
                self.trigger_bomb_explosion();
            }
            PowerUpType::Magnetize => {
                // [NEW] Magnetize the first ball to the paddle
                if !self.state.balls.is_empty() {
                    self.state.balls[0].is_magnetized = true;
                    self.state.active_powerups.push(crate::types::ActivePowerUp {
                        power_type,
                        remaining_frames: POWERUP_MAGNETIZE_DURATION,
                    });
                }
            }
            PowerUpType::PaddleShrink => {
                // [NEW] Shrink paddle and play audio
                self.state.paddle.is_shrunk = true;
                self.state.paddle.width = self.state.paddle.shrunk_width;
                self.state.paddle.is_extended = false; // Cancel extend if active
                self.state.audio.play_paddle_shrink(); // [NEW] Play shrink sound
                self.state.achievements.increment_progress(crate::achievements::AchievementId::PowerUpHoarder, 1);
            }
        }
    }

    fn trigger_bomb_explosion(&mut self) {
        // Destroy all bricks in 3x3 area around paddle position
        let paddle_center_x = self.state.paddle.x + self.state.paddle.width / 2.0;
        let bomb_radius = 90.0; // 3 brick widths
        
        for brick in &mut self.state.bricks {
            if !brick.active {
                continue;
            }
            let brick_center_x = brick.x + BRICK_WIDTH / 2.0;
            let brick_center_y = brick.y + BRICK_HEIGHT / 2.0;
            let dx = (brick_center_x - paddle_center_x).abs();
            let dy = (brick_center_y - PADDLE_Y).abs();
            
            // Check if brick is in explosion radius
            if dx < bomb_radius && dy < bomb_radius * 1.5 {
                brick.active = false;
                self.state.score += BRICK_POINTS;
                self.state.particle_system.brick_destruction(
                    brick_center_x,
                    brick_center_y,
                    brick.color,
                );
            }
        }
    }

    fn check_game_conditions(&mut self) {
        // Check if all bricks destroyed
        let active_bricks = self.state.bricks.iter().filter(|b| b.active).count();
        if active_bricks == 0 {
            self.state.phase = GamePhase::LevelComplete;
            self.state.score += LEVEL_COMPLETE_BONUS;
            self.state.level_complete_timer = 120; // 2 seconds
            
            // [NEW] Play level complete sound
            self.state.audio.play_level_complete();
            
            // Track level completion for speedrunner achievement
            self.state.achievements.increment_progress(crate::achievements::AchievementId::Speedrunner, 1);
        }

        // Check if no balls left
        if self.state.balls.is_empty() && self.state.phase == GamePhase::Playing {
            self.state.lives = 0;
            self.state.phase = GamePhase::GameOver;
            
            // Play game over sound and stop music
            self.state.audio.play_game_over();
            self.state.audio.stop_music();
        }
    }

    fn update_level_complete(&mut self) {
        if self.state.level_complete_timer > 0 {
            self.state.level_complete_timer -= 1;
        } else {
            if self.state.level < NUM_LEVELS {
                self.load_level(self.state.level + 1);
                self.state.phase = GamePhase::Playing;
            } else {
                self.state.phase = GamePhase::Victory;
                self.state.score += ALL_LEVELS_BONUS;
                
                // Play victory sound and stop music
                self.state.audio.play_victory();
                self.state.audio.stop_music();
                
                // Check for PerfectClear (beat all levels without losing a life)
                if self.state.lives == 3 {
                    // Assuming we started with 3 lives
                    self.state.achievements.unlock(crate::achievements::AchievementId::PerfectClear);
                }
                
                if self.state.score > self.state.high_score {
                    self.state.high_score = self.state.score;
                }
            }
        }
    }

    fn update_game_over(&mut self) {
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
