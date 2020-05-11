use amethyst::{
    prelude::*,
};
use crate::component::Movable;

///
/// ...
///
pub(crate) struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world
            .create_entity()
            .with(
                Movable::new()
            ).build();
    }
}
