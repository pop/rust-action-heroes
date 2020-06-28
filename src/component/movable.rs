use amethyst::ecs::{Component, NullStorage};

///
/// Flags an object as movable.
///
/// Includes things like crates, keys, other players, etc.
///
/// Not sure how this differs from Pushable...
/// I assume I had a good reason to put it here.
///
#[derive(Debug, Default)]
pub(crate) struct Movable;

impl Component for Movable {
    type Storage = NullStorage<Self>;
}
