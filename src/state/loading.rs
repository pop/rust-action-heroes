use crate::assets::GameLevel;
use crate::assets::LevelFormat;
use crate::audio::initialize_audio;
use crate::lib::load_sprite_sheet;
use crate::state::{LevelProgression, Levels, MenuState};
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter};
use amethyst::prelude::*;
use std::path::{Path, PathBuf};
use amethyst::renderer::SpriteSheet;

///
/// ...
#[derive(Default)]
pub(crate) struct LoadingState {
    sprite_sheet_progress: ProgressCounter,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl LoadingState {
    /// ...
    fn load_level(
        &self,
        loader: &Loader,
        storage: &AssetStorage<GameLevel>,
        path: PathBuf,
    ) -> Option<(Handle<GameLevel>, ProgressCounter)> {
        if let Some(path_str) = path.to_str() {
            let mut progress = ProgressCounter::new();
            Some((
                loader.load(path_str, LevelFormat, &mut progress, storage),
                progress,
            ))
        } else {
            None
        }
    }

    /// ...
    fn load_levels(
        &self,
        loader: &Loader,
        storage: &AssetStorage<GameLevel>,
        dir_list: Vec<PathBuf>,
    ) -> (Vec<Handle<GameLevel>>, Vec<ProgressCounter>) {
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

    /// ...
    fn find_levels(&self, dir_list: std::fs::ReadDir) -> Vec<PathBuf> {
        let mut dir_list_vec: Vec<PathBuf> = Vec::new();
        // So
        for e in dir_list {
            // Many
            if let Ok(p) = e {
                // Layers
                if let Ok(l) = p.path().strip_prefix("assets/") {
                    // Please
                    if let Some(extension) = l.extension() {
                        // Help
                        if extension.to_str() == Some("level") {
                            // Me
                            dir_list_vec.push(l.to_path_buf());
                        }
                    }
                }
            }
        }
        dir_list_vec.sort_unstable();
        dir_list_vec
    }

    fn load_sprite_sheet(&self, world: &mut World) -> (Handle<SpriteSheet>, ProgressCounter) {
        load_sprite_sheet(
            world,
            "texture/evg1_spritesheet.png",
            "texture/evg1_spritesheet.ron",
        )
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

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
                levels = Levels {
                    levels: result.0,
                    progress: result.1,
                };
                progression = LevelProgression { current: 0 };
            }
            Err(_) => (),
        }

        world.insert(levels);
        world.insert(progression);

        initialize_audio(world);

        let sprite_stuff = self.load_sprite_sheet(world);
        self.sprite_sheet_handle = Some(sprite_stuff.0);
        self.sprite_sheet_progress = sprite_stuff.1;
    }

    fn update(&mut self, _state_data: &mut StateData<GameData>) -> SimpleTrans {
        // Basically just wait for the sprite sheet to load
        if self.sprite_sheet_progress.is_complete() {
            println!("Loading sprite sheet is complete!");
            if let Some(sprite_sheet_handle) = &self.sprite_sheet_handle {
                Trans::Switch(Box::new(MenuState::new(sprite_sheet_handle.clone())))
            } else { Trans::None }
        } else { Trans::None }
    }
}
