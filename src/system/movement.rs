use amethyst::{
    derive::SystemDesc,
    ecs::{ System, SystemData, Read, ReadStorage, WriteStorage, Join},
    shrev::{ReaderId, EventChannel},
};

use crate::lib::TransformedInputEvent;
use crate::component::{Movable, Named, Name};

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
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, (channel, mut movables, names): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            println!("Reading {:?}", event);
            for (movable, name) in (&mut movables, &names).join() {
                match event {
                    TransformedInputEvent::Up => {
                        if name.is(Name::Vertical) {
                            println!("Movable {:?} {:?}", name, movable);
                            movable.move_up()
                        }
                    },
                    TransformedInputEvent::Down => {
                        if name.is(Name::Vertical) {
                            println!("Movable {:?} {:?}", name, movable);
                            movable.move_down()
                        }
                    },
                    TransformedInputEvent::Left => {
                        if name.is(Name::Horizontal) {
                            println!("Movable {:?} {:?}", name, movable);
                            movable.move_right()
                        }
                    },
                    TransformedInputEvent::Right => {
                        if name.is(Name::Horizontal) {
                            println!("Movable {:?} {:?}", name, movable);
                            movable.move_left()
                        }
                    },
                    TransformedInputEvent::Interact => {
                        if name.is(Name::Interact) {
                            println!("Movable {:?} {:?}", name, movable);
                            movable.interact()
                        }
                    },
                }
            }
        }
    }
}


