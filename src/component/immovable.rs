use amethyst::ecs::{prelude::NullStorage, Component};

///
/// Marks an entity as immovable.
///
#[derive(Debug, Default)]
pub(crate) struct Immovable;

impl Component for Immovable {
    type Storage = NullStorage<Self>;
}
