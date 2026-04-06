use crate::constants::*;
use crate::types::{Ball, Brick, BrickType, Paddle, PowerUp};

pub fn check_ball_brick_collision(ball: &mut Ball, brick: &mut Brick) -> bool {
    if !brick.active {
        return false;
    }

    // Closest point on brick to ball center
    let closest_x = ball.x.max(brick.x).min(brick.x + brick.width);
    let closest_y = ball.y.max(brick.y).min(brick.y + brick.height);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    let distance = (dx * dx + dy * dy).sqrt();

    if distance < ball.radius {
        // Collision detected - handle based on brick type
        match brick.brick_type {
            BrickType::Normal => {
                brick.active = false;
            }
            BrickType::Frozen => {
                // Apply slow effect to ball
                ball.speed_multiplier = FROZEN_SPEED_REDUCTION;
                ball.frozen_timer = FROZEN_DURATION;
                brick.active = false;
            }
            BrickType::Exploding => {
                brick.active = false;
            }
            BrickType::Steel => {
                if brick.health > 0 {
                    brick.health -= 1;
                    if brick.health == 0 {
                        brick.active = false;
                    }
                }
            }
            BrickType::Regenerating => {
                brick.active = false;
                brick.is_hit = true;
                brick.regen_timer = REGENERATING_DURATION;
            }
        }

        // Determine entry side and bounce
        if dx.abs() > dy.abs() {
            ball.vx = -ball.vx;
        } else {
            ball.vy = -ball.vy;
        }

        return true;
    }

    false
}

pub fn check_ball_paddle_collision(ball: &mut Ball, paddle: &Paddle) -> bool {
    if !ball.active {
        return false;
    }

    // Only check collision if ball is moving downward toward paddle
    if ball.vy <= 0.0 {
        return false;
    }

    // Closest point on paddle to ball center
    let closest_x = ball.x.max(paddle.x).min(paddle.x + paddle.width);
    let closest_y = ball.y.max(paddle.y).min(paddle.y + paddle.height);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    let distance = (dx * dx + dy * dy).sqrt();

    if distance < ball.radius {
        // Calculate hit position (-1.0 = left edge, +1.0 = right edge)
        let hit_pos = (ball.x - (paddle.x + paddle.width / 2.0)) / (paddle.width / 2.0);

        // Reverse vertical velocity and apply angle variation
        ball.vy = -ball.vy.abs();
        ball.vx = hit_pos * 2.5;

        // Clamp horizontal velocity to prevent excessive angles
        ball.vx = ball.vx.clamp(-BALL_MAX_SPEED * 0.7, BALL_MAX_SPEED * 0.7);

        return true;
    }

    false
}

