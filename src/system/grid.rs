use crate::{component::Position, lib::Int};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

pub const GRID_SIZE: Int = 9;

#[derive(SystemDesc)]
pub(crate) struct GridSystem;

impl GridSystem {
    pub(crate) fn new() -> Self {
        GridSystem
    }
}

impl<'s> System<'s> for GridSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Position>);

    fn run(&mut self, (mut transforms, positions): Self::SystemData) {
        for (transform, position) in (&mut transforms, &positions).join() {
            let (x_int, y_int) = position.as_tuple();
            let (x, y): (f32, f32) = ((x_int * GRID_SIZE).into(), (y_int * GRID_SIZE).into());
            transform.set_translation_x(x);
            transform.set_translation_y(y);
        }
    }
}
