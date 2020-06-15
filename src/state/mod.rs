use crate::assets::GameLevel;
use amethyst::assets::{Handle, ProgressCounter};

#[derive(Default)]
pub(crate) struct Levels {
    pub(crate) progress: Vec<ProgressCounter>,
    pub(crate) levels: Vec<Handle<GameLevel>>,
}

#[derive(Default)]
pub(crate) struct LevelProgression {
    pub(crate) current: usize,
}

mod menu;
pub(crate) use menu::*;

mod level;
pub(crate) use level::*;

mod loading;
pub(crate) use loading::*;
