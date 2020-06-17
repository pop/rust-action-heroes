use crate::assets::GameLevel;
use crate::entity::*;
use crate::lib::load_sprite_sheet;
use crate::state::{LevelProgression, Levels};
use amethyst::assets::Handle;
use amethyst::ecs::Entity;
use amethyst::input::{InputEvent, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::ui::UiCreator;

///
/// ...
///
pub(crate) struct GameLevelState {
    player_entities: Vec<Entity>,
    level_handle: Handle<GameLevel>,
    npc_entities: Vec<Entity>,
    ui_handle: Option<Entity>,
}

impl GameLevelState {
    pub fn new(level_handle: Handle<GameLevel>) -> Self {
        GameLevelState {
            player_entities: Vec::new(),
            level_handle: level_handle,
            npc_entities: Vec::new(),
            ui_handle: None,
        }
    }
}

impl SimpleState for GameLevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator| creator.create("leveltext.ron", ())));

        let sprite_sheet_handle = load_sprite_sheet(
            world,
            "texture/evg1_spritesheet.png",
            "texture/evg1_spritesheet.ron",
        );
        if let Some(e) = make_interact(world, &self.level_handle, &sprite_sheet_handle) {
            self.player_entities.push(e);
        }
        if let Some(e) = make_vertical(world, &self.level_handle, &sprite_sheet_handle) {
            self.player_entities.push(e);
        }
        if let Some(e) = make_horizontal(world, &self.level_handle, &sprite_sheet_handle) {
            self.player_entities.push(e);
        }
        self.npc_entities
            .extend(make_locks(world, &self.level_handle, &sprite_sheet_handle));
        self.npc_entities
            .extend(make_keys(world, &self.level_handle, &sprite_sheet_handle));
        self.npc_entities
            .extend(make_crates(world, &self.level_handle, &sprite_sheet_handle));
        self.npc_entities
            .extend(make_walls(world, &self.level_handle, &sprite_sheet_handle));
        self.npc_entities
            .extend(make_floor(world, &self.level_handle, &sprite_sheet_handle));
        self.npc_entities
            .push(make_camera(world, &self.level_handle));
        self.npc_entities
            .push(make_exit(world, &self.level_handle, &sprite_sheet_handle));
    }

    /// Cleanup entities
    fn on_stop(&mut self, data: StateData<GameData>) {
        match data.world.delete_entities(
            &[
                &self.npc_entities[..],
                &self.player_entities[..],
                &[self.ui_handle.unwrap()],
            ]
            .concat(),
        ) {
            _ => (),
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(InputEvent::KeyPressed {
                key_code: VirtualKeyCode::R,
                ..
            }) => {
                // Reload the current level
                Trans::Switch(Box::new(GameLevelState::new(self.level_handle.clone())))
            }
            StateEvent::Input(InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Escape,
                ..
            }) => {
                // Go back to the main menu
                Trans::Pop
            }
            _ => {
                let world = &data.world;

                for entity in &self.player_entities {
                    if world.is_alive(*entity) {
                        return Trans::None;
                    }
                }

                if let Some(mut progression_resource) = world.try_fetch_mut::<LevelProgression>() {
                    progression_resource.current += 1;

                    if let Some(levels_resource) = world.try_fetch::<Levels>() {
                        if let Some(next_level) =
                            levels_resource.levels.get(progression_resource.current)
                        {
                            // Progress to the next level
                            Trans::Switch(Box::new(GameLevelState::new(next_level.clone())))
                        } else {
                            // Reset the level counter becuase we beat all the levels
                            progression_resource.current = 0;
                            Trans::Pop
                        }
                    } else {
                        Trans::Pop
                    }
                } else {
                    // Something went wrong...
                    Trans::Pop
                }
            }
        }
    }
}
