use amethyst::ecs::{prelude::DenseVecStorage, Component};
use std::fmt;

///
/// Gives a playable character a name.
///
/// This is used to make some important decisions like "when moving left, which entity moves".
///
/// If I did it again I would make each playable character a separate component.
/// This was implemented pretty early on in my education on ECS so I wasn't sure what an
/// appropriate pattern was.
///
/// Not to say that this is bad -- just would probably be easier to work with if we didn't have
/// this weird Name entity that really contains three sub-entities.
/// For example, no two entities have the same name, so it might be better if each name was just a
/// flag component.
///
#[derive(PartialEq, Debug, Eq, Hash, Component)]
pub(crate) struct Named(Name);

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub(crate) enum Name {
    Horizontal,
    Vertical,
    Interact,
}

impl Named {
    pub(crate) fn new(name: Name) -> Self {
        Named(name)
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
