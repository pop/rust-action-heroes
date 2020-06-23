use amethyst::{
    assets::{AssetStorage, Handle},
    prelude::*,
    renderer::SpriteSheet,
};
use crate::audio::toggle_mute;
use crate::assets::GameLevel;
use crate::entity::*;
use crate::state::{LevelProgression, Levels};
use amethyst::ecs::Entity;
use amethyst::input::{InputEvent, VirtualKeyCode};
use amethyst::ui::UiCreator;

///
/// ...
///
pub(crate) struct GameLevelState {
    player_entities: Vec<Entity>,
    level_handle: Handle<GameLevel>,
    npc_entities: Vec<Entity>,
    ui_handle: Option<Entity>,
    sprite_sheet_handle: Handle<SpriteSheet>,
}

impl GameLevelState {
    pub fn new(level_handle: Handle<GameLevel>, sprite_sheet_handle: Handle<SpriteSheet>) -> Self {
        GameLevelState {
            player_entities: Vec::new(),
            level_handle: level_handle,
            npc_entities: Vec::new(),
            ui_handle: None,
            sprite_sheet_handle: sprite_sheet_handle,
        }
    }
}

impl SimpleState for GameLevelState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator| creator.create("leveltext.ron", ())));

        let level: GameLevel = {
            let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
            asset_storage
                .get(&self.level_handle)
                .expect("Could not load game level")
                .clone()
        };

        if let Some((x, y)) = level.characters.interact {
            self.player_entities.push(
                make_interact(world, &self.sprite_sheet_handle, (x,y))
            )
        }

        if let Some((x,y)) = level.characters.vertical {
            self.player_entities.push(
                make_vertical(world, &self.sprite_sheet_handle, (x,y))
            );
        }

        if let Some((x,y)) = level.characters.horizontal {
            self.player_entities.push(
                make_horizontal(world, &self.sprite_sheet_handle, (x,y))
            );
        }

        self.npc_entities
            .extend(make_floors(world, &self.sprite_sheet_handle, level.floors));

        self.npc_entities
            .extend(make_crates(world, &self.sprite_sheet_handle, level.crates));

        self.npc_entities
            .extend(make_locks(world, &self.sprite_sheet_handle, level.locks));

        self.npc_entities
            .extend(make_keys(world, &self.sprite_sheet_handle, level.keys));

        self.npc_entities
            .extend(make_switches(world, &self.sprite_sheet_handle, level.switches));

        self.npc_entities
            .extend(make_doors(world, &self.sprite_sheet_handle, level.doors));

        self.npc_entities
            .extend(make_walls(world, &self.sprite_sheet_handle, level.walls));

        self.npc_entities
            .push(make_exit(world, &self.sprite_sheet_handle, level.exit));

        self.npc_entities
            .push(make_camera(world, &self.level_handle));
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
                Trans::Switch(Box::new(GameLevelState::new(self.level_handle.clone(), self.sprite_sheet_handle.clone())))
            }
            StateEvent::Input(InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Escape,
                ..
            }) |
            StateEvent::Input(InputEvent::KeyPressed {
                key_code: VirtualKeyCode::Q,
                ..
            }) => {
                // Go back to the main menu
                Trans::Pop
            }
            StateEvent::Input(InputEvent::KeyPressed {
                key_code: VirtualKeyCode::M,
                ..
            }) => {
                // Mute audio
                let mut world = data.world;

                toggle_mute(world);

                Trans::None
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
                            Trans::Switch(Box::new(GameLevelState::new(next_level.clone(), self.sprite_sheet_handle.clone())))
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
