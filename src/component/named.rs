use amethyst::{
    ecs::{prelude::{Component, DenseVecStorage}},
};

///
/// Movable Component
///
#[derive(PartialEq, Debug)]
pub(crate) struct Named(Name);

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum Name {
    Horizontal,
    Vertical,
    Interact,
}

impl Component for Named {
    type Storage = DenseVecStorage<Self>;
}

impl Named {
    pub(crate) fn new(name: Name) -> Self {
        Named(name)
    }

    pub(crate) fn is(&self, other: Name) -> bool {
        self.0 == other
    }
}



