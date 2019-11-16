use amethyst::{
    core::timing::Time, 
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}
};

use crate::pong::Ball;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = {
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    };

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        for (ball, local) in (&balls, &mut locals).join() { // for each ball, mutate it's location according to its current vectors
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
            // delta time is used here, gets duration since last frame (which accounts for actual time difference)
        }
    }
}