pub fn check_powerup_pickup(powerup: &PowerUp, paddle: &Paddle) -> bool {
    if !powerup.active {
        return false;
    }

    // Simple circle-rectangle collision
    let closest_x = powerup.x.max(paddle.x).min(paddle.x + paddle.width);
    let closest_y = powerup.y.max(paddle.y).min(paddle.y + paddle.height);

    let dx = powerup.x - closest_x;
    let dy = powerup.y - closest_y;

    let distance = (dx * dx + dy * dy).sqrt();

    distance < (POWERUP_WIDTH / 2.0 + paddle.height / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Ball, Brick, Paddle, PowerUp, PowerUpType};

    #[test]
    fn test_ball_paddle_collision() {
        // Setup ball moving downward toward paddle
        // Ball needs to actually overlap with paddle for collision detection
        let mut ball = Ball {
            x: 100.0,
            y: 548.0, // Deep enough to overlap (ball radius=5, paddle top=550)
            vx: 2.0,
            vy: 2.0, // moving down
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        };

        let mut paddle = Paddle {
            x: 90.0,
            y: PADDLE_Y,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            normal_width: PADDLE_WIDTH,
            extended_width: PADDLE_WIDTH * 1.5,
            is_extended: false,
            is_shrunk: false,
            shrunk_width: PADDLE_WIDTH * 0.6,
            shield_count: 0,
            magnetized_ball: None,
        };

        // Should collide
        assert!(check_ball_paddle_collision(&mut ball, &mut paddle));

        // After collision, ball should move upward
        assert!(ball.vy < 0.0);
    }

    #[test]
    fn test_ball_paddle_no_collision_when_moving_up() {
        // Setup ball moving upward (away from paddle)
        let mut ball = Ball {
            x: 140.0,                        // center within paddle width
            y: PADDLE_Y - BALL_RADIUS + 1.0, // just inside paddle top
            vx: 2.0,
            vy: -2.0, // moving up
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        };

        let mut paddle = Paddle {
            x: 90.0,
            y: PADDLE_Y,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            normal_width: PADDLE_WIDTH,
            extended_width: PADDLE_WIDTH * 1.5,
            is_extended: false,
            is_shrunk: false,
            shrunk_width: PADDLE_WIDTH * 0.6,
            shield_count: 0,
            magnetized_ball: None,
        };

        // Should NOT collide when moving up (only collides when moving down)
        assert!(!check_ball_paddle_collision(&mut ball, &mut paddle));
    }

    #[test]
    fn test_ball_brick_collision() {
        let mut ball = Ball {
            x: 50.0,
            y: 50.0,
            vx: 2.0,
            vy: 2.0,
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        };

        let mut brick = Brick {
            x: 40.0,
            y: 40.0,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            active: true,
            color: RED,
            brick_type: BrickType::Normal,
            health: 0,
            regen_timer: 0,
            is_hit: false,
        };

        // Should collide
        assert!(check_ball_brick_collision(&mut ball, &mut brick));

        // Brick should be deactivated
        assert!(!brick.active);
    }

    #[test]
    fn test_ball_brick_no_collision_when_inactive() {
        let mut ball = Ball {
            x: 50.0,
            y: 50.0,
            vx: 2.0,
            vy: 2.0,
            radius: BALL_RADIUS,
            active: true,
            is_magnetized: false,
            speed_multiplier: 1.0,
            frozen_timer: 0,
        };

        let mut brick = Brick {
            x: 40.0,
            y: 40.0,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            active: false, // inactive
            color: RED,
            brick_type: BrickType::Normal,
            health: 0,
            regen_timer: 0,
            is_hit: false,
        };

        // Should NOT collide with inactive brick
        assert!(!check_ball_brick_collision(&mut ball, &mut brick));
        // Brick should remain inactive
        assert!(!brick.active);
    }

    #[test]
    fn test_powerup_pickup() {
        let powerup = PowerUp {
            x: 100.0,
            y: 100.0,
            power_type: PowerUpType::MultiBall,
            active: true,
        };

        let paddle = Paddle {
            x: 90.0,
            y: 90.0,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            normal_width: PADDLE_WIDTH,
            extended_width: PADDLE_WIDTH * 1.5,
            is_extended: false,
            is_shrunk: false,
            shrunk_width: PADDLE_WIDTH * 0.6,
            shield_count: 0,
            magnetized_ball: None,
        };

        // Should pickup when overlapping
        assert!(check_powerup_pickup(&powerup, &paddle));
    }

    #[test]
    fn test_powerup_no_pickup_when_inactive() {
        let powerup = PowerUp {
            x: 100.0,
            y: 100.0,
            power_type: PowerUpType::MultiBall,
            active: false, // inactive
        };

        let paddle = Paddle {
            x: 90.0,
            y: 90.0,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            normal_width: PADDLE_WIDTH,
            extended_width: PADDLE_WIDTH * 1.5,
            is_extended: false,
            is_shrunk: false,
            shrunk_width: PADDLE_WIDTH * 0.6,
            shield_count: 0,
            magnetized_ball: None,
        };

        // Should NOT pickup when inactive
        assert!(!check_powerup_pickup(&powerup, &paddle));
    }
}
