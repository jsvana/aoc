mod intcode;
mod point;

use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::fmt;

use anyhow::{format_err, Error, Result};
use log::debug;

use intcode::Program;
use point::Point;

// TODO(jsvana): Make a generic?
struct Map {
    data: BTreeMap<i64, BTreeMap<i64, Tile>>,
}

impl Map {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn set_point(&mut self, point: &Point, tile: &Tile) {
        self.data
            .entry(point.y)
            .or_insert(BTreeMap::new())
            .insert(point.x, tile.clone());
    }

    fn get_point(&self, point: &Point) -> Tile {
        self.data
            .get(&point.y)
            .unwrap_or(&BTreeMap::new())
            .get(&point.x)
            .unwrap_or(&Tile::Unknown)
            .clone()
    }

    fn to_string(&self, robot_position: &Point) -> String {
        let mut top_left = Point::max();
        let mut bottom_right = Point::min();

        for (y, row) in self.data.iter() {
            for x in row.keys() {
                top_left.x = min(top_left.x, *x);
                top_left.y = min(top_left.y, *y);
                bottom_right.x = max(bottom_right.x, *x);
                bottom_right.y = max(bottom_right.y, *y);
            }
        }

        let row_size = bottom_right.x - top_left.x + 1;

        let mut rows = Vec::new();
        for y in top_left.y..bottom_right.y + 1 {
            let mut row = String::with_capacity(row_size as usize);
            for x in top_left.x..bottom_right.x + 1 {
                if robot_position.x == x && robot_position.y == y {
                    row.push('R');
                } else {
                    row.push(self.get_point(&Point { x, y }).into());
                }
            }
            rows.push(row);
        }

        rows.join("\n")
    }
}

#[derive(Clone, Debug)]
enum Tile {
    Floor,
    Wall,
    Oxygen,
    Unknown,
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> char {
        match value {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Oxygen => 'O',
            Tile::Unknown => ' ',
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Oxygen => 'O',
            Tile::Unknown => ' ',
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "north",
                Direction::South => "south",
                Direction::West => "west",
                Direction::East => "east",
            }
        )
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl From<&Direction> for i64 {
    fn from(value: &Direction) -> i64 {
        match value {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

enum MoveResult {
    HitWall,
    MovedOneStep,
    MovedOneStepAndFoundOxygen,
}

impl TryFrom<i64> for MoveResult {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MoveResult::HitWall),
            1 => Ok(MoveResult::MovedOneStep),
            2 => Ok(MoveResult::MovedOneStepAndFoundOxygen),
            _ => Err(format_err!("Unknown move result {}", value)),
        }
    }
}

fn point_in_direction(point: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::North => Point {
            x: point.x,
            y: point.y - 1,
        },
        Direction::South => Point {
            x: point.x,
            y: point.y + 1,
        },
        Direction::West => Point {
            x: point.x - 1,
            y: point.y,
        },
        Direction::East => Point {
            x: point.x + 1,
            y: point.y,
        },
    }
}

fn move_robot(robot: &mut Point, direction: &Direction) {
    match direction {
        Direction::North => robot.y -= 1,
        Direction::South => robot.y += 1,
        Direction::West => robot.x -= 1,
        Direction::East => robot.x += 1,
    }
}

fn get_path(map: &Map, start: &Point, end: &Point) -> Option<Vec<Direction>> {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut visited = BTreeSet::new();

    let mut to_visit = VecDeque::new();
    let mut paths = VecDeque::new();

    to_visit.push_back(start.as_tuple());
    paths.push_back(Vec::new());

    let end_tuple = end.as_tuple();

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();
        let next_path = paths.pop_front().unwrap();

        if visited.contains(&next) {
            continue;
        }

        visited.insert(next);

        if next == end_tuple {
            return Some(next_path);
        }

        for direction in directions.iter() {
            let next_point = point_in_direction(&Point::from_tuple(&next), direction);
            if let Tile::Unknown | Tile::Wall = map.get_point(&next_point) {
                continue;
            }

            let mut further_path = next_path.clone();
            further_path.push(direction.clone());

            let point_tuple = next_point.as_tuple();
            to_visit.push_back(point_tuple);

            paths.push_back(further_path);
        }
    }

    None
}

fn follow_path(program: &mut Program, robot: &mut Point, path: &Vec<Direction>) -> Result<()> {
    for direction in path.iter() {
        let mut inputs = VecDeque::new();
        inputs.push_back(direction.into());
        let output = program.run_to_next_output(&mut inputs)?.unwrap();

        let move_result: MoveResult = output.try_into()?;

        match move_result {
            MoveResult::HitWall => {
                return Err(format_err!(
                    "Error moving requested direction (robot at position {}, direction {:?})",
                    robot,
                    direction
                ));
            }
            MoveResult::MovedOneStep => {
                move_robot(robot, &direction);
            }
            MoveResult::MovedOneStepAndFoundOxygen => {
                move_robot(robot, &direction);
            }
        }
    }

    Ok(())
}

