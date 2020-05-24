use crate::entity::{make_camera, make_horizontal, make_interact, make_vertical, make_walls};
use crate::lib::load_sprite_sheet;
use amethyst::prelude::*;

///
/// ...
///
pub(crate) struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world, "texture/evg1_spritesheet.png", "texture/evg1_spritesheet.ron");

        make_camera(world);
        make_interact(world, sprite_sheet_handle.clone());
        make_vertical(world, sprite_sheet_handle.clone());
        make_horizontal(world, sprite_sheet_handle.clone());
        make_walls(world, sprite_sheet_handle.clone());
    }
}
