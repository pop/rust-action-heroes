use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Flags an entity as a lock.
///
#[derive(Debug, Default)]
pub(crate) struct Lock;

impl Component for Lock {
    type Storage = NullStorage<Self>;
}
