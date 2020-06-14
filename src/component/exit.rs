use amethyst::ecs::{NullStorage, Component};

///
/// Exit Component
///
#[derive(Debug, Default)]
pub(crate) struct Exit;

impl Component for Exit {
    type Storage = NullStorage<Self>;
}
