use amethyst::{
    core::{bundle::SystemBundle},
    ecs::DispatcherBuilder,
    prelude::*,
    shrev::EventChannel,
    Error,
};

use crate::lib::TransformedInputEvent;
use crate::system::{MovementSystem, GrabSystem, SpriteSystem};

///
/// ...
///
pub(crate) struct MovementBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MovementBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let mut channel = EventChannel::<TransformedInputEvent>::new();

        let movement_reader = channel.register_reader();
        let grab_reader = channel.register_reader();
        let sprite_reader = channel.register_reader();

        world.insert(channel);

        builder.add(
            MovementSystem::new(movement_reader),
            "movement_system",
            &["input_transform_system"],
        );

        builder.add(
            GrabSystem::new(grab_reader),
            "grab_system",
            &["movement_system"],
        );

        builder.add(
            SpriteSystem::new(sprite_reader),
            "sprite_system",
            &["grab_system"],
        );

        Ok(())
    }
}
