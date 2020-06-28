//!
//! # "Bloop"
//!
//! The only thing here is the DoorSystem struct, so go read about that!
//!

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
/// MoveSound System plays a sound when anything moves.
///
/// It's got a bug where if multiple entities move, it plays the move sound multiple times over itself.
/// That's probably trivial to fix; just skip playing the sound after the first time we play it.
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
