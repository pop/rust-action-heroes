//!
//! # This grid ain't free!
//!
//! The only thing here is the GridSystem struct, so go read about that!
//!
use crate::{component::Position, lib::Int};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

/// Our sprites are 9x9 pixels, so our grid shift is 9.
/// If we decide to have bigger sprites this would need to change!
pub const GRID_SIZE: Int = 9;

///
/// Grid System handles locking all entities to a grid.
///
/// Every entity has the `Position` component which gives it an X and a Y in space.
/// This system translates those X/Y positions into pixel locations on screen.
///
/// It would probably be notably more performant if we made it event based, since nothing really
/// happens on screen unless the user interacts, but it works!
///
#[derive(SystemDesc, Default)]
pub(crate) struct GridSystem;

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
