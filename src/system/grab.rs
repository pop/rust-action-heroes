//!
//! # How does the Grabaron work?
//!
//! The only thing here is the GrabSystem struct, so go read about that!
//!

use crate::{
    component::{Holding, Name, Named, Position},
    lib::TransformedInputEvent,
};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use std::collections::{HashSet, VecDeque};

///
/// # Grab System
///
/// The grab system was an interesting challenge.
///
/// The mechanic in One Action Heroes is that all entities touching Grabaron get toggled into a
/// "Grab" state.
/// Not only that, but all entities touching _those_ entities get toggled to the "Grab" state, and
/// so on.
///
/// I decided to implement it by having a `Holding` component which tags a component as holdable.
/// These are the player characters, crates, and keys mostly.
///
/// The crux of the solution then was to start with Grabaron and iteratively process all entities
/// it, and all entities touching those entities, etc.
/// It's not performant, but it gets the job done.
///
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
            // Only process "Interact" events
            if event != &TransformedInputEvent::Interact {
                continue;
            }

            let mut pos = Position::default();
            let mut desired_holding = false;

            // Find the "Interact" character.
            // One Action Heroes calls this the "Grabaron"
            for (name, position, holding) in (&nameds, &positions, &holdings).join() {
                if name.get() == Name::Interact {
                    pos = *position;
                    desired_holding = !holding.status();
                    break;
                }
            }

            let mut toggle_holding: HashSet<Position> = HashSet::new();

            let mut toggle_queue: VecDeque<Position> = VecDeque::new();
            toggle_queue.push_front(pos);

            // Collect a set of entities which should be in the "holding" state
            //
            // We do this by generating and consuming a queue of candidate entities and adding them
            // to the set.
            while let Some(pos) = toggle_queue.pop_back() {
                toggle_holding.insert(pos);
                for (position, _hold) in (&positions, &holdings).join() {
                    if !toggle_holding.contains(&position) && touching(&pos, position) {
                        toggle_queue.push_front(*position);
                    }
                }
            }

            // For everything in the above generated set, toggle the holding state.
            //
            // This should toggle _everything_ to either "holding" or "not holding".
            // We shouldn't get into any "some holding" states.
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

/// I got this code to work early on and completely forgot how it works.
///
/// I love it.
fn touching(Position { x: x1, y: y1 }: &Position, Position { x: x2, y: y2 }: &Position) -> bool {
    (x1 == x2 && (y1 - 1 == *y2 || y1 + 1 == *y2)) || (y1 == y2 && (x1 - 1 == *x2 || x1 + 1 == *x2))
}
