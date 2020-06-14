use crate::assets::GameLevel;
use crate::component::{Exit, Holding, Position, Movable, Name, Named};
use crate::lib::get_sprite;
use crate::system::grid::GRID_SIZE;
use amethyst::{
    assets::{AssetStorage, Handle},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, SpriteSheet, SpriteRender},
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
        println!("Creating horizontal at {:?}", (x, y));
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
        println!("Creating vertical at {:?}", (x, y));
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
        println!("Creating interact at {:?}", (x, y));
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

fn make_wall(world: &mut World, sprite: SpriteRender, (x, y): (i8, i8)) -> Entity {
    world
        .create_entity()
        .with(Transform::default())
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

    let (size_x, size_y) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.dimensions
    };

    let min = 0;

    let mut entities: Vec<Entity> = Vec::new();

    for n in min..=size_x {
        entities.push(
            make_wall(world, sprite.clone(), (n, min))
        );
        entities.push(
            make_wall(world, sprite.clone(), (size_x, n))
        );
    }
    for n in min..size_y {
        entities.push(
            make_wall(world, sprite.clone(), (min, n))
        );
        entities.push(
            make_wall(world, sprite.clone(), (n, size_y))
        );
    }
    entities
}

pub(crate) fn make_floor(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let (size_x, size_y) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.dimensions
    };

    let sprite = get_sprite(sprite_sheet_handle, 9);

    let mut entities: Vec<Entity> = Vec::new();

    for x in 1..size_x {
        for y in 1..size_y {
            entities.push(
                world
                    .create_entity()
                    .with(Transform::default())
                    .with(sprite.clone())
                    .with(Position::new(x, y))
                    .build(),
            );
        }
    }
    entities
}

pub(crate) fn make_crates(
    world: &mut World,
    level_handle: &Handle<GameLevel>,
    sprite_sheet_handle: &Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite = get_sprite(sprite_sheet_handle, 6);
    let obstacles = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.obstacles.to_vec()
    };
    let mut entities: Vec<Entity> = Vec::new();
    for (x, y) in obstacles {
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

pub(crate) fn make_camera(world: &mut World, level_handle: &Handle<GameLevel>) -> Entity {
    let (size_x, size_y) = {
        let asset_storage = world.read_resource::<AssetStorage<GameLevel>>();
        let level: &GameLevel = asset_storage
            .get(&level_handle)
            .expect("Cannot load game level");
        level.dimensions
    };

    let (x_adjust, y_adjust): (f32, f32) = ((size_x * GRID_SIZE / 2).into(), (size_y * GRID_SIZE / 2).into());

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
