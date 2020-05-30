use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Entities},
};

use crate::component::{Movable, Exit, Named};

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
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Named>,
        Entities<'s>
    );

    fn run(&mut self, (exits, movables, nameds, entities): Self::SystemData) {
        let mut pos = (0,0);
        for (movable, _exit) in (&movables, &exits).join() {
            pos = movable.get_pos()
        }
        for (movable, _named, entity) in (&movables, &nameds, &*entities).join() {
            if movable.get_pos() == pos {
                match entities.delete(entity) {
                    Ok(_success) => (),
                    Err(_error) => (),
                }
            }
        }
    }
}
