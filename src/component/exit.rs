use amethyst::ecs::{prelude::*, Component};

///
/// Exit Component
///
#[derive(Debug, Component)]
pub(crate) struct Exit;

impl Exit {
    pub fn new() -> Self {
        Exit
    }
}
