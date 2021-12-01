use std::cmp::{min, max};
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use anyhow::{Error, format_err, Result};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Distance {
    direction: Direction,
    magnitude: i64,
}

impl FromStr for Distance {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < 2 {
            return Err(format_err!("Malformed direction"));
        }

        let mut value_chars = value.chars();

        let direction_char = value_chars.next().unwrap();
        let direction = match direction_char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                return Err(format_err!("Unknown direction {}", direction_char));
            }
        };

        let magnitude: i64 = value_chars.collect::<String>().parse()?;

        Ok(Self {
            direction,
            magnitude,
        })
    }
}

#[derive(Clone, Debug, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

type PointMap = BTreeMap<i64, BTreeMap<i64, BTreeSet<i64>>>;

struct CountResult {
    found: bool,
    steps: i64,
    ending_point: Point,
}

impl Distance {
    fn add_points(&self, points: &mut PointMap, id: i64, starting_point: &mut Point) {
        for _ in 1..self.magnitude + 1 {
            match self.direction {
                Direction::Up => starting_point.y -= 1,
                Direction::Down => starting_point.y += 1,
                Direction::Left => starting_point.x -= 1,
                Direction::Right => starting_point.x += 1,
            };

            points.entry(starting_point.y).or_insert(BTreeMap::new()).entry(starting_point.x).or_insert(BTreeSet::new()).insert(id);
        }
    }

    fn count_steps(&self, starting_point: &Point, target_point: &Point) -> CountResult {
        let mut found = false;
        let mut steps = 0;

        let mut point = starting_point.clone();
        for _ in 1..self.magnitude + 1 {
            match self.direction {
                Direction::Up => point.y -= 1,
                Direction::Down => point.y += 1,
                Direction::Left => point.x -= 1,
                Direction::Right => point.x += 1,
            };

            steps += 1;
            if point == *target_point {
                found = true;
                break;
            }
        }

        CountResult {
            found,
            steps,
            ending_point: point,
        }
    }
}

#[derive(Debug)]
struct Line {
    id: i64,
    parts: Vec<Distance>,
}

impl Line {
    fn from_string(value: &str, id: i64) -> Result<Self> {
        let mut parts = Vec::new();
        for part in value.split(",") {
            parts.push(part.parse()?);
        }

        Ok(Self { id, parts })
    }

    fn add_points(&self, points: &mut PointMap) {
        let mut starting_point = Point { x: 0, y: 0 };
        points.entry(0).or_insert(BTreeMap::new()).entry(0).or_insert(BTreeSet::new()).insert(self.id);
        for part in self.parts.iter() {
            part.add_points(points, self.id, &mut starting_point);
        }
    }

    fn count_steps(&self, target_point: &Point) -> Result<i64> {
        let mut total_steps = 0;
        let mut starting_point = Point { x: 0, y: 0 };
        for part in self.parts.iter() {
            let result = part.count_steps(&starting_point, target_point);
            total_steps += result.steps;
            if result.found {
                return Ok(total_steps);
            }
            starting_point = result.ending_point;
        }

        Err(format_err!("Point not found in line"))
    }
}

fn read_input(filename: &str) -> Result<Vec<Line>> {
    let data = std::fs::read_to_string(filename)?;

    let mut lines = Vec::new();
    for line in data.split("\n").filter(|l| l.len() > 0) {
        lines.push(Line::from_string(line, lines.len() as i64)?);
    }

    Ok(lines)
}

fn print_points(points: &PointMap, top_left: &Point, bottom_right: &Point) {
    let mut horizontal = Vec::new();
    horizontal.resize((bottom_right.x - top_left.x + 1) as usize, ".".to_string());
    let mut data = Vec::new();
    data.resize((bottom_right.y - top_left.y + 1) as usize, horizontal);

    for (y, xs) in points.iter() {
        for (x, ids) in xs.iter() {
            let x = (x - top_left.x) as usize;
            let y = (y - top_left.y) as usize;
            if ids.len() > 1 {
                data[y][x] = "*".to_string();
            } else if ids.len() == 1 {
                data[y][x] = format!("{}", ids.iter().next().unwrap());
            }
        }
    }

    for i in 0..data.len() {
        for j in 0 ..data[i].len() {
            print!("{} ", data[i][j]);
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let mut points: PointMap = BTreeMap::new();
    let lines = read_input("input.txt")?;
    for line in lines.iter() {
        line.add_points(&mut points);
    }

    let mut min_distance = std::i64::MAX;
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;

    for (y, xs) in points.iter() {
        for (x, ids) in xs.iter() {
            min_x = min(*x, min_x);
            min_y = min(*y, min_y);
            max_x = max(*x, max_x);
            max_y = max(*y, max_y);

            if ids.len() > 1 && *x != 0 && *y != 0{
                let mut total_line_distance = 0;
                for id in ids.iter() {
                    total_line_distance += lines[*id as usize].count_steps(&Point { x: *x, y: *y })?;
                }
                min_distance = min(min_distance, total_line_distance);
            }
        }
    }

    //print_points(&points, &Point { x: min_x, y: min_y }, &Point { x: max_x, y: max_x });

    println!("Min distance: {}", min_distance);

    Ok(())
}
