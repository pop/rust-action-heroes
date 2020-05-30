use crate::entity::*;
use crate::lib::load_sprite_sheet;
use amethyst::ecs::Entity;
use amethyst::prelude::*;

///
/// ...
///
pub(crate) struct GameState {
    player_entities: Vec<Entity>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            player_entities: Vec::new(),
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(
            world,
            "texture/evg1_spritesheet.png",
            "texture/evg1_spritesheet.ron",
        );

        make_exit(world, sprite_sheet_handle.clone());
        self.player_entities
            .push(make_interact(world, sprite_sheet_handle.clone()));
        self.player_entities
            .push(make_vertical(world, sprite_sheet_handle.clone()));
        self.player_entities
            .push(make_horizontal(world, sprite_sheet_handle.clone()));
        make_crates(world, sprite_sheet_handle.clone());
        make_walls(world, sprite_sheet_handle.clone());
        make_floor(world, sprite_sheet_handle.clone());

        make_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;

        let mut quit = true;

        for entity in &self.player_entities {
            if world.is_alive(*entity) {
                quit = false;
            }
        }

        match quit {
            false => Trans::None,
            true => Trans::Pop,
        }
    }
}
