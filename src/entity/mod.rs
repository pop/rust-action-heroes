use crate::assets::GameLevel;
use crate::component::{Exit, Holding, Movable, Name, Named};
use crate::lib::get_sprite;
use amethyst::{
    assets::{AssetStorage, Handle},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, SpriteSheet},
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
                .with(sprite.clone())
                .with(Movable::new(x, y))
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
                .with(sprite.clone())
                .with(Movable::new(x, y))
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
                .with(sprite.clone())
                .with(Movable::new(x, y))
                .with(Named::new(Name::Interact))
                .with(Holding::new())
                .build(),
        )
    } else {
        None
    }
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
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Movable::new(n, min))
                .with(Named::new(Name::Wall))
                .with(Holding::new())
                .build(),
        );
        entities.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Movable::new(size_x, n))
                .with(Named::new(Name::Wall))
                .with(Holding::new())
                .build(),
        );
    }
    for n in min..=size_y {
        entities.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Movable::new(min, n))
                .with(Named::new(Name::Wall))
                .with(Holding::new())
                .build(),
        );
        entities.push(
            world
                .create_entity()
                .with(Transform::default())
                .with(sprite.clone())
                .with(Movable::new(n, size_y))
                .with(Named::new(Name::Wall))
                .with(Holding::new())
                .build(),
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

    for x in 1..=size_x {
        for y in 1..=size_y {
            entities.push(
                world
                    .create_entity()
                    .with(Transform::default())
                    .with(sprite.clone())
                    .with(Movable::new(x, y))
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
                .with(Transform::default())
                .with(sprite.clone())
                .with(Movable::new(x, y))
                .with(Named::new(Name::Crate))
                .with(Holding::new())
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
        .with(Exit::new())
        .with(Movable::new(x, y))
        .build()
}

pub(crate) fn make_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::standard_2d(100.0, 100.0))
        .with({
            // I just want to call Transform::from_xyz(x, y, z)...
            let mut transform = Transform::default();
            transform.set_translation_xyz(50.0, 50.0, 5.0);
            transform
        })
        .build()
}
