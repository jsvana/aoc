use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Unknown direction \"{}\"", value)),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    magnitude: usize,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let direction: Direction = parts
            .get(0)
            .ok_or_else(|| anyhow!("Move \"{}\" missing direction", value))?
            .parse()?;
        let magnitude: usize = parts
            .get(1)
            .ok_or_else(|| anyhow!("Move \"{}\" missing magnitude", value))?
            .parse()?;

        Ok(Self {
            direction,
            magnitude,
        })
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn zero() -> Self {
        Position { x: 0, y: 0 }
    }
}

fn clamp(num: i32, min: i32, max: i32) -> i32 {
    if num > max {
        max
    } else if num < min {
        min
    } else {
        num
    }
}

fn adjust_positions(position1: &mut Position, position2: &mut Position) {
    let x_delta = position1.x - position2.x;
    let y_delta = position1.y - position2.y;
    if x_delta.abs() == 2 || y_delta.abs() == 2 {
        position2.x += clamp(x_delta, -1, 1);
        position2.y += clamp(y_delta, -1, 1);
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        let mut knots = Vec::new();
        for _ in 0..knot_count {
            knots.push(Position::zero());
        }

        Rope { knots }
    }

    fn process_movement(&mut self, movement: &Movement, visited: &mut HashSet<(i32, i32)>) {
        visited.insert((
            self.knots[self.knots.len() - 1].x,
            self.knots[self.knots.len() - 1].y,
        ));

        for _ in 0..movement.magnitude {
            match movement.direction {
                Direction::Up => {
                    self.knots[0].y -= 1;
                }
                Direction::Down => {
                    self.knots[0].y += 1;
                }
                Direction::Left => {
                    self.knots[0].x -= 1;
                }
                Direction::Right => {
                    self.knots[0].x += 1;
                }
            }

            for i in 0..self.knots.len() - 1 {
                let mut first = self.knots.remove(i);
                let mut second = self.knots.remove(i);
                adjust_positions(&mut first, &mut second);
                self.knots.insert(i, second);
                self.knots.insert(i, first);
            }

            visited.insert((
                self.knots[self.knots.len() - 1].x,
                self.knots[self.knots.len() - 1].y,
            ));
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut rope = Rope::new(2);
    let mut visited = HashSet::new();

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let movement: Movement = line.parse()?;
        rope.process_movement(&movement, &mut visited);
    }

    println!("{}", visited.len());

    let mut rope = Rope::new(10);
    let mut visited = HashSet::new();

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let movement: Movement = line.parse()?;
        rope.process_movement(&movement, &mut visited);
    }

    println!("{}", visited.len());

    Ok(())
}
