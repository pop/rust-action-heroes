use amethyst::{
    assets::AssetStorage,
    assets::Loader,
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

const MOVE_SOUND: &str = "audio/bloop.ogg";

pub(crate) struct Sounds {
    pub(crate) move_sfx: SourceHandle,
    pub(crate) muted: bool,
}

fn load_audio(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub(crate) fn initialize_audio(world: &mut World) {
    world.insert({
        let loader = world.read_resource::<Loader>();

        Sounds {
            move_sfx: load_audio(&loader, &world, MOVE_SOUND),
            muted: false,
        }
    });
}

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

pub(crate) fn toggle_mute(world: &mut World) {
    let mut sounds = world.write_resource::<Sounds>();

    sounds.muted = !sounds.muted;
}
