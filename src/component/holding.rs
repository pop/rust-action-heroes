use amethyst::ecs::{prelude::*, Component};

///
/// Holding Component
///
#[derive(Debug, Component)]
pub(crate) struct Holding(bool);

impl Holding {
    pub(crate) fn new() -> Self {
        Holding(false)
    }

    pub(crate) fn set_holding(&mut self) {
        self.0 = true;
    }

    pub(crate) fn set_not_holding(&mut self) {
        self.0 = false;
    }

    pub(crate) fn status(&self) -> bool {
        self.0
    }
}
