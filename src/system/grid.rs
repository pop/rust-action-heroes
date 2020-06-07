use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::component::Movable;

pub const GRID_SIZE: u8 = 9;

#[derive(SystemDesc)]
pub(crate) struct GridSystem;

impl GridSystem {
    pub(crate) fn new() -> Self {
        GridSystem
    }
}

impl<'s> System<'s> for GridSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Movable>);

    fn run(&mut self, (mut transforms, movables): Self::SystemData) {
        for (transform, movable) in (&mut transforms, &movables).join() {
            let (x_u8, y_u8) = movable.get_pos();
            let (x, y): (f32, f32) = ((x_u8 * GRID_SIZE).into(), (y_u8 * GRID_SIZE).into());
            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}
