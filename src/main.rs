mod component;
mod entity;
mod lib;
mod state;
mod system;

use crate::lib::TransformedInputEvent;
use crate::state::GameState;
use crate::system::{MovementSystem, ProcessInputSystem};
use amethyst::{
    core::{bundle::SystemBundle, transform::TransformBundle},
    ecs::DispatcherBuilder,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::EventChannel,
    utils::application_root_dir,
    Error,
};
use std::env;

///
/// ...
///
struct MovementBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MovementBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let mut channel = EventChannel::<TransformedInputEvent>::new();
        let reader = channel.register_reader();

        world.insert(channel);

        builder.add(
            MovementSystem::new(reader),
            "movement_system",
            &["input_transform_system"],
        );
        Ok(())
    }
}

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
                        .with_clear([0.8, 0.2, 0.2, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(ProcessInputSystem::new(), "input_transform_system", &[])
        .with_bundle(MovementBundle)?;

    let mut game = Application::new(assets_dir, GameState, game_data)?;

    game.run();

    Ok(())
}
