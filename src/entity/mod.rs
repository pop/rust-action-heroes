use crate::assets::GameLevel;
use crate::component::{Exit, Immovable, Holding, Key, Lock, Movable, Name, Named, Position, Switch, Door};
use crate::lib::{get_sprite, Int};
use crate::system::grid::GRID_SIZE;
use amethyst::{
    assets::{AssetStorage, Handle},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet},
};

pub(crate) fn make_horizontal(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Option<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 2);

    if let Some((x, y)) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.characters.horizontal
    } {
        Some(
            world
                .create_entity()
                .with(Transform::default())
                .with(Movable::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Named::new(Name::Horizontal))
                .with(Holding::new())
                .build(),
        )
    } else {
        None
    }
}

pub(crate) fn make_vertical(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Option<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 0);

    if let Some((x, y)) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.characters.vertical
    } {
        Some(
            world
                .create_entity()
                .with(Transform::default())
                .with(Movable::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Named::new(Name::Vertical))
                .with(Holding::new())
                .build(),
        )
    } else {
        None
    }
}

pub(crate) fn make_interact(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Option<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 4);

    if let Some((x, y)) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.characters.interact
    } {
        Some(
            world
                .create_entity()
                .with(Transform::default())
                .with(Movable::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Named::new(Name::Interact))
                .with(Holding::new())
                .build(),
        )
    } else {
        None
    }
}

fn make_wall(world: &mut World, sprite: SpriteRender, (x, y): (Int, Int)) -> Entity {
    world
        .create_entity()
        .with(Transform::default())
        .with(Immovable::default())
        .with(Position::new(x, y))
        .with(sprite)
        .build()
}

pub(crate) fn make_walls(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 10);

    let wall_coordinates = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.walls.clone()
    };

    let mut entities: Vec<Entity> = Vec::new();

    for (x, y) in wall_coordinates {
        entities.push(make_wall(world, sprite.clone(), (x, y)));
    }
    entities
}

pub(crate) fn make_floor(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let floors = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.floors.clone()
    };

    let sprite = get_sprite(sprite_sheet_handle, 9);

    let mut entities: Vec<Entity> = Vec::new();

    for (x, y) in floors {
        entities.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .build(),
        );
    }
    entities
}

pub(crate) fn make_crates(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 6);
    let crates = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.obstacles.clone()
    };
    let mut entities: Vec<Entity> = Vec::new();
    for (x, y) in crates {
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

pub(crate) fn make_exit(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Entity {
    let sprite = get_sprite(sprite_sheet_handle, 8);

    let (x, y) = {
        let asset_storage = world.write_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(level_handle)
            .expect("Cannot load game level");
        level.exit
    };

    world
        .create_entity()
        .with(Transform::default())
        .with(sprite.clone())
        .with(Exit::default())
        .with(Position::new(x, y))
        .build()
}

pub(crate) fn make_locks(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 11);

    let locks = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(level_handle)
            .expect("Could not load game level");
        level.locks.clone()
    };

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in locks {
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

pub(crate) fn make_keys(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 12);

    let keys = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(level_handle)
            .expect("Could not load game level");
        level.keys.clone()
    };

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in keys {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Movable::default())
                .with(Key::default())
                .with(Holding::new()) // This is gonna be a bug
                .build(),
        )
    }

    levels
}

pub(crate) fn make_switches(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 14);

    let switches = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(level_handle)
            .expect("Could not load game level");
        level.switches.clone()
    };

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in switches {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Switch::default())
                .build(),
        )
    }

    levels
}

pub(crate) fn make_doors(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 16);

    let doors = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(level_handle)
            .expect("Could not load game level");
        level.doors.clone()
    };

    let mut levels: Vec<Entity> = Vec::new();

    for (x, y) in doors {
        levels.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Position::new(x, y))
                .with(Movable::default())
                .with(Door::default())
                .with(Immovable::default())
                .build(),
        )
    }

    levels
}

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
        .with(Camera::standard_2d(100.0, 100.0))
        .with({
            // I just want to call Transform::from_xyz(x, y, z)...
            let mut transform = Transform::default();
            transform.set_translation_xyz(x_adjust, y_adjust, 5.0);
            transform
        })
        .build()
}
