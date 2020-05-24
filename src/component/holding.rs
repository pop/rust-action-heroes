use amethyst::ecs::prelude::{Component, DenseVecStorage};

///
/// Holding Component
///
#[derive(Debug)]
pub(crate) struct Holding(bool);

impl Component for Holding {
    type Storage = DenseVecStorage<Self>;
}

impl Holding {
    pub(crate) fn is_holding(&mut self) {
        self.0 = true;
    }

    pub(crate) fn is_not_holding(&mut self) {
        self.0 = false;
    }
}
