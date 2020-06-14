use crate::assets::GameLevel;
use crate::state::{GameLevelState, LevelProgression, Levels};
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::winit::VirtualKeyCode;
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiFinder};
use std::path::{PathBuf, Path};

///
/// ...
///
pub(crate) struct MenuState {
    ui_handle: Option<Entity>,
    start_button: Option<Entity>,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            ui_handle: None,
            start_button: None,
        }
    }

    fn load_level(&self, loader: &Loader, storage: &AssetStorage<GameLevel>, path: PathBuf) -> Option<(Handle<GameLevel>, ProgressCounter)> {
        if let Some(path_str) = path.to_str() {
            println!("Loading level {:?}", path_str);
            let mut progress = ProgressCounter::new();
            Some(
                (loader.load(path_str, RonFormat, &mut progress, storage), progress)
            )
        } else {
            None
        }
    }

    fn load_levels(&self, loader: &Loader, storage: &AssetStorage<GameLevel>, dir_list: Vec<PathBuf>) -> (Vec<Handle<GameLevel>>, Vec<ProgressCounter>) {
        let mut levels = Vec::new();
        let mut progresses = Vec::new();
        for path in dir_list {
            if let Some((level, progress)) = self.load_level(loader, storage, path) {
                levels.insert(0, level);
                progresses.insert(0, progress);
            }
        }
        (levels, progresses)
    }

    fn find_levels(&self, dir_list: std::fs::ReadDir) -> Vec<PathBuf> {
        let mut dir_list_vec: Vec<PathBuf> = Vec::new();
        for e in dir_list {
            if let Ok(p) = e {
                if let Ok(l) = p.path().strip_prefix("assets/") {
                    dir_list_vec.push(l.to_path_buf());
                }
            }
        }
        dir_list_vec
    }

    fn start_current_level(&mut self, world: &World) -> SimpleTrans {
        let current_level = match world.try_fetch::<LevelProgression>() {
            Some(level_progress) => level_progress.current,
            None => 0,
        };
        let levels_resource = world.try_fetch::<Levels>().expect("Could not load level handles!");
        match levels_resource.progress.get(current_level) {
            Some(progress) => {
                if progress.is_complete() {
                    match levels_resource.levels.get(current_level) {
                        Some(level) => {
                            println!("Starting level: {:?}", level);
                            Trans::Push(Box::new(GameLevelState::new(level.clone())))
                        },
                        None => Trans::None,
                    }
                } else {
                    Trans::None
                }
            },
            None => Trans::None,
        }
    }
}


impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle = Some(
            world.exec(|mut creator: UiCreator| {
                creator.create("menu.ron", ())
            }
        ));

        let mut levels = Levels::default();
        let mut progression = LevelProgression::default();

        // TODO: I'm pretty sure there's an Amethyst idiomatic way to register "levels" as a source
        // and load from there...
        match Path::new("assets/levels").read_dir() {
            Ok(dir_list) => {
                let asset_loader = &world.read_resource::<Loader>();
                let level_storage = &world.read_resource::<AssetStorage<GameLevel>>();
                let level_files = self.find_levels(dir_list);
                let result = self.load_levels(asset_loader, level_storage, level_files);
                let total_num_levels = result.0.len();
                levels = Levels { levels: result.0, progress: result.1 };
                progression = LevelProgression { current: 0, total: total_num_levels };
            }
            Err(_) => (),
        }

        world.insert(levels);
        world.insert(progression);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.start_button.is_none() {
            self.start_button = world.exec(|ui_finder: UiFinder| {
                    ui_finder.find("start_button")
                }
            );
            println!("start_button: {:?}", self.start_button);
        }

        Trans::None
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        match self.ui_handle {
            Some(entity) => {
                match data.world.delete_entity(entity) {
                    Ok(_) => self.ui_handle = None,
                    Err(_) => (),
                }
            },
            None => ()
        };
        match self.start_button {
            Some(entity) => {
                match data.world.delete_entity(entity) {
                    Ok(_) => self.start_button = None,
                    Err(_) => (),
                }
            },
            None => ()
        };
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.ui_handle = Some(
            world.exec(|mut creator: UiCreator| {
                creator.create("menu.ron", ())
            }
        ));
    }

    fn handle_event(
        &mut self,
        data: StateData<GameData>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            },
            StateEvent::Ui(UiEvent{
                event_type: UiEventType::Click,
                target,
            }) => {
                println!("Ui Event: {:?}", target);
                if Some(target) == self.start_button {
                    self.start_current_level(data.world) // Trans::Push(...)
                } else {
                    Trans::None
                }
            },
            _ => Trans::None
        }
    }
}
