use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// dimensions of playable area
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0); // x, y, z
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT)) // orthographic projection of size of arena
        .with(transform) // position in game world
        .build();
    // camera's looking at xy plane, where z is 0
}

// SimpleState: simplified version of State
// Implements stuff like update() and handle_event() for us.
// Especially handling the 'exit' signal -> closing the window.
impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Paddle>(); // configures storage for specific entity, there's a better way to do this

        initialise_camera(world);
        initialise_paddles(world);
    }
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

// this trait allows the class to be 'attached' to game entities
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>; // variations for fast access, low mem usage, etc. https://slide-rs.github.io/specs/05_storages.html#densevecstorage
}

fn initialise_paddles(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // correctly position paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // left plank
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // right plank
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}
