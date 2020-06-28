use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Flags an entity as a key.
///
#[derive(Debug, Default)]
pub(crate) struct Key;

impl Component for Key {
    type Storage = NullStorage<Self>;
}
