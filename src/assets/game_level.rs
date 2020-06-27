use crate::lib::Int;
use amethyst::{
    assets::{Asset, Format, Handle},
    ecs::VecStorage,
    Error,
};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::convert::TryFrom;
use std::iter::Iterator;

pub(crate) type Coordinates = (Int, Int);

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
    pub crates: Vec<Coordinates>,
    pub walls: Vec<Coordinates>,
    pub floors: Vec<Coordinates>,
    pub locks: Vec<Coordinates>,
    pub keys: Vec<Coordinates>,
    pub switches: Vec<Coordinates>,
    pub doors: Vec<Coordinates>,
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

        let height =
            Int::try_from(bytes.iter().filter(|&b| char::from(*b) == '\n').count()).unwrap();

        val.dimensions.1 = height;

        let (mut x, mut y) = (0, height);

        for byte in bytes {
            x += 1;
            match char::from(byte) {
                'W' | 'w' | '#' => {
                    do_floor = true;
                    val.walls.push((x, y))
                }
                'H' | 'h' => {
                    val.characters.horizontal = Some((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'V' | 'v' => {
                    val.characters.vertical = Some((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'G' | 'g' => {
                    val.characters.interact = Some((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'C' | 'c' => {
                    val.crates.push((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'E' | 'e' => val.exit = (x, y),
                'K' | 'k' => {
                    val.keys.push((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'L' | 'l' => {
                    val.locks.push((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                'S' | 's' => {
                    val.switches.push((x, y));
                }
                'D' | 'd' => {
                    val.doors.push((x, y));
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                ' ' => {
                    if do_floor {
                        val.floors.push((x, y));
                    }
                }
                '\t' => {
                    // tabs are four spaces
                    if do_floor {
                        val.floors.push((x, y));
                        val.floors.push((x, y));
                        val.floors.push((x, y));
                        val.floors.push((x, y));
                    }
                }
                '\n' => {
                    y -= 1;
                    val.dimensions.0 = max(val.dimensions.0, x);
                    x = 0;
                    do_floor = false;
                }
                _ => (),
            }
        }
        Ok(val)
    }
}
