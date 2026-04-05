use crate::constants::*;
use crate::types::{Ball, Paddle, Brick, PowerUp};

pub fn check_ball_paddle_collision(ball: &mut Ball, paddle: &Paddle) -> bool {
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
            let hit_offset = hit_offset.max(-1.0).min(1.0);

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
