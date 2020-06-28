//!
//! # Doors are opened with keys
//!
//! The only thing here is the DoorSystem struct, so go read about that!
//!

use crate::{component::Door, system::switches::SwitchEvent};
use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData},
    shrev::{EventChannel, ReaderId},
};

///
/// The Door System listens to `SwitchEvent`. When an `AllSwitchesDown` event is triggered, the system deletes all doors.
///
/// Under the hood we do that by tagging all Doors with `component::Door` and iterating over all
/// doors when we recieve `SwitchEvent::AllSwitchesDown`.
///
/// This is a pretty straight forward "listen to event, do simple task" system, which I like.
/// These types of systems require very little maintenance and become mostly solved quantities.
///
#[derive(SystemDesc)]
pub(crate) struct DoorSystem {
    reader: ReaderId<SwitchEvent>,
}

impl DoorSystem {
    pub(crate) fn new(reader: ReaderId<SwitchEvent>) -> Self {
        DoorSystem { reader }
    }
}

impl<'s> System<'s> for DoorSystem {
    type SystemData = (
        Read<'s, EventChannel<SwitchEvent>>,
        ReadStorage<'s, Door>,
        Entities<'s>,
    );

    fn run(&mut self, (switch_events, doors, entities): Self::SystemData) {
        for event in switch_events.read(&mut self.reader) {
            match event {
                SwitchEvent::AllSwitchesDown => {
                    for (_door, entity) in (&doors, &entities).join() {
                        match entities.delete(entity) {
                            Ok(_success) => (),
                            Err(_error) => (),
                        }
                    }
                }
            }
        }
    }
}
