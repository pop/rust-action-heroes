use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub(crate) type Int = i16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TransformedInputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
}

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
        progress
    )
}

pub(crate) fn get_sprite(handle: &Handle<SpriteSheet>, index: usize) -> SpriteRender {
    SpriteRender {
        sprite_sheet: handle.clone(),
        sprite_number: index,
    }
}
