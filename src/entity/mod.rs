//!
//! Builder functions for creating entities.
//!
//! These all follow a repeating pattern:
//! 1. Given the World, a Sprite Sheet Handle, and Coordinates for the entitiy/entities.
//! 2. Create an entity in the world with that information.
//! 3. Each method hard-codes which sprite number that entity has, and the components that entity
//!    has.
//!
//! For my next game I think I'll try to implement some file-based entity specification.
//! There's a lot of boilerplate that doesn't need to be here and the game wouldn't need to
//! re-compile every time I change an entity's component list.
//! Each entity could be specified in RON syntax sorta deal like:
//!
//! ```text
//! # assets/entities/001.ron
//! Components([
//!     Transform,
//!     Movable,
//!     Sprite(2),
//!     Position(x, y),
//!     Named(Name::Horizontal),
//!     Holding,
//! ])
//! ```
//!
//! Then the entity-loader would iterate over all files in `assets/entities/*.ron` and build:
//!
//! ```
//! let mut entitybuilder = world.create_entity();
//! for component in EntitySpec.Components {
//!     entitybuilder = entitybuilder.with(component);
//! }
//! entitybuilder.build();
//! ```
//!
//! And we can trivially do that in parallel (probably) so it wouldn't be too slow.
//!
//! The tedium required to write out entity files for hundreds, or even dozens, of entities
//! wouldn't be fun, but I also plan on writing a GUI game editor for the next game, which should
//! play nice with this.
//! I'd just need to figure out some way to serialize/deserialize an entity -- which I assume is
//! non-trivial.
//!

use crate::assets::GameLevel;
use crate::component::{
    Door, Exit, Holding, Immovable, Key, Lock, Movable, Name, Named, Position, Switch,
};
use crate::lib::{get_sprite, Int};
use crate::system::grid::GRID_SIZE;
use amethyst::{
    assets::{AssetStorage, Handle},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, SpriteSheet},
};

///
/// Creates "Prince horizontival the first"
///
pub(crate) fn make_horizontal(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    (x, y): (Int, Int),
) -> Entity {
    let sprite = get_sprite(sprite_sheet_handle, 2);

    world
        .create_entity()
        .with(Transform::default())
        .with(Movable::default())
        .with(sprite.clone())
        .with(Position::new(x, y))
        .with(Named::new(Name::Horizontal))
        .with(Holding::new())
        .build()
}

///
/// Creates "Duke vert the pure"
///
pub(crate) fn make_vertical(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    (x, y): (Int, Int),
) -> Entity {
    let sprite = get_sprite(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(Transform::default())
        .with(Movable::default())
        .with(sprite.clone())
        .with(Position::new(x, y))
        .with(Named::new(Name::Vertical))
        .with(Holding::new())
        .build()
}

///
/// Creates "Grabaron the wise"
///
pub(crate) fn make_interact(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    (x, y): (Int, Int),
) -> Entity {
    let sprite = get_sprite(sprite_sheet_handle, 4);

    world
        .create_entity()
        .with(Transform::default())
        .with(Movable::default())
        .with(sprite.clone())
        .with(Position::new(x, y))
        .with(Named::new(Name::Interact))
        .with(Holding::new())
        .build()
}

///
/// Creates all wall entities
///
/// Given a vector of coordinates it returns a vector of Entities which are the walls.
///
pub(crate) fn make_walls(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    wall_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 10);

    let mut entities: Vec<Entity> = Vec::new();

    for (x, y) in wall_coordinates {
        entities.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(Immovable::default())
                .with(Position::new(x, y))
                .with(sprite.clone())
                .build()
        );
    }
    entities
}

