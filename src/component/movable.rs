use amethyst::ecs::{NullStorage, Component};

///
/// Movable Component
///
#[derive(Debug, Default)]
pub(crate) struct Movable;

impl Component for Movable {
    type Storage = NullStorage<Self>;
}
