mod assets;
mod bundle;
mod component;
mod entity;
mod lib;
mod state;
mod system;

use std::env;

use amethyst::{
    assets::Processor,
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

use crate::assets::GameLevel;
use crate::bundle::MovementBundle;
use crate::state::MenuState;
use crate::system::{GridSystem, LevelSystem, ProcessInputSystem};

///
/// ...
///
fn main() -> amethyst::Result<()> {
    env::set_var("WINIT_UNIX_BACKEND", "x11");

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 0.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(Processor::<GameLevel>::new(), "game_level_processor", &[])
        .with(
            ProcessInputSystem::new(),
            "input_transform_system",
            &["input_system"],
        )
        .with_bundle(MovementBundle)?
        .with(
            LevelSystem::new(),
            "level_system",
            &["game_level_processor", "movement_system"],
        )
        .with(GridSystem::new(), "grid_system", &["movement_system"]);

    let mut game = Application::new(assets_dir, MenuState::new(), game_data)?;

    game.run();

    Ok(())
}
