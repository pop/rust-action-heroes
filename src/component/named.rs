use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::fmt;

///
/// Movable Component
///
#[derive(PartialEq, Debug, Eq, Hash)]
pub(crate) struct Named(Name);

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
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

    pub(crate) fn get(&self) -> Name {
        self.0
    }
}

impl fmt::Display for Named {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(_f, "{:?}", self.0)
    }
}
