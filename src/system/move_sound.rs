use amethyst::{
    ecs::{Read, System, ReadExpect},
    assets::AssetStorage,
    audio::{output::Output, Source},
    shrev::{EventChannel, ReaderId},
};
use crate::audio::{play_move_sound, Sounds};
use crate::system::movement_solver::MovementEvent;
use std::ops::Deref;

pub(crate) struct MoveSoundSystem {
    reader: ReaderId<MovementEvent>,
}

impl MoveSoundSystem {
    pub(crate) fn new(reader: ReaderId<MovementEvent>) -> Self {
        MoveSoundSystem { reader: reader }
    }
}


impl<'s> System<'s> for MoveSoundSystem {
    type SystemData = (
        Read<'s, AssetStorage<Source>>,
        Read<'s, EventChannel<MovementEvent>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (storage, events, sounds, audio_output): Self::SystemData)  {
        for _ in events.read(&mut self.reader) {
            play_move_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
        }
    }
}
