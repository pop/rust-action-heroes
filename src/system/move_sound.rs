use crate::{
    audio::{play_move_sound, Sounds},
    system::movement_solver::MovementEvent,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    derive::SystemDesc,
    ecs::{Read, ReadExpect, System, SystemData},
    shrev::{EventChannel, ReaderId},
};
use std::ops::Deref;

///
/// ...
///
#[derive(SystemDesc)]
pub(crate) struct MoveSoundSystem {
    reader: ReaderId<MovementEvent>,
}

impl MoveSoundSystem {
    pub(crate) fn new(reader: ReaderId<MovementEvent>) -> Self {
        MoveSoundSystem { reader }
    }
}

impl<'s> System<'s> for MoveSoundSystem {
    type SystemData = (
        Read<'s, AssetStorage<Source>>,
        Read<'s, EventChannel<MovementEvent>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (storage, events, sounds, audio_output): Self::SystemData) {
        for _ in events.read(&mut self.reader) {
            if !sounds.muted {
                play_move_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}