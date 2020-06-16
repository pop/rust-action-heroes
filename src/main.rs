mod assets;
mod audio;
mod bundle;
mod component;
mod entity;
mod lib;
mod state;
mod system;

use std::{
    env,
    time::Duration,
};

use amethyst::{
    audio::AudioBundle,
    assets::Processor,
    core::{
        transform::TransformBundle,
        frame_limiter::FrameRateLimitStrategy,
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{UiBundle, RenderUi},
    utils::application_root_dir,
};

use crate::assets::GameLevel;
use crate::bundle::MovementBundle;
use crate::state::LoadingState;
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
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderFlat2D::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                .with_plugin(
                    RenderUi::default()
                )
        )?
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
            &["game_level_processor", "mover_system", "movement_solver_system", "sprite_system"],
        )
        .with_bundle(AudioBundle::default())?
        .with(GridSystem::new(), "grid_system", &["mover_system"]);

    let mut game = Application::build(assets_dir, LoadingState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        ).build(game_data)?;

    game.run();

    Ok(())
}