fn move_once_in_direction(
    program: &mut Program,
    robot: &mut Point,
    map: &mut Map,
    direction: &Direction,
) -> Result<MoveResult> {
    let mut inputs = VecDeque::new();
    inputs.push_back(direction.into());
    let output = program.run_to_next_output(&mut inputs)?.unwrap();

    let move_result: MoveResult = output.try_into()?;

    match move_result {
        MoveResult::HitWall => {
            // Set this one more in the direction
            map.set_point(&point_in_direction(&robot, direction), &Tile::Wall);
        }
        MoveResult::MovedOneStep => {
            move_robot(robot, &direction);
            map.set_point(&robot, &Tile::Floor);
        }
        MoveResult::MovedOneStepAndFoundOxygen => {
            move_robot(robot, &direction);
            map.set_point(&robot, &Tile::Oxygen);
        }
    }

    Ok(move_result)
}

fn populate_map(program: &mut Program, map: &mut Map, start: &Point) -> Result<Option<Point>> {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut visited = BTreeSet::new();

    let mut to_visit = VecDeque::new();

    for direction in directions.iter() {
        to_visit.push_front((start.x, start.y, direction));
        //to_visit.push_back((start.x, start.y, direction));
    }

    let mut oxygen_point = None;

    let mut previous_point = start.clone();
    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();

        if visited.contains(&next) {
            continue;
        }

        //println!("\x1B[2J{}", map.to_string(&previous_point));

        visited.insert(next);

        let mut point = Point {
            x: next.0,
            y: next.1,
        };

        // Navigate to the next point
        let back_path = get_path(&map, &previous_point, &point).ok_or(format_err!(
            "Unable to find path from {} to {}",
            previous_point,
            point
        ))?;
        follow_path(program, &mut previous_point, &back_path)?;

        let next_direction = next.2;
        let result = move_once_in_direction(program, &mut point, map, &next_direction)?;
        if let MoveResult::MovedOneStep | MoveResult::MovedOneStepAndFoundOxygen = result {
            previous_point = point.clone();

            if let MoveResult::MovedOneStepAndFoundOxygen = result {
                oxygen_point = Some(point.clone());
            }

            for direction in directions.iter() {
                to_visit.push_front((point.x, point.y, direction));
                //to_visit.push_back((point.x, point.y, direction));
            }
        }
    }

    Ok(oxygen_point)
}

fn count_shortest_path(map: &Map, start: &Point, end: &Point) -> Option<u64> {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut scores = BTreeMap::new();

    let mut visited = BTreeSet::new();

    let mut to_visit = VecDeque::new();

    let start_tuple = start.as_tuple();
    to_visit.push_back(start_tuple);

    scores.insert(start_tuple, 0);

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();

        if visited.contains(&next) {
            continue;
        }

        visited.insert(next);

        let point_score = scores.get(&next).unwrap().clone();

        for direction in directions.iter() {
            let next_point = point_in_direction(&Point::from_tuple(&next), direction);
            if let Tile::Unknown | Tile::Wall = map.get_point(&next_point) {
                continue;
            }

            let point_tuple = next_point.as_tuple();
            to_visit.push_back(point_tuple);

            let next_score = scores.get(&point_tuple).unwrap_or(&std::u64::MAX).clone();
            scores.insert(point_tuple, min(next_score, point_score + 1));
        }
    }

    scores.get(&end.as_tuple()).cloned()
}

fn visit_all(map: &mut Map, start: &Point) -> u64 {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut scores = BTreeMap::new();

    let mut visited = BTreeSet::new();

    let mut to_visit = VecDeque::new();

    let start_tuple = start.as_tuple();
    to_visit.push_back(start_tuple);

    scores.insert(start_tuple, 0);

    let mut max_count = 0;

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();

        if visited.contains(&next) {
            continue;
        }

        visited.insert(next);

        let point_score = scores.get(&next).unwrap().clone();

        for direction in directions.iter() {
            let next_point = point_in_direction(&Point::from_tuple(&next), direction);
            if let Tile::Unknown | Tile::Wall = map.get_point(&next_point) {
                continue;
            }

            let point_tuple = next_point.as_tuple();
            to_visit.push_back(point_tuple);

            let mut next_score = scores.get(&point_tuple).unwrap_or(&std::u64::MAX).clone();
            next_score = min(next_score, point_score + 1);
            max_count = max(max_count, next_score);

            scores.insert(point_tuple, next_score);
        }
    }

    max_count
}

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut map = Map::new();
    let robot = Point::zero();
    map.set_point(&robot, &Tile::Floor);

    let mut program = Program::from_file("input.txt")?;

    match populate_map(&mut program, &mut map, &robot)? {
        Some(oxygen_point) => {
            println!("Found oxygen at {}", oxygen_point);
            println!(
                "Shortest_path: {}",
                count_shortest_path(&map, &Point::zero(), &oxygen_point).unwrap()
            );
            println!("Minutes to fill: {}", visit_all(&mut map, &oxygen_point));
        }
        None => println!("Unable to find oxygen"),
    }

    Ok(())
}
