//!
//! # How to open locked doors
//!
//! The only thing here is the LockSystem struct, so go read about that!
//!

use crate::component::{Key, Lock, Position};
use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

///
/// The Lock System is pretty simple.
///
/// If any keys overlap with any locks we consider that "unlocking" the lock and we delete both the
/// key and the lock from the world.
///
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
                        _ => (),
                    }
                }
            }
        }
    }
}
