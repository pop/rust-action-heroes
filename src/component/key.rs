use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Key Component
///
#[derive(Debug, Default)]
pub(crate) struct Key;

impl Component for Key {
    type Storage = NullStorage<Self>;
}
