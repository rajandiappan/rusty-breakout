use crate::constants::*;
use crate::types::{Ball, Brick, Paddle, PowerUp};

pub fn check_ball_paddle_collision(ball: &mut Ball, paddle: &mut Paddle) -> bool {
    // Simple rectangle-circle collision detection
    let closest_x = ball.x.max(paddle.x).min(paddle.x + paddle.width);
    let closest_y = ball.y.max(paddle.y).min(paddle.y + paddle.height);

    let dx = ball.x - closest_x;
    let dy = ball.y - closest_y;

    let distance = (dx * dx + dy * dy).sqrt();

    if distance < ball.radius {
        // Collision detected
        // Only bounce if coming from above
        if ball.vy > 0.0 {
            // Reverse vertical velocity
            ball.vy = -ball.vy.abs();

            // Calculate hit position for horizontal spin
            let paddle_center = paddle.x + paddle.width / 2.0;
            let hit_offset = (ball.x - paddle_center) / (paddle.width / 2.0);
            let hit_offset = hit_offset.clamp(-1.0, 1.0);

            // Apply spin
            ball.vx = hit_offset * 2.5;

            // Clamp total speed
            let speed = (ball.vx * ball.vx + ball.vy * ball.vy).sqrt();
            if speed > BALL_MAX_SPEED {
                let scale = BALL_MAX_SPEED / speed;
                ball.vx *= scale;
                ball.vy *= scale;
            }

            return true;
        }
    }

    false
}

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
        // Collision detected
        brick.active = false;

        // Determine entry side
        if dx.abs() > dy.abs() {
            // Horizontal entry
            ball.vx = -ball.vx;
        } else {
            // Vertical entry
            ball.vy = -ball.vy;
        }

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
        };

        let mut brick = Brick {
            x: 40.0,
            y: 40.0,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            active: true,
            color: RED,
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
        };

        let mut brick = Brick {
            x: 40.0,
            y: 40.0,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            active: false, // inactive
            color: RED,
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
