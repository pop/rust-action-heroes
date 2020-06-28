//!
//! # Loading, Menu, Level
//!
//! Amethyst States are like clear plastic you can draw on.
//! When you want a new drawing surface, you put a new state on top.
//!
//! When you put on new state on, you may want to cleanup your current state, or leave it as is.
//!
//! This game uses a lot of "push" state to add a state to the stack, "pop" state to go back to the
//! previous state, and "switch" state to replace the current state.
//!

use crate::assets::GameLevel;
use amethyst::assets::{Handle, ProgressCounter};

///
/// The "Levels" struct tracks which levels we have loaded.
///
/// `progress: Vec<ProgressCounter>` tracks if we have loaded a particular level.
///
#[derive(Default)]
pub(crate) struct Levels {
    pub(crate) progress: Vec<ProgressCounter>,
    pub(crate) levels: Vec<Handle<GameLevel>>,
}

///
/// LevelProgression tracks what the current level is.
/// Because of the way we have implemented level progression, this is vitally important.
///
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
