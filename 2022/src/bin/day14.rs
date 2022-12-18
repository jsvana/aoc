use std::collections::HashMap;
use std::fmt;

use anyhow::Result;
use aoc_2022::{Args, Point};
use structopt::StructOpt;

#[derive(Clone, Debug)]
enum Space {
    Empty,
    Rock,
    Sand,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Empty => ".",
                Space::Rock => "#",
                Space::Sand => "O",
            }
        )
    }
}

#[derive(Debug)]
struct Matrix {
    pub data: HashMap<Point, Space>,
    pub max_y: usize,
}

#[derive(Debug)]
enum FallResult {
    Placed,
    FallingForever,
}

impl Matrix {
    fn new() -> Self {
        Matrix {
            data: HashMap::new(),
            max_y: 0,
        }
    }

    fn get(&self, point: &Point) -> Space {
        self.data.get(point).cloned().unwrap_or(Space::Empty)
    }

    fn add_point(&mut self, point: Point, space: Space) {
        self.max_y = std::cmp::max(self.max_y, point.y);
        self.data.insert(point, space);
    }

    fn add_line(&mut self, start: Point, end: Point, space: Space) {
        for i in std::cmp::min(start.y, end.y)..=std::cmp::max(start.y, end.y) {
            for j in std::cmp::min(start.x, end.x)..=std::cmp::max(start.x, end.x) {
                self.add_point(Point::new(j, i), space.clone());
            }
        }
    }

    fn add_sand(&mut self, start: Point) -> FallResult {
        let mut current_point = start;
        loop {
            if current_point.y > self.max_y {
                return FallResult::FallingForever;
            }

            let down_point = Point::new(current_point.x, current_point.y + 1);
            if let Space::Empty = self.get(&down_point) {
                current_point = down_point;
                continue;
            }

            let down_left_point = Point::new(current_point.x - 1, current_point.y + 1);
            if let Space::Empty = self.get(&down_left_point) {
                current_point = down_left_point;
                continue;
            }

            let down_right_point = Point::new(current_point.x + 1, current_point.y + 1);
            if let Space::Empty = self.get(&down_right_point) {
                current_point = down_right_point;
                continue;
            }

            self.add_point(current_point, Space::Sand);
            return FallResult::Placed;
        }
    }

    fn render(&self, top_left: Point, bottom_right: Point) {
        for i in top_left.y..=bottom_right.y {
            for j in top_left.x..=bottom_right.x {
                print!("{}", self.get(&Point::new(j, i)));
            }
            println!("");
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut matrix = Matrix::new();

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let mut points = Vec::new();
        let parts: Vec<&str> = line.split(" -> ").collect();
        for part in parts {
            points.push(part.parse::<Point>()?);
        }

        for i in 0..points.len() - 1 {
            matrix.add_line(points[i].clone(), points[i + 1].clone(), Space::Rock);
        }
    }

    let mut count = 0;
    loop {
        if let FallResult::FallingForever = matrix.add_sand(Point::new(500, 0)) {
            break;
        }

        count += 1;
    }

    println!("{}", count);

    let mut matrix = Matrix::new();

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let mut points = Vec::new();
        let parts: Vec<&str> = line.split(" -> ").collect();
        for part in parts {
            points.push(part.parse::<Point>()?);
        }

        for i in 0..points.len() - 1 {
            matrix.add_line(points[i].clone(), points[i + 1].clone(), Space::Rock);
        }
    }

    let line_y = matrix.max_y + 2;
    matrix.add_line(
        Point::new(0, line_y),
        Point::new(100000, line_y),
        Space::Rock,
    );

    let mut count = 0;
    loop {
        matrix.add_sand(Point::new(500, 0));
        count += 1;
        if let Space::Sand = matrix.get(&Point::new(500, 0)) {
            break;
        }
    }

    println!("{}", count);

    Ok(())
}
