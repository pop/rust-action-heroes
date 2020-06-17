use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Lock Component
///
#[derive(Debug, Default)]
pub(crate) struct Lock;

impl Component for Lock {
    type Storage = NullStorage<Self>;
}
