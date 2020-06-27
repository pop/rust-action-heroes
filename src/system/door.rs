use amethyst::{
    derive::SystemDesc,
    ecs::{Read, Join, System, Entities, SystemData, ReadStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::component::Door;
use crate::system::switches::SwitchEvent;

///
/// ...
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
                },
            }
        }
    }
}

