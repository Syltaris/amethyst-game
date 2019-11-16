use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

// import some stuff from pong
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

pub mod bounce;
pub mod move_balls;

// System Descriptor unit struct
// Systems must have SystemDesc trait to specify logic for System instantiation
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    // data system operates on
    // describes what kind of world resources are required to power the system
    // Mutates Transform components
    // Reads Paddle components
    // also assesses InputHandler resource
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        // unpack the SystemData object
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            // performs join over Transform and Paddle storages
            // iterates over all entities that have both a Paddle and Transform attached to them
            // while giving mutable access to Transform, immutable access to Paddle
            // possible to use par_join for multi threaded, but not worth overhead here
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            // if there is change for the paddle, apply the transform for it
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + scaled_amount)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5),
                ); // clamps the paddle within arena boundaries
            } // would usually use amethyse::core::timing::Time to get time diff between frames for consistency
        }
    }
}
