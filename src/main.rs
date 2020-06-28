//! # Rust Action Heroes
//!
//! Rust Action Heroes is a remake of the delightful [One Action Heroes][oah] gamejam
//! written in Rust using the wonderful game engine [Amethyst][amethyst].
//!
//! Rust Action Heroes is my first attempt at writing a game, so it's pretty shotty!
//! If you have feedback, please [make an issue on the repo][issue].
//!
//! These are the Rust API docs, so if you're interested in _how_ this game is implemented this
//! will probably help.
//!
//! If you are new to Amethyst, check out the [Amethyst Book][amethyst-book] which covers the ECS
//! architecture, what a System, Component, State, and Entity are.
//!
//! ## About this website
//!
//! This is sort of like an extended meandering blog-post, so please explore!
//!
//! Technically, these are API docs for the Rust Action Heroes codebase, but I'm using it to record
//! what I've learned about Entity Component System architecture and Amethyst.
//!
//! Note that I am using pseudocode for most of my descriptions of ECS, so don't be surprised if
//! rustc won't compile them. :-P
//!
//! [oah]: https://tapehead-co.itch.io/one-action-heroes
//! [amethyst]: https://amethyst.rs/
//! [amethyst-book]: https://book.amethyst.rs/stable/
//! [issue]: https://github.com/pop/rust-action-heroes/issues/

mod assets;
mod audio;
mod bundle;
mod component;
mod entity;
mod lib;
mod state;
mod system;

use std::env;

use amethyst::{
    assets::Processor,
    audio::AudioBundle,
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

use crate::{
    assets::GameLevel,
    bundle::MovementBundle,
    state::LoadingState,
    system::{GridSystem, LevelSystem, LockSystem, ProcessInputSystem},
};

///
/// Main builds the game object and starts running the game.
/// The interesting bits are in the system, state, assets, and entity modules!
///
/// One gotcha is that to get the game to run on a Linux desktop with Wayland, you need to set the
/// environment variable `WINIT_UNIX_BACKEND=x11`.
/// Thankfully we can just set that variable in main before starting the game.
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
                .with_plugin(RenderUi::default()),
        )?
        .with(Processor::<GameLevel>::new(), "game_level_processor", &[])
        .with(
            ProcessInputSystem::new(),
            "input_transform_system",
            &["input_system"],
        )
        .with_bundle(MovementBundle)?
        .with(
            LevelSystem::default(),
            "level_system",
            &[
                "game_level_processor",
                "mover_system",
                "movement_solver_system",
                "sprite_system",
            ],
        )
        .with(
            LockSystem::default(),
            "lock_system",
            &[
                "game_level_processor",
                "mover_system",
                "movement_solver_system",
                "sprite_system",
            ],
        )
        .with_bundle(AudioBundle::default())?
        .with(GridSystem::default(), "grid_system", &["mover_system"]);

    let mut game = Application::build(assets_dir, LoadingState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
