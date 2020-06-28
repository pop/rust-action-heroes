//!
//! # Movement was so complicated it has two systems
//!
//! The only thing here is the MoverSystem struct, so go read about that!
//!

use crate::{component::Position, system::movement_solver::MovementEvent};
use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

///
/// MovementSystem works closely with MovementSolverSystem.
/// It reads MovementEvents and moves entities based on messages in that channel.
///
/// Honestly, it's not much to write home about.
/// The interesting bits are in MovementSolverSystem.
///
#[derive(SystemDesc)]
pub(crate) struct MovementSystem {
    reader: ReaderId<MovementEvent>,
}

impl MovementSystem {
    pub(crate) fn new(reader: ReaderId<MovementEvent>) -> Self {
        MovementSystem { reader: reader }
    }
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, EventChannel<MovementEvent>>,
        WriteStorage<'s, Position>,
    );

    fn run(&mut self, (movement_channel, mut positions): Self::SystemData) {
        for event in movement_channel.read(&mut self.reader) {
            match event {
                MovementEvent {
                    entity: e, to: t, ..
                } => match positions.get_mut(*e) {
                    Some(p) => {
                        p.set_pos(*t);
                    }
                    None => (),
                },
            }
        }
    }
}
