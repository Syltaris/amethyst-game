use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?; // root path
    let display_config_path = app_root.join("config").join("display.ron"); // connect /config/display.ron to path

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
        .with_bundle(TransformBundle::new())?; // handles tracking entity positions

    let assets_dir = app_root.join("assets"); // path to assets

    let mut world = World::new();
    let mut game = Application::new(assets_dir, Pong, game_data)?; // connect (path_to_assets, State, GameDataBuilder )
                                                                   // binds OS event loop, state machines, timers, other core components together

    game.run(); // simply start loop, until State returns Trans::Quit, or all states popped off State
    Ok(())
}
