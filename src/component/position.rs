use std::ops::Add;

use amethyst::ecs::{prelude::DenseVecStorage, Component};


///
///
/// Position Component
///
#[derive(Debug, Component, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl Position {
    pub(crate) fn new(x: i8, y: i8) -> Self {
        Position { x: x, y: y }
    }

    pub(crate) fn set_pos(&mut self, Position { x, y }: Position ) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn as_tuple(&self) -> (i8, i8) {
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

impl From<(i8, i8)> for Position {
    fn from(some: (i8, i8)) -> Self {
        Self {
            x: some.0,
            y: some.1,
        }
    }
}
