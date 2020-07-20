use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::game::{PlayerSide, Player, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let movement = match player.side {
                PlayerSide::Left => input.axis_value("left_player"),
                PlayerSide::Right => input.axis_value("right_player"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let y = transform.translation().y;
                transform.set_translation_y(
                    (y + scaled_amount)
                        .min(ARENA_HEIGHT - player.height * 0.5)
                        .max(player.height * 0.5),
                );
            }
        }
    }
}