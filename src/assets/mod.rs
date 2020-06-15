use amethyst::{
    assets::{Asset, Handle, Format},
    ecs::VecStorage,
    Error,
};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;
use std::convert::TryFrom;
use std::cmp::max;

pub(crate) type Coordinates = (i8, i8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CharacterPlacement {
    pub horizontal: Option<Coordinates>,
    pub vertical: Option<Coordinates>,
    pub interact: Option<Coordinates>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GameLevel {
    pub dimensions: Coordinates,
    pub characters: CharacterPlacement,
    pub exit: Coordinates,
    pub obstacles: Vec<Coordinates>,
    pub walls: Vec<Coordinates>,
    pub floors: Vec<Coordinates>,
}

impl Asset for GameLevel {
    const NAME: &'static str = "evg1::GameLevel";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<GameLevel>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct LevelFormat;

impl Format<GameLevel> for LevelFormat {
    fn name(&self) -> &'static str {
        "LevelFormat"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<GameLevel, Error> {
        let mut val = GameLevel::default();
        let mut do_floor = false;

        let height = i8::try_from(
            bytes
                .iter()
                .filter(|&b| char::from(*b) == '\n')
                .count()
        ).unwrap();

        val.dimensions.1 = height;

        let (mut x, mut y) = (0, height);

        for byte in bytes {
            print!("[{}]", char::from(byte));
            x += 1;
            match char::from(byte) {
                'W' | 'w' | '#'  => {
                    do_floor = true;
                    val.walls.push((x, y))
                },
                'H' | 'h'  => {
                    val.characters.horizontal = Some((x,y));
                    if do_floor {
                        val.floors.push((x,y));
                    }
                },
                'V' | 'v'  => {
                    val.characters.vertical = Some((x,y));
                    if do_floor {
                        val.floors.push((x,y));
                    }
                },
                'G' | 'g'  => {
                    val.characters.interact = Some((x,y));
                    if do_floor {
                        val.floors.push((x,y));
                    }
                },
                'C' | 'c'  => {
                    val.obstacles.push((x,y));
                    if do_floor {
                        val.floors.push((x,y));
                    }
                },
                'E' | 'e'  => {
                    val.exit = (x,y)
                },
                ' ' | '\t' => {
                    if do_floor {
                        val.floors.push((x,y));
                    }
                },
                '\n' => {
                    y -= 1;
                    val.dimensions.0 = max(val.dimensions.0, x);
                    x = 0;
                    do_floor = false;
                },
                _ => (),
            }
        }
        println!("Level: {:?}", val);
        Ok(val)
    }
}
