use amethyst::{
    ecs::{prelude::{Component, DenseVecStorage}},
};

///
/// Movable Component
///
#[derive(Debug)]
pub(crate) struct Movable {
    pos: (u8, u8),
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

impl Movable {
    pub(crate) fn get_pos(&self) -> (u8, u8) {
        self.pos
    }

    pub(crate) fn move_up(&mut self) {
        self.pos.0 = self.pos.0 + 1;
    }

    pub(crate) fn move_down(&mut self) {
        match self.pos.0.checked_sub(1) {
            Some(res) => self.pos.0 = res,
            None => (),
        }
    }

    pub(crate) fn move_right(&mut self) {
        self.pos.1 = self.pos.1 + 1;
    }

    pub(crate) fn move_left(&mut self) {
        match self.pos.1.checked_sub(1) {
            Some(res) => self.pos.1 = res,
            None => (),
        }
    }

    pub(crate) fn interact(&self) {
        println!("🎯 interact at ({:?})", self.get_pos());
    }

    pub(crate) fn new() -> Self {
        Movable { pos: (0,0) }
    }
}


