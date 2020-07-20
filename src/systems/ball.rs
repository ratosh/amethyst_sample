use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::game::{Ball, ARENA_HEIGHT, PlayerSide, Player};

#[derive(SystemDesc)]
pub struct BallMovementSystem;

impl<'s> System<'s> for BallMovementSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (ball, local) in (&balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity_x * time.delta_seconds());
            local.prepend_translation_y(ball.velocity_y * time.delta_seconds());
        }
    }
}


pub struct BallBounceSystem;

impl<'s> System<'s> for BallBounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, players, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity_y < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity_y > 0.0)
            {
                ball.velocity_y = -ball.velocity_y;
            }

            for (player, paddle_transform) in (&players, &transforms).join() {
                let player_x = paddle_transform.translation().x - (player.width * 0.5);
                let player_y = paddle_transform.translation().y - (player.height * 0.5);

                if point_in_rect(
                    ball_x,
                    ball_y,
                    player_x - ball.radius,
                    player_y - ball.radius,
                    player_x + player.width + ball.radius,
                    player_y + player.height + ball.radius,
                ) {
                    if (player.side == PlayerSide::Left && ball.velocity_x < 0.0)
                        || (player.side == PlayerSide::Right && ball.velocity_x > 0.0)
                    {
                        ball.velocity_x = -ball.velocity_x;
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}