use crate::component::{Movable, Name, Named, Holding};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteSheet, SpriteRender},
};
use crate::lib::get_sprite;

// TODO: Better name
fn make_named_entity(world: &mut World, (x, y): (u8, u8), name: Name, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Movable::new(x, y))
        .with(Named::new(name))
        .with(Holding::new())
        .build();
}

// TODO: Better name
fn make_non_obsticle(world: &mut World, (x, y): (u8, u8), sprite: SpriteRender) {
    let transform = Transform::default();
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Movable::new(x, y))
        .build();
}

pub(crate) fn make_horizontal(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 0);
    make_named_entity(world, (5, 4), Name::Vertical, sprite);
}

pub(crate) fn make_vertical(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 2);
    make_named_entity(world, (5, 5), Name::Horizontal, sprite);
}

pub(crate) fn make_interact(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 4);
    make_named_entity(world, (5, 6), Name::Interact, sprite);
}

pub(crate) fn make_walls(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 10);
    let max = 10;
    let min = 1;
    for x in min ..= max {
        make_named_entity(world, (min, x), Name::Wall, sprite.clone());
        make_named_entity(world, (x, min), Name::Wall, sprite.clone());
        make_named_entity(world, (max, x), Name::Wall, sprite.clone());
        make_named_entity(world, (x, max), Name::Wall, sprite.clone());
    }
}

pub(crate) fn make_floor(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 9);
    let max = 10;
    for x in 1 ..= max {
        for y in 1 ..= max {
            make_non_obsticle(world, (x, y), sprite.clone());
        }
    }
}

pub(crate) fn make_crates(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 6);
    make_named_entity(world, (2,3), Name::Crate, sprite.clone());
}

pub(crate) fn make_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d(100.0, 100.0))
        .with(
            {
                // I just want to call Transform::from(x, y, z)...
                let mut transform = Transform::default();
                transform.set_translation_xyz(50.0, 50.0, 5.0);
                transform
            }
        )
        .build();
}
