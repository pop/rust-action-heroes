use std::collections::{VecDeque, HashSet};

use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};


use crate::component::{Movable, Name, Named, Holding};
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
        ReadStorage<'s, Holding>,
    );

    fn run(&mut self, (channel, mut movables, names, holdings): Self::SystemData) {
        // This code is so complicated I throw up a little every time I see it...
        for event in channel.read(&mut self.reader) {
            let mut to_move_queue: VecDeque<(Name, (u8, u8))> = VecDeque::new();

            let mut to_move_names: HashSet<Name> = HashSet::new();
            let mut to_move_next_pos: HashSet<(u8, u8)> = HashSet::new();
            match event {
                TransformedInputEvent::Up => {
                    // See if Vertical will collide with anything movable
                    // See if that will collide with anything movable, etc.
                    // Collect all things that can move into a Vector.
                    let mut is_holding = false;
                    for (name, movable, holding) in (&names, &movables, &holdings).join() {
                        // println!("{:?} {:?} {:?}", name.get(), movable.get_pos(), holding.status());
                        if name.get() == Name::Vertical {          // different
                            to_move_queue.push_front((name.get(), movable.up_pos()));  // different
                            is_holding = holding.status();
                            break
                        }
                    }
                    if is_holding {
                        for (name, movable, holding) in (&names, &movables, &holdings).join() {
                            if holding.status() {
                                to_move_queue.push_front((name.get(), movable.up_pos())); // different
                            }
                        }
                    }
                    // TODO: This is a new pattern with a queue and filling a vec, use that for the
                    // other directions
                    while let Some((name, pos)) = to_move_queue.pop_back() {
                        to_move_names.insert(name);
                        to_move_next_pos.insert(pos);
                        if name == Name::Wall {
                            // Hitting a wall, do not move;
                            to_move_names.clear();
                            to_move_next_pos.clear();
                        } else {
                            for (movable, new_name) in (&movables, &names).join() {
                                if name != new_name.get() && will_collide(&pos, &movable.get_pos()) {
                                    to_move_queue.push_front((new_name.get(), movable.up_pos()));  // different
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
                    // See if Vertical will collide with anything movable
                    // See if that will collide with anything movable, etc.
                    // Collect all things that can move into a Vector.
                    let mut is_holding = false;
                    for (name, movable, holding) in (&names, &movables, &holdings).join() {
                        // println!("{:?} {:?} {:?}", name.get(), movable.get_pos(), holding.status());
                        if name.get() == Name::Vertical {          // different
                            to_move_queue.push_front((name.get(), movable.down_pos()));  // different
                            is_holding = holding.status();
                            break
                        }
                    }
                    if is_holding {
                        for (name, movable, holding) in (&names, &movables, &holdings).join() {
                            if holding.status() {
                                to_move_queue.push_front((name.get(), movable.down_pos())); // different
                            }
                        }
                    }
                    // TODO: This is a new pattern with a queue and filling a vec, use that for the
                    // other directions
                    while let Some((name, pos)) = to_move_queue.pop_back() {
                        to_move_names.insert(name);
                        to_move_next_pos.insert(pos);
                        if name == Name::Wall {
                            // Hitting a wall, do not move;
                            to_move_names.clear();
                            to_move_next_pos.clear();
                        } else {
                            for (movable, new_name) in (&movables, &names).join() {
                                if name != new_name.get() && will_collide(&pos, &movable.get_pos()) {
                                    to_move_queue.push_front((new_name.get(), movable.down_pos()));  // different
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
                    // See if Horizontal will collide with anything movable
                    // See if that will collide with anything movable, etc.
                    // Collect all things that can move into a Vector.
                    let mut is_holding = false;
                    for (name, movable, holding) in (&names, &movables, &holdings).join() {
                        // println!("{:?} {:?} {:?}", name.get(), movable.get_pos(), holding.status());
                        if name.get() == Name::Horizontal {          // different
                            to_move_queue.push_front((name.get(), movable.left_pos()));  // different
                            is_holding = holding.status();
                            break
                        }
                    }
                    if is_holding {
                        for (name, movable, holding) in (&names, &movables, &holdings).join() {
                            if holding.status() {
                                to_move_queue.push_front((name.get(), movable.left_pos())); // different
                            }
                        }
                    }
                    // TODO: This is a new pattern with a queue and filling a vec, use that for the
                    // other directions
                    while let Some((name, pos)) = to_move_queue.pop_back() {
                        to_move_names.insert(name);
                        to_move_next_pos.insert(pos);
                        if name == Name::Wall {
                            // Hitting a wall, do not move;
                            to_move_names.clear();
                            to_move_next_pos.clear();
                        } else {
                            for (movable, new_name) in (&movables, &names).join() {
                                if name != new_name.get() && will_collide(&pos, &movable.get_pos()) {
                                    to_move_queue.push_front((new_name.get(), movable.left_pos()));  // different
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
                    // See if Horizontal will collide with anything movable
                    // See if that will collide with anything movable, etc.
                    // Collect all things that can move into a Vector.
                    let mut is_holding = false;
                    for (name, movable, holding) in (&names, &movables, &holdings).join() {
                        // println!("{:?} {:?} {:?}", name.get(), movable.get_pos(), holding.status());
                        if name.get() == Name::Horizontal {          // different
                            to_move_queue.push_front((name.get(), movable.right_pos()));  // different
                            is_holding = holding.status();
                            break
                        }
                    }
                    if is_holding {
                        for (name, movable, holding) in (&names, &movables, &holdings).join() {
                            if holding.status() {
                                to_move_queue.push_front((name.get(), movable.right_pos())); // different
                            }
                        }
                    }
                    // TODO: This is a new pattern with a queue and filling a vec, use that for the
                    // other directions
                    while let Some((name, pos)) = to_move_queue.pop_back() {
                        to_move_names.insert(name);
                        to_move_next_pos.insert(pos);
                        if name == Name::Wall {
                            // Hitting a wall, do not move;
                            to_move_names.clear();
                            to_move_next_pos.clear();
                        } else {
                            for (movable, new_name) in (&movables, &names).join() {
                                if name != new_name.get() && will_collide(&pos, &movable.get_pos()) {
                                    to_move_queue.push_front((new_name.get(), movable.right_pos()));  // different
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