///
/// Makes the floors entities.
///
/// Floors are actually pretty boring.
/// They're just sprites that exist in space.
/// You can't interact with them, the just sit there.
///
/// So lazy.
///
/// One interesting thing is that they caused a hilarious bug where sometimes the floor would cover
/// up another entity because all entities were at the same Z coordinate, so pretty much at random
/// an entity we care about (like a player or a crate) would be covered up by floor sprites.
///
/// The solution to that bug, which I only determined after convincing myself it wasn't some weird
/// sprite loading bug, was to put sprites at a different Z level so they're much lower than
/// eveything else.
/// The game is orthographic so floor tiles don't look far way thankfully.
///
pub(crate) fn make_floors(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    floor_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 9);

    let mut entities: Vec<Entity> = Vec::new();

    for (x, y) in floor_coordinates {
        entities.push(
            world
                .create_entity()
                .with({
                    let mut t = Transform::default();
                    t.prepend_translation_z(-10.0);
                    t
                })
                .with(sprite.clone())
                .with(Position::new(x, y))
                .build(),
        );
    }
    entities
}

///
/// Makes all crates in the level.
///
pub(crate) fn make_crates(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    crate_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 6);

    let mut entities: Vec<Entity> = Vec::new();

    for (x, y) in crate_coordinates {
        entities.push(
            world
                .create_entity()
                .with(Holding::new())
                .with(Movable::default())
                .with(Position::new(x, y))
                .with(Transform::default())
                .with(sprite.clone())
                .build(),
        );
    }
    entities
}

///
/// Makes the level exit.
///
pub(crate) fn make_exit(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    (x, y): (Int, Int),
) -> Entity {
    let sprite = get_sprite(sprite_sheet_handle, 8);

    world
        .create_entity()
        .with(Transform::default())
        .with(sprite.clone())
        .with(Exit::default())
        .with(Position::new(x, y))
        .build()
}

///
/// Makes all the locks in the current level.
///
pub(crate) fn make_locks(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    lock_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 11);

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in lock_coordinates {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Lock::default())
                .with(Position::new(x, y))
                .with(Immovable::default())
                .build(),
        )
    }

    levels
}

///
/// Makes all the keys in the level.
///
pub(crate) fn make_keys(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    key_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 12);

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in key_coordinates {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Movable::default())
                .with(Key::default())
                .with(Holding::new())
                .build(),
        )
    }

    levels
}

///
/// Makes all the door switches in the level
///
pub(crate) fn make_switches(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    switch_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 14);

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in switch_coordinates {
        levels.push(
            world
                .create_entity()
                .with({
                    let mut t = Transform::default();
                    t.prepend_translation_z(-9.0);
                    t
                })
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Switch::default())
                .build(),
        )
    }

    levels
}

///
/// Makes all the doors in the level
///
pub(crate) fn make_doors(
    world: &mut World,
    sprite_sheet_handle: &Handle<SpriteSheet>,
    door_coordinates: Vec<(Int, Int)>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 16);

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in door_coordinates {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Door::default())
                .with(Immovable::default())
                .build(),
        )
    }

    levels
}

///
/// Initializes the camera in the level.
///
/// It creates a camera with 1270x720 dimensions except we divide both of those numbers by 8 since
/// our sprite are so small.
///
/// We also center the camera on based on the level dimenisons, othwerise the level would always
/// have the bottom left corner on the center of the screen and the 3/4 of the screen would be
/// empty.
///
pub(crate) fn make_camera(world: &mut World, level_handle: &Handle<GameLevel>) -> Entity {
    let (size_x, size_y) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.dimensions
    };

    let (x_adjust, y_adjust): (f32, f32) = (
        (size_x * GRID_SIZE / 2).into(),
        (size_y * GRID_SIZE / 2).into(),
    );

    world
        .create_entity()
        .with(Camera::standard_2d(1280.0/8.0, 720.0/8.0))
        .with({
            // I just want to call Transform::from_xyz(x, y, z)...
            let mut transform = Transform::default();
            transform.set_translation_xyz(x_adjust, y_adjust, 5.0);
            transform
        })
        .build()
}
