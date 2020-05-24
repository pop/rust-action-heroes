use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::component::{Holding, Movable, Named};
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
    type SystemData = Read<'s, EventChannel<TransformedInputEvent>>;

    fn run(&mut self, channel: Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            // println!("grab system: {:?}", event);
            match event {
                TransformedInputEvent::Interact => {
                },
                _ => ()
            };
        }
    }
}
