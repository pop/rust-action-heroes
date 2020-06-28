use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Flags an entity as a door switch.
///
#[derive(Debug, Default)]
pub(crate) struct Switch;

impl Component for Switch {
    type Storage = NullStorage<Self>;
}
