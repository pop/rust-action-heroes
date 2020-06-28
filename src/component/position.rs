use crate::lib::Int;
use amethyst::ecs::{prelude::DenseVecStorage, Component};
use std::ops::Add;

///
/// Position Component. Places an entity on the play grid.
///
/// The most complex component in terms of `impls`.
/// We have some helper methods and we implement `Add` for ergonomics in a few systems.
///
#[derive(Debug, Component, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Position {
    pub(crate) x: Int,
    pub(crate) y: Int,
}

impl Position {
    pub(crate) fn new(x: Int, y: Int) -> Self {
        Position { x: x, y: y }
    }

    pub(crate) fn set_pos(&mut self, Position { x, y }: Position) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn as_tuple(&self) -> (Int, Int) {
        (self.x, self.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(Int, Int)> for Position {
    fn from(some: (Int, Int)) -> Self {
        Self {
            x: some.0,
            y: some.1,
        }
    }
}
