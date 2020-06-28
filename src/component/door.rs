use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Flags an entity as a Door.
///
#[derive(Debug, Default)]
pub(crate) struct Door;

impl Component for Door {
    type Storage = NullStorage<Self>;
}
