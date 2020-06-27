use crate::component::{Exit, Holding, Named, Position};
use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

#[derive(SystemDesc, Default)]
pub(crate) struct LevelSystem;

impl<'s> System<'s> for LevelSystem {
    type SystemData = (
        ReadStorage<'s, Exit>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Named>,
        Entities<'s>,
        ReadStorage<'s, Holding>,
    );

    fn run(&mut self, (exits, positions, nameds, entities, holdings): Self::SystemData) {
        let mut pos = &Position::default();

        // Get the position of the exit
        for (position, _exit) in (&positions, &exits).join() {
            pos = position;
            break;
        }

        // Delete every entity overlapping with that position
        for (position, entity, holding, _named) in
            (&positions, &*entities, &holdings, &nameds).join()
        {
            if position == pos {
                match entities.delete(entity) {
                    Ok(_success) => (),
                    Err(_error) => (),
                }
                // This entity is holding other entities, so clean those up too
                if holding.status() {
                    for (entity, _named, _holding) in (&entities, &nameds, &holdings).join() {
                        match entities.delete(entity) {
                            Ok(_success) => (),
                            Err(_error) => (),
                        }
                    }
                }
            }
        }
    }
}
