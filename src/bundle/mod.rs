use amethyst::{
    core::bundle::SystemBundle, ecs::DispatcherBuilder, prelude::*, shrev::EventChannel, Error,
};

use crate::lib::TransformedInputEvent;
// TODO: MovementEvent (and TransformedInputEvent) should go in a `channels` module or something.
use crate::system::movement_solver::MovementEvent;
use crate::system::switches::SwitchEvent;
use crate::system::{
    GrabSystem, MoveSoundSystem, MovementSolverSystem, MovementSystem, SpriteSystem, SwitchSystem, DoorSystem,
};

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
        let mut input_channel = EventChannel::<TransformedInputEvent>::new();
        let mut movement_channel = EventChannel::<MovementEvent>::new();
        let mut switch_channel = EventChannel::<SwitchEvent>::new();

        let movement_solver_reader = input_channel.register_reader();
        let grab_reader = input_channel.register_reader();
        let sprite_transform_reader = input_channel.register_reader();
        let sprite_switch_reader = switch_channel.register_reader();
        let mover_reader = movement_channel.register_reader();
        let move_sound_reader = movement_channel.register_reader();
        let door_reader = switch_channel.register_reader();

        world.insert(input_channel);
        world.insert(movement_channel);
        world.insert(switch_channel);

        builder.add(
            MovementSolverSystem::new(movement_solver_reader),
            "movement_solver_system",
            &["input_transform_system"],
        );

        builder.add(
            MovementSystem::new(mover_reader),
            "mover_system",
            &["movement_solver_system"],
        );

        builder.add(
            MoveSoundSystem::new(move_sound_reader),
            "move_sound_system",
            &["movement_solver_system"],
        );

        builder.add(
            GrabSystem::new(grab_reader),
            "grab_system",
            &["mover_system"],
        );

        builder.add(
            SpriteSystem::new(sprite_transform_reader, sprite_switch_reader),
            "sprite_system",
            &["grab_system"],
        );

        builder.add(
            SwitchSystem::default(),
            "switch_system",
            &["movement_solver_system"],
        );

        builder.add(
            DoorSystem::new(door_reader),
            "door_system",
            &["switch_system"],
        );

        Ok(())
    }
}
