use crate::state::{GameLevelState, LevelProgression, Levels};
use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiFinder};
use amethyst::winit::VirtualKeyCode;

///
/// ...
///
#[derive(Default)]
pub(crate) struct MenuState {
    ui_handle: Option<Entity>,
    start_button: Option<Entity>,
}

impl MenuState {
    fn start_current_level(&mut self, world: &World) -> SimpleTrans {
        let current_level = match world.try_fetch::<LevelProgression>() {
            Some(level_progress) => level_progress.current,
            None => 0,
        };
        let levels_resource = world
            .try_fetch::<Levels>()
            .expect("Could not load level handles!");
        match levels_resource.progress.get(current_level) {
            Some(progress) => {
                if progress.is_complete() {
                    match levels_resource.levels.get(current_level) {
                        Some(level) => Trans::Push(Box::new(GameLevelState::new(level.clone()))),
                        None => Trans::None,
                    }
                } else {
                    Trans::None
                }
            }
            None => Trans::None,
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle = Some(world.exec(|mut creator: UiCreator| creator.create("menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.start_button.is_none() {
            self.start_button = world.exec(|ui_finder: UiFinder| ui_finder.find("start_button"));
        }

        Trans::None
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        match self.ui_handle {
            Some(entity) => match data.world.delete_entity(entity) {
                Ok(_) => self.ui_handle = None,
                Err(_) => (),
            },
            None => (),
        };
        match self.start_button {
            Some(entity) => match data.world.delete_entity(entity) {
                Ok(_) => self.start_button = None,
                Err(_) => (),
            },
            None => (),
        };
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.ui_handle = Some(world.exec(|mut creator: UiCreator| creator.create("menu.ron", ())));
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.start_button {
                    self.start_current_level(data.world) // Trans::Push(...)
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}
