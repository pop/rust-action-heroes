//!
//! # Honestly `lib` is an anti-pattern, this shouldn't be here...
//!

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

///
/// For when you're too lazy to decide/remember if your whole numbers are i16 or u8, use `Int`.
///
pub(crate) type Int = i16;

///
/// For most gameplay, user interaction is filtered into one of these five inputs.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TransformedInputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
}

///
/// I think this was basically copy-pasted from the Amehtyst docs.
///
/// Except the Ametyst docs omit the use of ProgressCounter which I actually like using.
///
/// Don't forget to tip your ProgressCounters kids.
///
pub(crate) fn load_sprite_sheet(
    world: &mut World,
    sprite_img: &str,
    sprite_key: &str,
) -> (Handle<SpriteSheet>, ProgressCounter) {
    let loader = world.read_resource::<Loader>();

    let mut texture_progress = ProgressCounter::new();
    let texture_handle = loader.load(
        sprite_img,
        ImageFormat::default(),
        &mut texture_progress,
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    let mut progress = ProgressCounter::new();
    (
        loader.load(
            sprite_key,
            SpriteSheetFormat(texture_handle),
            &mut progress,
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        ),
        progress,
    )
}

///
/// Given a sprite sheet handle, and a sprite number, return that sprite.
///
/// Should maybe do some error handling for if that sprite doesn't exist, I've definitely tried to
/// use a non-existant sprite before...
///
pub(crate) fn get_sprite(handle: &Handle<SpriteSheet>, index: usize) -> SpriteRender {
    SpriteRender {
        sprite_sheet: handle.clone(),
        sprite_number: index,
    }
}
