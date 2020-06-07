use crate::assets::GameLevel;
use crate::entity::*;
use crate::lib::load_sprite_sheet;
use amethyst::assets::Handle;
use amethyst::ecs::Entity;
use amethyst::prelude::*;

///
/// ...
///
pub(crate) struct GameLevelState {
    player_entities: Vec<Entity>,
    level_handle: Handle<GameLevel>,
    npc_entities: Vec<Entity>,
}

impl GameLevelState {
    pub fn new(level_handle: Handle<GameLevel>) -> Self {
        GameLevelState {
            player_entities: Vec::new(),
            level_handle: level_handle,
            npc_entities: Vec::new(),
        }
    }
}

impl SimpleState for GameLevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(
            world,
            "texture/evg1_spritesheet.png",
            "texture/evg1_spritesheet.ron",
        );

        self.npc_entities.push(
            make_camera(world)
        );
        self.npc_entities.push(
            make_exit(world, &self.level_handle, &sprite_sheet_handle)
        );
        match make_interact(world, &self.level_handle, &sprite_sheet_handle) {
            Some(e) => {
                println!("Creating interact");
                self.player_entities.push(e);
            },
            None => (),
        }
        match make_vertical(world, &self.level_handle, &sprite_sheet_handle) {
            Some(e) => {
                println!("Creating vertical");
                self.player_entities.push(e);
            },
            None => (),
        }
        match make_horizontal(world, &self.level_handle, &sprite_sheet_handle) {
            Some(e) => {
                println!("Creating horizontal");
                self.player_entities.push(e);
            },
            None => (),
        }
        self.npc_entities.extend(
            make_crates(world, &self.level_handle, &sprite_sheet_handle)
        );
        self.npc_entities.extend(
            make_walls(world, &self.level_handle, &sprite_sheet_handle)
        );
    }

    /// Cleanup entities
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        match data.world.delete_entities(&self.npc_entities) {
            _ => ()
        };
        match data.world.delete_entities(&self.player_entities) {
            _ => ()
        }
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
