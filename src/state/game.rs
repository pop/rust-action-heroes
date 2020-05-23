use crate::entity::{make_camera, make_horizontal, make_interact, make_vertical};
use amethyst::prelude::*;

///
/// ...
///
pub(crate) struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        make_camera(world);
        make_horizontal(world);
        make_vertical(world);
        make_interact(world);
    }
}
