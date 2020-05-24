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

    pub(crate) fn is_holding(&mut self) {
        self.0 = true;
    }

    pub(crate) fn is_not_holding(&mut self) {
        self.0 = false;
    }
}
