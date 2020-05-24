use amethyst::ecs::{prelude::DenseVecStorage, Component};

///
/// Pushable Component
///
#[derive(Debug, Component)]
pub(crate) struct Pushable;

impl Pushable {
    pub(crate) fn new() -> Self {
        Pushable
    }
}
