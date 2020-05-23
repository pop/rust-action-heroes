use amethyst::ecs::prelude::{Component, DenseVecStorage};

///
/// Movable Component
///
#[derive(Debug)]
pub(crate) struct Movable {
    x: u8,
    y: u8,
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

impl Movable {
    pub(crate) fn get_pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    pub(crate) fn get_x(&self) -> u8 {
        self.x
    }

    pub(crate) fn get_y(&self) -> u8 {
        self.y
    }

    pub(crate) fn x_sub(&self, n: u8) -> u8 {
        match self.x.checked_sub(n) {
            Some(res) => res,
            None => 0,
        }
    }

    pub(crate) fn y_sub(&self, n: u8) -> u8 {
        match self.y.checked_sub(n) {
            Some(res) => res,
            None => 0,
        }
    }

    pub(crate) fn x_add(&self, n: u8) -> u8 {
        match self.x.checked_add(n) {
            Some(res) => res,
            None => u8::MAX,
        }
    }

    pub(crate) fn y_add(&self, n: u8) -> u8 {
        match self.y.checked_add(n) {
            Some(res) => res,
            None => u8::MAX,
        }
    }


    pub(crate) fn move_up(&mut self) {
        self.x = self.x + 1;
    }

    pub(crate) fn move_down(&mut self) {
        self.x = self.x_sub(1);
    }

    pub(crate) fn move_right(&mut self) {
        self.y = self.y + 1;
    }

    pub(crate) fn move_left(&mut self) {
        self.y = self.y_sub(1);
    }

    pub(crate) fn interact(&self) {}

    pub(crate) fn new(x: u8, y: u8) -> Self {
        Movable { x: x, y: y }
    }
}
