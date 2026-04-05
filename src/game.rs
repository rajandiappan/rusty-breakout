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
    }

    pub fn start_game(&mut self) {
        self.state.phase = GamePhase::Playing;
        self.load_level(self.state.level);
    }

    pub fn load_level(&mut self, level_num: usize) {
        self.state.level = level_num.min(NUM_LEVELS);
        self.state.bricks = level::create_level_bricks(level_num);
        self.state.powerups.clear();
        self.state.active_powerups.clear();

        // Apply difficulty multiplier to ball speed
        let speed_multiplier = self.state.difficulty.ball_speed_multiplier();
        let initial_ball = Ball {
            x: SCREEN_WIDTH / 2.0,
            y: PADDLE_Y - 30.0,
            vx: 2.0 * speed_multiplier,
            vy: -4.0 * speed_multiplier,
            radius: BALL_RADIUS,
            active: true,
        };
        self.state.balls = vec![initial_ball];

        // Reset paddle with difficulty-adjusted width
        let paddle_width = PADDLE_WIDTH * self.state.difficulty.paddle_width_multiplier();
        self.state.paddle.x = (SCREEN_WIDTH - paddle_width) / 2.0;
        self.state.paddle.width = paddle_width;
        self.state.paddle.normal_width = paddle_width;
        self.state.paddle.is_extended = false;
    }

    pub async fn update(&mut self) {
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
        if is_key_pressed(KeyCode::Space) {
            self.start_game();
        }
    }

    fn update_playing(&mut self) {
        // Handle pause toggle (P key)
        if is_key_pressed(KeyCode::P) {
            self.state.is_paused = !self.state.is_paused;
        }

        // Handle theme switching (T key)
        if is_key_pressed(KeyCode::T) {
            self.state.current_theme = self.state.current_theme.next();
            self.state.theme_colors = crate::themes::get_theme_colors(self.state.current_theme);
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
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.state.paddle.x -= PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
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

            // Bottom (lose life)
            if ball.y > SCREEN_HEIGHT {
                ball.active = false;
                self.state.lives -= 1;

                if self.state.lives == 0 {
                    self.state.phase = GamePhase::GameOver;
                } else {
                    // Reset ball
                    ball.x = SCREEN_WIDTH / 2.0;
                    ball.y = PADDLE_Y - 30.0;
                    ball.vx = 2.0;
                    ball.vy = -4.0;
                    ball.active = true;
                }
            }
        }

        // Remove inactive balls
        self.state.balls.retain(|b| b.active);
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

        // Handle paddle extension timeout
        if !self.state.active_powerups.iter().any(|p| p.power_type == PowerUpType::PaddleExtend) {
            self.state.paddle.is_extended = false;
            self.state.paddle.width = self.state.paddle.normal_width;
        }

        self.state.powerups.retain(|p| p.active);
    }

    fn check_collisions(&mut self) {
        // Check ball-paddle collisions
        for ball in &mut self.state.balls {
            if crate::physics::check_ball_paddle_collision(ball, &mut self.state.paddle) {
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
                        let power_type = match (self.state.frame_count + idx) % 3 {
                            0 => PowerUpType::MultiBall,
                            1 => PowerUpType::PaddleExtend,
                            _ => PowerUpType::SlowTime,
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
                self.state.paddle.width = self.state.paddle.normal_width * 1.5;
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_DURATION,
                });
                self.state.achievements.increment_progress(crate::achievements::AchievementId::PowerUpHoarder, 1);
            }
            PowerUpType::SlowTime => {
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_DURATION,
                });
                self.state.achievements.increment_progress(crate::achievements::AchievementId::TimeBender, 1);
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
            
            // Track level completion for speedrunner achievement
            self.state.achievements.increment_progress(crate::achievements::AchievementId::Speedrunner, 1);
        }

        // Check if no balls left
        if self.state.balls.is_empty() && self.state.phase == GamePhase::Playing {
            self.state.lives = 0;
            self.state.phase = GamePhase::GameOver;
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
