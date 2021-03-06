//!
//! Game Level Asset
//!
//! The GameLevel Asset is a struct containing necessary info to construct a level.
//!
//! This module also includes the method used to parse `.level` files into levels.
//!

use crate::lib::Int;
use amethyst::{
    assets::{Asset, Format, Handle},
    ecs::VecStorage,
    Error,
};
use serde::{Deserialize, Serialize};
use std::{cmp::max, convert::TryFrom, iter::Iterator};

///
/// It's not less characters, but it is easier to remember what coordinates are.
///
pub(crate) type Coordinates = (Int, Int);

///
/// CharacterPlacement tracks which of the playable characters are around and where their
/// coordinates are at the start of the level.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CharacterPlacement {
    pub horizontal: Option<Coordinates>,
    pub vertical: Option<Coordinates>,
    pub interact: Option<Coordinates>,
}

///
/// Tracks all metadata about a level necessary to construct it when the level starts.
///
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
    const NAME: &'static str = "rust_action_heroes::GameLevel";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<GameLevel>>;
}

///
/// LevelFormat, like RonFormat, is a file format for storing data.
///
/// Amethyst requires I implement this to store custom meta-data.
///
/// I tried for a long time to store levels in Ron format, but storing an entire level in Ron was
/// unweildy.
///
/// Instead I opted to write my own ascii-art level file format and write a parser for it.
/// The parser just goes character by character and adds coordinates for different entity types to
/// the GameLevel struct.
///
/// As an added bonus, you can write your own levels for Rust Action heroes pretty easiliy!
/// * W | w | # -> Wall
/// * H | h -> Prince horizontival the first
/// * V | v -> Duke vert the pure
/// * G | g -> Grabaron the wise
/// * C | c -> Crate
/// * E | e -> Exit
/// * K | k -> Key (for locks)
/// * L | l -> Locks (for keys)
/// * S | s -> Switch (for doors)
/// * D | d -> Door (for switches)
/// * <space> | <tab> -> Floor
///
/// ## Example level
///
/// ```text
/// ######
/// #g h #
/// # v s#####
/// #ksc  lde#
/// ##########
/// ```
///
/// This has all three playable characters, a crate, a key, a lock, a two switches, a door, and an exit.
/// Is that level even beatable?
///
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
