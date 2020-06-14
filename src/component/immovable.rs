use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Immovable Component
///
#[derive(Debug, Default)]
pub(crate) struct Immovable;

impl Component for Immovable {
    type Storage = NullStorage<Self>;
}
