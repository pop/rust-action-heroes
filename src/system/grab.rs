use std::collections::{HashSet, VecDeque};

use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::component::{Holding, Name, Named, Position};
use crate::lib::TransformedInputEvent;

#[derive(SystemDesc)]
pub(crate) struct GrabSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl GrabSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        GrabSystem { reader }
    }
}

impl<'s> System<'s> for GrabSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, Holding>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, (channel, positions, mut holdings, nameds): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            let mut pos = Position::default();
            let mut desired_holding = false;
            for (name, position, holding) in (&nameds, &positions, &holdings).join() {
                if name.get() == Name::Interact {
                    pos = *position;
                    if event == &TransformedInputEvent::Interact {
                        desired_holding = !holding.status();
                    } else {
                        desired_holding = holding.status();
                    }
                    break;
                }
            }

            let mut toggle_holding: HashSet<Position> = HashSet::new();

            let mut toggle_queue: VecDeque<Position> = VecDeque::new();
            toggle_queue.push_front(pos);

            while let Some(pos) = toggle_queue.pop_back() {
                toggle_holding.insert(pos);
                for (position, _hold) in (&positions, &holdings).join() {
                    if !toggle_holding.contains(&position) && touching(&pos, position) {
                        toggle_queue.push_front(*position);
                    }
                }
            }

            // Toggle all the things
            for (hold, position) in (&mut holdings, &positions).join() {
                if toggle_holding.contains(&position) {
                    match desired_holding {
                        true => hold.set_holding(),
                        false => hold.set_not_holding(),
                    }
                }
            }
        }
    }
}

fn touching(Position { x: x1, y: y1 }: &Position, Position { x: x2, y: y2 }: &Position) -> bool {
    (x1 == x2 && (y1 - 1 == *y2 || y1 + 1 == *y2)) || (y1 == y2 && (x1 - 1 == *x2 || x1 + 1 == *x2))
}
