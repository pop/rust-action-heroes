use amethyst::ecs::{prelude::DenseVecStorage, Component};

///
/// Immovable Component
///
#[derive(Debug, Component)]
pub(crate) struct Immovable;

impl Immovable {
    pub(crate) fn new() -> Self {
        Immovable
    }
}
