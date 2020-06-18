use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Switch Component
///
#[derive(Debug, Default)]
pub(crate) struct Switch;

impl Component for Switch {
    type Storage = NullStorage<Self>;
}

