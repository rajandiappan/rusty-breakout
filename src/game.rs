use crate::constants::*;
use crate::types::{Ball, Paddle, Brick, PowerUp, PowerUpType, GameState, GamePhase};
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
        self.state.lives = STARTING_LIVES;
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

        // Create initial ball
        let initial_ball = Ball {
            x: SCREEN_WIDTH / 2.0,
            y: PADDLE_Y - 30.0,
            vx: 2.0,
            vy: -4.0,
            radius: BALL_RADIUS,
            active: true,
        };
        self.state.balls = vec![initial_ball];

        // Reset paddle
        self.state.paddle.x = (SCREEN_WIDTH - PADDLE_WIDTH) / 2.0;
        self.state.paddle.width = PADDLE_WIDTH;
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
            self.state.paddle.width = PADDLE_WIDTH;
        }

        self.state.powerups.retain(|p| p.active);
    }

    fn check_collisions(&mut self) {
        // Check ball-paddle collisions
        for ball in &mut self.state.balls {
            crate::physics::check_ball_paddle_collision(ball, &mut self.state.paddle);
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

                    // Spawn power-up
                    if rand::random::<f32>() < POWERUP_SPAWN_CHANCE {
                        let power_type = match rand::random::<u32>() % 3 {
                            0 => PowerUpType::MultiBall,
                            1 => PowerUpType::PaddleExtend,
                            _ => PowerUpType::SlowTime,
                        };
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
        for powerup in &mut self.state.powerups {
            if !powerup.active {
                continue;
            }
            if crate::physics::check_powerup_pickup(powerup, &self.state.paddle) {
                powerup.active = false;
                self.apply_powerup(powerup.power_type);
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
            }
            PowerUpType::PaddleExtend => {
                self.state.paddle.is_extended = true;
                self.state.paddle.width = PADDLE_EXTENDED_WIDTH;
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_DURATION,
                });
            }
            PowerUpType::SlowTime => {
                self.state.active_powerups.push(crate::types::ActivePowerUp {
                    power_type,
                    remaining_frames: POWERUP_DURATION,
                });
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
