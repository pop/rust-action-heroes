use crate::component::{Movable, Name, Named, Immovable, Pushable};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteSheet, SpriteRender},
};
use crate::lib::get_sprite;

// TODO: Better name
fn make_playable_entity(world: &mut World, (x, y): (u8, u8), name: Name, sprite: SpriteRender) {
    let transform = Transform::default();
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Movable::new(x, y))
        .with(Named::new(name))
        .with(Pushable::new())
        .build();
}

// TODO: Better name
fn make_obsticle(world: &mut World, (x, y): (u8, u8), sprite: SpriteRender) {
    let transform = Transform::default();
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Movable::new(x, y))
        .with(Immovable::new())
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
    make_playable_entity(world, (5, 4), Name::Vertical, sprite);
}

pub(crate) fn make_vertical(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 2);
    make_playable_entity(world, (5, 5), Name::Horizontal, sprite);
}

pub(crate) fn make_interact(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 4);
    make_playable_entity(world, (5, 6), Name::Interact, sprite);
}

pub(crate) fn make_walls(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite = get_sprite(sprite_sheet_handle, 10);
    let max = 10;
    let min = 1;
    for x in min ..= max {
        make_obsticle(world, (min, x), sprite.clone());
        make_obsticle(world, (x, min), sprite.clone());
        make_obsticle(world, (max, x), sprite.clone());
        make_obsticle(world, (x, max), sprite.clone());
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
