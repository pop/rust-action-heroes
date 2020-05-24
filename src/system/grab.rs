use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::component::{Holding, Movable};
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
    );

    fn run(&mut self, (channel, movables, mut holdings): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            println!("{:?}", event);
            match event {
                TransformedInputEvent::Interact => {
                    for (m1, h1) in (&movables, &mut holdings).join() {
                        let mut modified = false;
                        for m2 in (&movables).join() {
                            if touching(m1.get_pos(), m2.get_pos()) {
                                h1.is_holding();
                                modified = true;
                                println!("Holding");
                            }
                        }
                        if !modified {
                            h1.is_not_holding();
                            println!("Not holding");
                        }
                    }
                },
                _ => ()
            };
        }
    }
}

fn touching((x1, y1): (u8, u8), (x2, y2): (u8, u8)) -> bool {
    // Thanks Mozilla!
    // https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection#Axis-Aligned_Bounding_Box
    x1 < x2 + 1 && x1 + 1 > x2 && y1 < y2 + 1 && y1 + 1 >y2
}
