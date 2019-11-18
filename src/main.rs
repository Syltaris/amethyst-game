use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
mod pong;
mod systems;

use crate::audio::Music;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?; // root path
    let assets_dir = app_root.join("assets"); // path to assets
    let display_config_path = app_root.join("config").join("display.ron"); // connect /config/display.ron to path
    let binding_path = app_root.join("config").join("bindings.ron"); // connect input bindings

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    // with StringBindings, we need to indicate params as strings, e.g. "left_paddle"

    let game_data = GameDataBuilder::default() // all game runtime logic
        .with_bundle(
            // uses RenderPlugin trait, that uses rendy crate
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)? // handles tracking entity positions
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]) // adding a system alone, not bundle, params are dependencies to run before this
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);
    let mut world = World::new();
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?; // connect (path_to_assets, State, GameDataBuilder )
                                                                              // binds OS event loop, state machines, timers, other core components together

    game.run(); // simply start loop, until State returns Trans::Quit, or all states popped off State
    Ok(())
}
