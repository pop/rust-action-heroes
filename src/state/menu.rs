use amethyst::prelude::*;
use crate::state::GameLevelState;
use crate::assets::GameLevel;
use amethyst::assets::{RonFormat, Loader, ProgressCounter, AssetStorage, Handle};

///
/// ...
///
pub(crate) struct MenuState {
    /// Tracks loaded assets.
    progress_counter: ProgressCounter,
    levels: Vec<Handle<GameLevel>>,
    level_1_handle: Option<Handle<GameLevel>>,
    game_level_asset_storage: Box<AssetStorage<GameLevel>>,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            progress_counter: ProgressCounter::new(),
            levels: Vec::new(), 
            level_1_handle: None,
            game_level_asset_storage: Box::new(AssetStorage::new()),
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Starting menu state!");
        let loader = &data.world.read_resource::<Loader>();
        self.level_1_handle = Some(
            loader.load(
                "levels/01.ron",
                RonFormat,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<GameLevel>>(),
            )
        );
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>,) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            let level_1_handle = self.level_1_handle.take().expect("Bar!");
            Trans::Push(Box::new(GameLevelState::new(level_1_handle)))
        } else {
            Trans::None
        }
    }
}
