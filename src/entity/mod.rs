use crate::component::{Movable, Name, Named};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub(crate) fn make_horizontal(world: &mut World) {
    world
        .create_entity()
        .with(Movable::new(0, 2))
        .with(Named::new(Name::Horizontal))
        .build();
}

pub(crate) fn make_vertical(world: &mut World) {
    world
        .create_entity()
        .with(Movable::new(0, 1))
        .with(Named::new(Name::Vertical))
        .build();
}

pub(crate) fn make_interact(world: &mut World) {
    world
        .create_entity()
        .with(Movable::new(0, 0))
        .with(Named::new(Name::Interact))
        .build();
}

pub(crate) fn make_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(50.0, 50.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(100.0, 100.0))
        .with(transform)
        .build();
}
