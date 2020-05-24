// use crate::component::{Movable, Name, Named, Holding};
use crate::component::{Movable, Name, Named};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteSheet, SpriteRender},
};
use crate::lib::get_sprite;

fn make_playable_entity(world: &mut World, (x, y): (u8, u8), name: Name, sprite: SpriteRender) {
    let transform = Transform::default();
    world
        .create_entity()
        .with(Movable::new(x, y))
        .with(Named::new(name))
        // .with(Holding::new())
        .with(sprite)
        .with(transform)
        .build();
}

pub(crate) fn make_horizontal(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 0);
    make_playable_entity(world, (5, 4), Name::Vertical, sprite);
}

pub(crate) fn make_vertical(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 1);
    make_playable_entity(world, (5, 5), Name::Horizontal, sprite);
}

pub(crate) fn make_interact(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 2);
    make_playable_entity(world, (5, 6), Name::Interact, sprite);
}

pub(crate) fn make_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d(100.0, 100.0))
        .with(
            {
                // I just want to call Transform::from(x, y, z)...
                let mut transform = Transform::default();
                transform.set_translation_xyz(50.0, 50.0, 1.0);
                transform
            }
        )
        .build();
}
