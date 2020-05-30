use amethyst::{
    renderer::{SpriteRender},
    ecs::{Join, Read, System, SystemData, ReadStorage, WriteStorage},
    shrev::{EventChannel, ReaderId},
    derive::SystemDesc,
};
use crate::{
    lib::TransformedInputEvent,
    component::Holding,
};

/// SpriteSystem figures out what a given entity's sprite should be
///
/// Pretty much just toggles the "holding" sprites.
#[derive(SystemDesc)]
pub(crate) struct SpriteSystem {
    reader: ReaderId<TransformedInputEvent>,
}

impl SpriteSystem {
    pub(crate) fn new(reader: ReaderId<TransformedInputEvent>) -> Self {
        SpriteSystem { reader: reader }
    }
}

impl<'s> System<'s> for SpriteSystem {
    type SystemData = (
        Read<'s, EventChannel<TransformedInputEvent>>,
        ReadStorage<'s, Holding>,
        WriteStorage<'s, SpriteRender>,
    );

    // TODO: Put sprites in a more formal data structure
    fn run(&mut self, (channel, holdings, mut sprites): Self::SystemData) {
        for _ in channel.read(&mut self.reader) {
            for (holding, sprite) in (&holdings, &mut sprites).join() {
                // TODO: The "grabbing" and "not_grabbing" sprite number should be a
                // component, not this dumb even or odd sprite number shit.
                match sprite.sprite_number % 2 {
                    0 => {
                        if holding.status() {
                            sprite.sprite_number = sprite.sprite_number + 1;
                        }
                    },
                    1 => {
                        if !holding.status() {
                            sprite.sprite_number = sprite.sprite_number - 1;
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}
