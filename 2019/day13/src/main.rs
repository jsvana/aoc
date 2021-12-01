mod intcode;

use std::cmp::{max, min};
use std::collections::{BTreeMap, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

use anyhow::{format_err, Error, Result};

use crate::intcode::{Program, ProgramState};

type Map = BTreeMap<i64, BTreeMap<i64, Tile>>;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(format_err!("Unknown tile {}", value)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "+",
                Tile::Block => "#",
                Tile::Paddle => "-",
                Tile::Ball => "O",
            }
        )
    }
}

fn count_blocks(map: &Map) -> u64 {
    let mut total = 0;
    for row in map.values() {
        for ch in row.values() {
            if let Tile::Block = *ch {
                total += 1;
            }
        }
    }

    total
}

fn print_map(map: &Map) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for (y, row) in map.iter() {
        for x in row.keys() {
            min_x = min(*x, min_x);
            max_x = max(*x, max_x);
            min_y = min(*y, min_y);
            max_y = max(*y, max_y);
        }
    }

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let tile = *map
                .get(&y)
                .unwrap_or(&BTreeMap::new())
                .get(&x)
                .unwrap_or(&Tile::Empty);
            print!("{}", tile);
        }
        println!("");
    }
}

fn set_value(map: &mut Map, x: i64, y: i64, value: Tile) {
    *map.entry(y)
        .or_insert(BTreeMap::new())
        .entry(x)
        .or_insert(Tile::Empty) = value;
}

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut inputs = VecDeque::new();

    let program_str = std::fs::read_to_string("input.txt")?;

    let mut program = Program::from_str(&program_str)?;

    program.set_memory_value(0, 2)?;

    let mut map = BTreeMap::new();
    let mut score = 0;

    let mut paddle_x = 0;

    let mut ball_x = 0;

    loop {
        if let ProgramState::Terminated = *program.get_state() {
            break;
        }

        let mut outputs = program.run_to_next_input(&mut inputs)?;

        while outputs.len() > 0 {
            let x = outputs.pop_front().unwrap();
            let y = outputs.pop_front().unwrap();

            if x == -1 && y == 0 {
                score = outputs.pop_front().unwrap();
            } else {
                let tile: Tile = outputs.pop_front().unwrap().try_into()?;

                if let Tile::Paddle = tile {
                    paddle_x = x;
                }

                if let Tile::Ball = tile {
                    ball_x = x;
                }

                set_value(&mut map, x, y, tile);
            }
        }

        if paddle_x < ball_x {
            inputs.push_back(1);
        } else if paddle_x > ball_x {
            inputs.push_back(-1);
        } else {
            inputs.push_back(0);
        }
    }

    println!("Score: {}", score);

    Ok(())
}
