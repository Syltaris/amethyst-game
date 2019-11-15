use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;
mod systems;

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
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)? // handles tracking entity positions
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]); // adding a system alone, not bundle, params are dependencies to run before this

    let mut world = World::new();
    let mut game = Application::new(assets_dir, Pong, game_data)?; // connect (path_to_assets, State, GameDataBuilder )
                                                                   // binds OS event loop, state machines, timers, other core components together

    game.run(); // simply start loop, until State returns Trans::Quit, or all states popped off State
    Ok(())
}
