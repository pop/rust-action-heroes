use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

use crate::component::{Lock, Key, Position};

#[derive(SystemDesc, Default)]
pub(crate) struct LockSystem;

impl<'s> System<'s> for LockSystem {
    type SystemData = (
        ReadStorage<'s, Lock>,
        ReadStorage<'s, Key>,
        ReadStorage<'s, Position>,
        Entities<'s>,
    );

    fn run(&mut self, (locks, keys, positions, entities): Self::SystemData) {
        for (_key, key_position, key_entity) in (&keys, &positions, &*entities).join() {
            for (_lock, lock_position, lock_entity) in (&locks, &positions, &*entities).join() {
                if lock_position == key_position {
                    match entities.delete(lock_entity) {
                        _ => (),
                    }
                    match entities.delete(key_entity) {
                        _ => ()
                    }
                }
            }
        }
    }
}

