use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

use crate::component::{Lock, Position, Named, Holding};

#[derive(SystemDesc, Default)]
pub(crate) struct LockSystem;

impl<'s> System<'s> for LockSystem {
    type SystemData = (
        ReadStorage<'s, Lock>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Named>,
        Entities<'s>,
        ReadStorage<'s, Holding>,
    );

    fn run(&mut self, (exits, positions, nameds, entities, holdings): Self::SystemData) {
    }
}

