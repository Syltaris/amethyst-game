use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

// import some stuff from pong
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// System Descriptor unit struct
// Systems must have SystemDesc trait to specify logic for System instantiation
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    // data system operates on
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
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving {}", side_name, mv_amount);
                }
            }
        }
    }
}
