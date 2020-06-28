//!
//! # We make one sound in the whole game
//!

use amethyst::{
    assets::AssetStorage,
    assets::Loader,
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

///
/// The path to our sound file is hard coded because there is only one.
/// I'm sure if we had more sounds to deal with there would be some way of cataloging and loading
/// them.
///
const MOVE_SOUND: &str = "audio/bloop.ogg";

///
/// The Sounds struct holds all of the sound effects (again, only one) and if sound is muted.
///
/// If we had more sound effects, we would add more SourceHandles.
///
/// Off the top of my head I'm not sure if Amethyst has a way of handling volume, but we might add
/// that to this struct too.
///
pub(crate) struct Sounds {
    pub(crate) move_sfx: SourceHandle,
    pub(crate) muted: bool,
}

///
/// Loads a given audio file and returns a handle.
///
/// Pretty simple since our audio requirements don't demand much flexibility.
///
fn load_audio(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

///
/// Inserts our Sounds struct into the world so we can access it as a resource later.
///
pub(crate) fn initialize_audio(world: &mut World) {
    world.insert({
        let loader = world.read_resource::<Loader>();

        Sounds {
            move_sfx: load_audio(&loader, &world, MOVE_SOUND),
            muted: false,
        }
    });
}

///
/// Helper function to play the move sound.
/// We use this in the MoveSound System.
///
pub(crate) fn play_move_sound(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.move_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

///
/// Another helper function to toggle global mute.
///
pub(crate) fn toggle_mute(world: &mut World) {
    let mut sounds = world.write_resource::<Sounds>();

    sounds.muted = !sounds.muted;
}
