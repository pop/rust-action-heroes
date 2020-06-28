//!
//! # There's a system for managing sprites!
//!
//! The only thing here is the SpriteSystem struct, so go read about that!
//!

use crate::{
    component::{Holding, Switch},
    lib::TransformedInputEvent,
    system::switches::SwitchEvent,
};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
};

///
/// SpriteSystem figures out what a given entity's sprite should be
///
/// Pretty much just toggles the "holding" sprites.
///
#[derive(SystemDesc)]
pub(crate) struct SpriteSystem {
    transform_reader: ReaderId<TransformedInputEvent>,
    switch_reader: ReaderId<SwitchEvent>,
}

impl SpriteSystem {
    pub(crate) fn new(
        transform_reader: ReaderId<TransformedInputEvent>,
        switch_reader: ReaderId<SwitchEvent>,
    ) -> Self {
        SpriteSystem {
            transform_reader,
            switch_reader,
        }
    }
}

impl<'s> System<'s> for SpriteSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        Read<'s, EventChannel<SwitchEvent>>,
        ReadStorage<'s, Holding>,
        ReadStorage<'s, Switch>,
        WriteStorage<'s, SpriteRender>,
    );

    // TODO: Put sprites in a more formal data structure
    fn run(
        &mut self,
        (input_events, switch_events, holdings, switches, mut sprites): Self::SystemData,
    ) {
        for _ in input_events.read(&mut self.transform_reader) {
            for (holding, sprite) in (&holdings, &mut sprites).join() {
                // TODO: The "grabbing" and "not_grabbing" sprite number should be a
                // component, not this dumb even or odd sprite number shit.
                match sprite.sprite_number % 2 {
                    0 => {
                        if holding.status() {
                            sprite.sprite_number = sprite.sprite_number + 1;
                        }
                    }
                    1 => {
                        if !holding.status() {
                            sprite.sprite_number = sprite.sprite_number - 1;
                        }
                    }
                    _ => (),
                }
            }
        }
        for _ in switch_events.read(&mut self.switch_reader) {
            for (_switch, sprite) in (&switches, &mut sprites).join() {
                // This should always be a one-way transformation
                match sprite.sprite_number % 2 {
                    0 => {
                        sprite.sprite_number = sprite.sprite_number + 1;
                    }
                    _ => (),
                }
            }
        }
    }
}
