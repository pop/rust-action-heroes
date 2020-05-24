use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};


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
        // This code is so complicated I throw up a little every time I see it...
        for event in channel.read(&mut self.reader) {
            let mut to_move_names: Vec<Name> = Vec::new();
            let mut to_move_next_pos: Vec<(u8, u8)> = Vec::new();
            match event {
                TransformedInputEvent::Up => {
                    // See if Vertical will collide with anything movable
                    // See if that will collide with anything movable, etc.
                    // Collect all things that can move into a Vector.
                    for (movable, name) in (&movables, &names).join() {
                        if name.get() == Name::Vertical {                   // different
                            to_move_names.push(name.get());
                            to_move_next_pos.push(movable.up_pos());        // different
                        }
                    }
                    let mut added_to_move = true;
                    while added_to_move {
                        added_to_move = false;
                        for (movable, name) in (&movables, &names).join() {
                            if let Some(next_pos) = to_move_next_pos.last() {
                                if will_collide(&next_pos, &movable.get_pos()) {
                                    if name.get() == Name::Wall {
                                        // Hitting a wall, do not move;
                                        to_move_names.clear();
                                        to_move_next_pos.clear();
                                    } else {
                                        to_move_names.push(name.get());
                                        to_move_next_pos.push(movable.up_pos());    // different
                                        added_to_move = true;
                                    }
                                }
                            }
                        }
                    }
                    for (movable, name) in (&mut movables, &names).join() {
                        if to_move_names.contains(&name.get()) {
                            movable.move_up()                                   // different
                        }
                    }

                    println!("UP {:?}", to_move_names);
                },
                TransformedInputEvent::Down => {
                    for (movable, name) in (&movables, &names).join() {
                        if name.get() == Name::Vertical {                   // different
                            to_move_names.push(name.get());
                            to_move_next_pos.push(movable.down_pos());        // different
                        }
                    }
                    let mut added_to_move = true;
                    while added_to_move {
                        added_to_move = false;
                        for (movable, name) in (&movables, &names).join() {
                            if let Some(next_pos) = to_move_next_pos.last() {
                                if will_collide(&next_pos, &movable.get_pos()) {
                                    if name.get() == Name::Wall {
                                        // Hitting a wall, do not move;
                                        to_move_names.clear();
                                        to_move_next_pos.clear();
                                    } else {
                                        to_move_names.push(name.get());
                                        to_move_next_pos.push(movable.down_pos());    // different
                                        added_to_move = true;
                                    }
                                }
                            }
                        }
                    }
                    for (movable, name) in (&mut movables, &names).join() {
                        if to_move_names.contains(&name.get()) {
                            movable.move_down()                                   // different
                        }
                    }

                    println!("DOWN {:?}", to_move_names);

                },
                TransformedInputEvent::Left => {

                    for (movable, name) in (&movables, &names).join() {
                        if name.get() == Name::Horizontal {                   // different
                            to_move_names.push(name.get());
                            to_move_next_pos.push(movable.left_pos());        // different
                        }
                    }
                    let mut added_to_move = true;
                    while added_to_move {
                        added_to_move = false;
                        for (movable, name) in (&movables, &names).join() {
                            if let Some(next_pos) = to_move_next_pos.last() {
                                if will_collide(&next_pos, &movable.get_pos()) {
                                    if name.get() == Name::Wall {
                                        // Hitting a wall, do not move;
                                        to_move_names.clear();
                                        to_move_next_pos.clear();
                                    } else {
                                        to_move_names.push(name.get());
                                        to_move_next_pos.push(movable.left_pos());    // different
                                        added_to_move = true;
                                    }
                                }
                            }
                        }
                    }
                    for (movable, name) in (&mut movables, &names).join() {
                        if to_move_names.contains(&name.get()) {
                            movable.move_left()                                   // different
                        }
                    }

                    println!("LEFT {:?}", to_move_names);
                },
                TransformedInputEvent::Right => {
                    for (movable, name) in (&movables, &names).join() {
                        if name.get() == Name::Horizontal {                   // different
                            to_move_names.push(name.get());
                            to_move_next_pos.push(movable.right_pos());        // different
                        }
                    }
                    let mut added_to_move = true;
                    while added_to_move {
                        added_to_move = false;
                        for (movable, name) in (&movables, &names).join() {
                            if let Some(next_pos) = to_move_next_pos.last() {
                                if will_collide(&next_pos, &movable.get_pos()) {
                                    if name.get() == Name::Wall {
                                        // Hitting a wall, do not move;
                                        to_move_names.clear();
                                        to_move_next_pos.clear();
                                    } else {
                                        to_move_names.push(name.get());
                                        to_move_next_pos.push(movable.right_pos());    // different
                                        added_to_move = true;
                                    }
                                }
                            }
                        }
                    }
                    for (movable, name) in (&mut movables, &names).join() {
                        if to_move_names.contains(&name.get()) {
                            movable.move_right()                                   // different
                        }
                    }

                    println!("RIGHT {:?}", to_move_names);
                },
                TransformedInputEvent::Interact => ()
            }

        }
    }
}

fn will_collide((x1, y1): &(u8, u8), (x2, y2): &(u8, u8)) -> bool {
    x1 == x2 && y1 == y2
}
