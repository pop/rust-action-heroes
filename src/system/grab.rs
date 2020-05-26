use std::collections::{VecDeque, HashSet};

use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::component::{Holding, Movable, Named, Name};
use crate::lib::TransformedInputEvent;


#[derive(SystemDesc)]
pub(crate) struct GrabSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl GrabSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        GrabSystem { reader: reader }
    }
}


impl<'s> System<'s> for GrabSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Holding>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, (channel, movables, mut holdings, nameds): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            // println!("grab system: {:?}", event);
            match event {
                TransformedInputEvent::Interact => {
                    let mut pos = (0, 0);
                    let mut desired_holding = false;
                    for (name, movable, holding) in (&nameds, &movables, &holdings).join() {
                        if name.get() == Name::Interact {
                            pos = movable.get_pos();
                            desired_holding = !holding.status();
                        }
                    }

                    let mut toggle_holding: HashSet<(Name, (u8, u8))> = HashSet::new();

                    let mut toggle_queue: VecDeque<(Name, (u8, u8))> = VecDeque::new();
                    toggle_queue.push_front((Name::Interact, pos));

                    while let Some((name, pos)) = toggle_queue.pop_back() {
                        toggle_holding.insert((name, pos));
                        for (movable, name) in (&movables, &nameds).join() {
                            if name.get() != Name::Wall {
                                println!("{:?} {:?}", name, movable.get_pos());
                                if !toggle_holding.contains(&(name.get(), movable.get_pos())) && touching(pos, movable.get_pos()) {
                                    println!("{:?} is touching Interact", name.get());
                                    toggle_queue.push_front((name.get(), movable.get_pos()));
                                }
                            }
                        }
                    }

                    // Toggle all the things
                    for (hold, movable, name) in (&mut holdings, &movables, &nameds).join() {
                        if toggle_holding.contains(&(name.get(), movable.get_pos())) {
                            match desired_holding {
                                true => hold.is_holding(),
                                false => hold.is_not_holding(),
                            }
                            println!("{:?} is now holding: {}", name.get(), desired_holding);
                        }
                    }
                },
                _ => ()
            };
        }
    }
}

fn touching((x1, y1): (u8, u8), (x2, y2): (u8, u8)) -> bool {
    (x1 == x2 && (y1 - 1 == y2 || y1 + 1 == y2)) || (y1 == y2 && (x1 - 1 == x2 || x1 + 1 == x2))
}
