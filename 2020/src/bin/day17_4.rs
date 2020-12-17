use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use anyhow::{format_err, Result};
//use maplit::hashmap;
use structopt::StructOpt;
use thiserror::Error;

use aoc_2020::{read_lines, Args};

#[derive(Clone, Debug)]
enum Tile {
    Active,
    Inactive,
}

#[derive(Error, Debug)]
enum ParseTileError {
    #[error("unknown tile \"{0}\"")]
    Unknown(String),
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Tile::Inactive),
            '#' => Ok(Tile::Active),
            state => Err(ParseTileError::Unknown(state.to_string())),
        }
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Active => '#',
            Tile::Inactive => '.',
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Vec4 {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Vec4 {
    fn neighbors(&self) -> Vec<Vec4> {
        let mut points = Vec::new();

        for z_mod in -1..=1 {
            for y_mod in -1..=1 {
                for x_mod in -1..=1 {
                    for w_mod in -1..=1 {
                        if z_mod == 0 && y_mod == 0 && x_mod == 0 && w_mod == 0 {
                            continue;
                        }

                        points.push(
                            (
                                self.w + w_mod,
                                self.x + x_mod,
                                self.y + y_mod,
                                self.z + z_mod,
                            )
                                .into(),
                        );
                    }
                }
            }
        }

        points
    }
}

impl From<(i64, i64, i64, i64)> for Vec4 {
    fn from(tuple: (i64, i64, i64, i64)) -> Self {
        Self {
            w: tuple.0,
            x: tuple.1,
            y: tuple.2,
            z: tuple.3,
        }
    }
}

impl From<&Vec4> for Vec4 {
    fn from(v: &Vec4) -> Self {
        Self {
            w: v.w,
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

struct NeighborStates {
    active: usize,
    inactive: usize,
}

impl NeighborStates {
    fn new() -> Self {
        Self {
            active: 0,
            inactive: 0,
        }
    }
}

#[derive(Debug)]
struct Map4 {
    min_w: i64,
    min_x: i64,
    min_y: i64,
    min_z: i64,
    max_w: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64,

    tiles: HashMap<Vec4, Tile>,
}

impl Map4 {
    fn new() -> Self {
        Self {
            min_w: 0,
            min_x: 0,
            min_y: 0,
            min_z: 0,
            max_w: 0,
            max_x: 0,
            max_y: 0,
            max_z: 0,
            tiles: HashMap::new(),
        }
    }

    fn set_tile<T: Into<Vec4>>(&mut self, point: T, tile: Tile) {
        let point: Vec4 = point.into();

        self.min_w = std::cmp::min(point.w, self.min_w);
        self.min_x = std::cmp::min(point.x, self.min_x);
        self.min_y = std::cmp::min(point.y, self.min_y);
        self.min_z = std::cmp::min(point.z, self.min_z);

        self.max_w = std::cmp::max(point.w, self.max_w);
        self.max_x = std::cmp::max(point.x, self.max_x);
        self.max_y = std::cmp::max(point.y, self.max_y);
        self.max_z = std::cmp::max(point.z, self.max_z);

        self.tiles.insert(point, tile);
    }

    fn get_tile<T: Into<Vec4>>(&self, point: T) -> Tile {
        self.tiles
            .get(&point.into())
            .cloned()
            .unwrap_or(Tile::Inactive)
    }

    fn tile_neighbor_states<T: Into<Vec4>>(&self, point: T) -> NeighborStates {
        let mut states = NeighborStates::new();

        let point = point.into();
        for neighbor in point.neighbors() {
            match self.get_tile(neighbor) {
                Tile::Active => states.active += 1,
                Tile::Inactive => states.inactive += 1,
            }
        }

        states
    }

    fn tick(&self) -> Self {
        let mut new_map = Self::new();

        for z in self.min_z - 1..=self.max_z + 1 {
            for y in self.min_y - 1..=self.max_y + 1 {
                for x in self.min_x - 1..=self.max_x + 1 {
                    for w in self.min_w - 1..=self.max_w + 1 {
                        let position: Vec4 = (w, x, y, z).into();

                        match self.get_tile(position.clone()) {
                            Tile::Active => {
                                let states = self.tile_neighbor_states(position.clone());
                                if states.active == 2 || states.active == 3 {
                                    new_map.set_tile(position, Tile::Active);
                                } else {
                                    new_map.set_tile(position, Tile::Inactive);
                                }
                            }
                            Tile::Inactive => {
                                let states = self.tile_neighbor_states(position.clone());
                                if states.active == 3 {
                                    new_map.set_tile(position, Tile::Active);
                                } else {
                                    new_map.set_tile(position, Tile::Inactive);
                                }
                            }
                        }
                    }
                }
            }
        }

        new_map
    }

    fn count_active(&self) -> usize {
        let mut count = 0;

        for state in self.tiles.values() {
            if let Tile::Active = state {
                count += 1;
            }
        }

        count
    }

    /*
    fn print(&self) {
        for z in self.min_z..=self.max_z {
            println!("z={}", z);
            for y in self.min_y..=self.max_y {
                let mut line: Vec<char> = Vec::new();
                for x in self.min_x..=self.max_x {
                    line.push(self.get_tile((x, y, z)).into());
                }
                println!("{}", line.iter().collect::<String>());
            }
        }
    }
    */
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut map = Map4::new();

    let contents = std::fs::read_to_string(&args.filename)?;
    let mut y: i64 = 0;
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            map.set_tile((0, x as i64, y, 0), c.try_into()?);
        }

        y += 1;
    }

    //println!("start");
    //map.print();
    for i in 0..6 {
        map = map.tick();

        //println!("");
        //println!("tick {}", i + 1);
        //map.print();
    }

    println!("Part 1: {}", map.count_active());

    Ok(())
}
