use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use std::collections::HashMap;

use crate::component::{Movable, Name, Named};
use crate::lib::TransformedInputEvent;

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
            let mut others = HashMap::new();
            for (movable, name) in (&movables, &names).join() {
                others.insert(name, movable.get_pos());
            }

            for (movable, name) in (&mut movables, &names).join() {
                // Get the other movables and their names
                // Decide what directions we cannot go in
                match (event, name.get()) {
                    (TransformedInputEvent::Up, Name::Vertical) => {
                        let mut safe = true;
                        for (_k, v) in &others {
                            let next = &(movable.get_x(), movable.y_add(1));
                            if collision(next, v) {
                                safe = false;
                            }
                        }
                        if safe {
                            movable.move_up()
                        }
                    }
                    (TransformedInputEvent::Down, Name::Vertical) => {
                        let mut safe = true;
                        for (_k, v) in &others {
                            let next = &(movable.get_x(), movable.y_sub(1));
                            if collision(next, v) {
                                safe = false;
                            }
                        }
                        if safe {
                            movable.move_down()
                        }
                    }
                    (TransformedInputEvent::Left, Name::Horizontal) => {
                        let mut safe = true;
                        for (_k, v) in &others {
                            let next = &(movable.x_sub(1), movable.get_y());
                            if collision(next, v) {
                                safe = false;
                            }
                        }
                        if safe {
                            movable.move_left()
                        }
                    }
                    (TransformedInputEvent::Right, Name::Horizontal) => {
                        let mut safe = true;
                        for (_k, v) in &others {
                            let next = &(movable.x_add(1), movable.get_y());
                            if collision(next, v) {
                                safe = false;
                            }
                        }
                        if safe {
                            movable.move_right()
                        }
                    }
                    (TransformedInputEvent::Interact, Name::Interact) => movable.interact(),
                    (_, _) => (),
                }
            }
        }
    }
}

fn collision((x1, y1): &(u8, u8), (x2, y2): &(u8, u8)) -> bool {
    x1 == x2 && y1 == y2
}
