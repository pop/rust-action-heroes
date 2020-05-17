use amethyst::{
    derive::SystemDesc,
    ecs::{ System, SystemData, Read, ReadStorage, WriteStorage, Join},
    shrev::{ReaderId, EventChannel},
};
use std::collections::HashMap;

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

            let mut others = HashMap::new();
            for (movable, name) in (&movables, &names).join() {
                others.insert(name, movable.get_pos());
            }
            // println!("{:?}", others);

            for (movable, name) in (&mut movables, &names).join() {
                // Get the other movables and their names
                // Decide what directions we cannot go in
                match (event, name.get()) {
                    (TransformedInputEvent::Up, Name::Vertical) => {
                        // println!("Moving up");
                        let mut safe = true;
                        for (_k, v) in &others {
                            // println!("Checking {:?} + 1 against {:?}", movable.get_pos().0, v.0);
                            if movable.get_pos().0 + 1 == v.0 {
                                safe = false;
                            }
                        }
                        // println!("{:?}", safe);
                        if safe {
                            movable.move_up()
                        }
                    },
                    (TransformedInputEvent::Down, Name::Vertical) => {
                        // println!("Moving down");
                        let mut safe = true;
                        for (_k, v) in &others {
                            // println!("Checking {:?} - 1 against {:?}", movable.get_pos().0, v.0);
                            if movable.get_pos().0 > 0 && movable.get_pos().0 - 1 == v.0 {
                                safe = false;
                            }
                        }
                        if safe {
                            movable.move_down()
                        }
                    },
                    (TransformedInputEvent::Left, Name::Horizontal) => {
                        // println!("moving left");
                        let mut safe = true;
                        for (_k, v) in &others {
                            // println!("Checking {:?} + 1 against {:?}", movable.get_pos().1, v.1);
                            if movable.get_pos().1 + 1 == v.1 {
                                safe = false;
                            }
                        }
                        // println!("{:?}", safe);
                        if safe {
                            movable.move_right()
                        }
                    },
                    (TransformedInputEvent::Right, Name::Horizontal) => {
                        // println!("moving right");
                        let mut safe = true;
                        for (_k, v) in &others {
                            // println!("Checking {:?} - 1 against {:?}", movable.get_pos().1, v.1);
                            if movable.get_pos().1 > 0 && movable.get_pos().1 - 1 == v.1 {
                                safe = false;
                            }
                        }
                        // println!("{:?}", safe);
                        if safe {
                            movable.move_left()
                        }
                    },
                    (TransformedInputEvent::Interact, Name::Interact) => {
                        movable.interact()
                    },
                    (_, _) => ()
                }

                println!("{}@{:?}", name, movable.get_pos());
            }
        }
    }
}


