use amethyst::{
    derive::SystemDesc,
    ecs::{Read, WriteStorage, System, SystemData},
    shrev::{EventChannel, ReaderId},
};

use crate::component::Position;
use crate::system::movement_solver::MovementEvent;

///
/// ...
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
                MovementEvent { entity: e, to: t, .. } => {
                    match positions.get_mut(*e) {
                        Some(p) => {
                            p.set_pos(*t);
                        },
                        None => ()
                    }
                },
                _ => ()
            }
        }
    }
}