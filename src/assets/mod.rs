use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};
use serde::{Deserialize, Serialize};

pub(crate) type Coordinates = (i8, i8);

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

impl Asset for GameLevel {
    const NAME: &'static str = "evg1::GameLevel";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<GameLevel>>;
}
