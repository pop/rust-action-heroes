use amethyst::ecs::{Component, NullStorage};

///
/// Movable Component
///
#[derive(Debug, Default)]
pub(crate) struct Movable;

impl Component for Movable {
    type Storage = NullStorage<Self>;
}
