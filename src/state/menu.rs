use crate::assets::GameLevel;
use crate::state::GameLevelState;
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat};
use amethyst::prelude::*;
use std::path::Path;

///
/// ...
///
pub(crate) struct MenuState {
    /// Tracks loaded assets.
    progress: Vec<ProgressCounter>,
    levels: Vec<Handle<GameLevel>>,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            progress: Vec::new(),
            levels: Vec::new(),
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let loader = &world.read_resource::<Loader>();
        let levels_dir = Path::new("assets/levels");
        match levels_dir.read_dir() {
            Ok(dir_list) => {
                for path in dir_list {
                    if let Ok(path) = path {
                        let mut progress = ProgressCounter::new();
                        self.levels.push(loader.load(
                            format!("levels/{}", path.file_name().to_str().unwrap()),
                            RonFormat,
                            &mut progress,
                            &world.read_resource::<AssetStorage<GameLevel>>(),
                        ));
                        self.progress.push(progress);
                    }
                }
            }
            Err(_) => (),
        }
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData>,
    ) -> SimpleTrans {
        // TODO: Cleanup existing state when we pop back here
        match self.progress.last() {
            Some(progress) => {
                if progress.is_complete() {
                    self.progress.pop();
                    match self.levels.pop() {
                        Some(level) => Trans::Push(Box::new(GameLevelState::new(level))),
                        None => Trans::None,
                    }
                } else {
                    Trans::None
                }
            }
            None => Trans::Quit,
        }
    }
}
