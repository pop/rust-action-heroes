use amethyst::{
    derive::SystemDesc,
    ecs::{ System, SystemData, Read, WriteStorage, Join},
    shrev::{ReaderId, EventChannel},
};

use crate::lib::TransformedInputEvent;
use crate::component::Movable;

///
/// ...
///
#[derive(SystemDesc)]
pub(crate) struct MovementSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl MovementSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        MovementSystem { reader: reader }
    }
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        WriteStorage<'s, Movable>,
    );

    fn run(&mut self, (channel, mut movables): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            println!("Reading {:?}", event);
            for movable in (&mut movables).join() {
                println!("Movable {:?}", movable);
                match event {
                    TransformedInputEvent::Up =>  movable.move_up(),
                    TransformedInputEvent::Down => movable.move_down(),
                    TransformedInputEvent::Left => movable.move_right(),
                    TransformedInputEvent::Right => movable.move_left(),
                    TransformedInputEvent::Interact => movable.interact(),
                }
            }
        }
    }
}


