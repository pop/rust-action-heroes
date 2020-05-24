use amethyst::ecs::{prelude::DenseVecStorage, Component};

///
/// Movable Component
///
#[derive(Debug, Component)]
pub(crate) struct Movable {
    x: u8,
    y: u8,
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

    pub(crate) fn set_pos(&mut self, (x, y): (u8, u8)) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn move_up(&mut self) {
        let new = self.up_pos();
        self.set_pos(new);
    }

    pub(crate) fn move_down(&mut self) {
        let new = self.down_pos();
        self.set_pos(new);
    }

    pub(crate) fn move_right(&mut self) {
        let new = self.right_pos();
        self.set_pos(new);
    }

    pub(crate) fn move_left(&mut self) {
        let new = self.left_pos();
        self.set_pos(new);
    }

    pub(crate) fn up_pos(&self) -> (u8, u8) {
        (self.get_x(), self.y_add(1))
    }

    pub(crate) fn down_pos(&self) -> (u8, u8) {
        (self.get_x(), self.y_sub(1))
    }

    pub(crate) fn left_pos(&self) -> (u8, u8) {
        (self.x_sub(1), self.get_y())
    }

    pub(crate) fn right_pos(&self) -> (u8, u8) {
        (self.x_add(1), self.get_y())
    }

    pub(crate) fn interact(&self) {}

    pub(crate) fn new(x: u8, y: u8) -> Self {
        Movable { x: x, y: y }
    }
}
