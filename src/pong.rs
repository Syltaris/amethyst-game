use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    audio::AudioBundle,
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::audio::initialise_audio;

// dimensions of playable area
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p2_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,  // x
        -50., // y
        1.,   // z
        200., //width
        50.,  // height, ... tab-order?
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),     // font
            "0".to_string(),  // initial value
            [1., 1., 1., 1.], // rgba colors
            50.,              // font_size
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(ScoreText { p1_score, p2_score });
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}
// needs own state since, multiple mutators need to use sprite_sheet_handle
// and also we only want the timer to be used once
// timer will count down to 0, and then be None
// Default allows creating default empty state

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
        // let sprite_sheet_handle = load_sprite_sheet(world);
        //        world.register::<Paddle>(); // configures storage for specific entity, there's a better way to do this
        // no longer needed since done in PaddleSystem, and registered via there
        // world.register::<Ball>();
        // initialise_ball(world, sprite_sheet_handle.clone());

        // wait 1 second before spawning ball
        self.ball_spawn_timer.replace(1.0);
        // shared loader
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap()); // need to unwrap the option
        initialise_camera(world);
        initialise_scoreboard(world);
        initialise_audio(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            /* .pop() */
            {
                let time = data.world.fetch::<Time>(); // fetch... implied time object?
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                // when timer hits zero
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                // if not expired yet, push() it back in
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None // this allows transitioning out of state (for now its None)
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

fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // correctly position paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);
    // assign sprites for paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // first sprite index
    };

    // left plank
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // right plank
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

// returns reference, for others to lazy 'read'
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // loads spritesheet needed for graphics
    // texture contains pixel data
    // 'texture_handle' is a cloneable ref to texture
    let texture_handle = {
        // sharable resource, loaded when app is built
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        // path, type,
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle), // refernce to source data?
        (),
        &sprite_sheet_store, // place to store data in?
    )
}

pub struct Ball {
    pub velocity: [f32; 2], // probably, type; length
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}
