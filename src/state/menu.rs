use crate::assets::GameLevel;
use crate::state::GameLevelState;
use crate::entity::make_camera;
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::winit::VirtualKeyCode;
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use amethyst::ui::{UiButtonBuilder, UiButton, UiButtonBuilderResources, get_default_font, FontAsset, UiCreator, UiEvent, UiEventType, UiFinder};
use std::path::{PathBuf, Path};

///
/// ...
///
pub(crate) struct MenuState {
    progress: Vec<ProgressCounter>,
    levels: Vec<Handle<GameLevel>>,
    ui_handle: Option<Entity>,
    start_button: Option<Entity>,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            progress: Vec::new(),
            levels: Vec::new(),
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
                levels.push(level);
                progresses.push(progress);
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
}


impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle = Some(
            world.exec(|mut creator: UiCreator| {
                creator.create("menu.ron", ())
            }
        ));
        println!("ui_handle: {:?}", self.ui_handle);

        // TODO: I'm pretty sure there's an Amethyst idiomatic way to register "levels" as a source
        // and load from there...
        match Path::new("assets/levels").read_dir() {
            Ok(dir_list) => {
                let asset_loader = &world.read_resource::<Loader>();
                let level_storage = &world.read_resource::<AssetStorage<GameLevel>>();
                let level_files = self.find_levels(dir_list);
                let result = self.load_levels(asset_loader, level_storage, level_files);
                println!("Loading level: {:?}", result);
                self.levels = result.0;
                self.progress = result.1;
            }
            Err(_) => (),
        }
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

    fn on_resume(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
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
                    match self.progress.last() {
                        Some(progress) => {
                            if progress.is_complete() {
                                self.progress.pop();
                                match self.levels.pop() {
                                    Some(level) => {
                                        match self.ui_handle {
                                            Some(entity) => {
                                                data.world.delete_entity(entity).expect("couldnt delete menu UI...");
                                            },
                                            None => ()
                                        };
                                        Trans::Push(Box::new(GameLevelState::new(level)))
                                    },
                                    None => Trans::None,
                                }
                            } else {
                                Trans::None
                            }
                        },
                        None => Trans::None,
                    }
                } else {
                    Trans::None
                }
            },
            _ => Trans::None
        }
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData>,
    ) -> SimpleTrans {
        // TODO: Cleanup existing state when we pop back here
        Trans::None
    }
}
