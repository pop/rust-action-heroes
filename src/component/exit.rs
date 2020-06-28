use amethyst::ecs::{Component, NullStorage};

///
/// Flags an entity as the level exit.
///
#[derive(Debug, Default)]
pub(crate) struct Exit;

impl Component for Exit {
    type Storage = NullStorage<Self>;
}
