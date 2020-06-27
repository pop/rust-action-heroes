use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Door Component
///
#[derive(Debug, Default)]
pub(crate) struct Door;

impl Component for Door {
    type Storage = NullStorage<Self>;
}
