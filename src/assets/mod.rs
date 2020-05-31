use amethyst::{
    assets::{Handle, Asset},
    ecs::VecStorage,
};
use serde::{Serialize, Deserialize};

pub(crate) type Coordinates = (u8, u8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CharacterPlacement {
    pub horizontal: Option<Coordinates>,
    pub vertical: Option<Coordinates>,
    pub interact: Option<Coordinates>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GameLevel {
    pub dimensions: Coordinates,
    pub characters: CharacterPlacement,
    pub exit: Coordinates,
    pub obstacles: Vec<Coordinates>,
}

pub(crate) type GameLevelHandle = Handle<GameLevel>;

impl Asset for GameLevel {
    const NAME: &'static str = "evg1::GameLevel";
    type Data = Self;
    type HandleStorage = VecStorage<GameLevelHandle>;
}
