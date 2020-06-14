use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

use crate::component::{Exit, Position, Named};

#[derive(SystemDesc)]
pub(crate) struct LevelSystem;

impl LevelSystem {
    pub(crate) fn new() -> Self {
        LevelSystem
    }
}

impl<'s> System<'s> for LevelSystem {
    type SystemData = (
        ReadStorage<'s, Exit>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Named>,
        Entities<'s>,
    );

    fn run(&mut self, (exits, positions, nameds, entities): Self::SystemData) {
        let mut pos = &Position::default();

        // Get the position of the exit
        for (position, _exit) in (&positions, &exits).join() {
            pos = position;
            break;
        }

        // Delete every entity overlapping with that position
        for (position, _named, entity) in (&positions, &nameds, &*entities).join() {
            if position == pos {
                match entities.delete(entity) {
                    Ok(_success) => (),
                    Err(_error) => (),
                }
            }
        }
    }
}
