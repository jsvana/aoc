use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

use aoc_2021::{read_lines, Args};

fn part2(lines: Vec<String>) -> Result<()> {
    Ok(())
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input_lines: Vec<String> = read_lines(&args.filename)?;

    let mut map: HashMap<Point, usize> = HashMap::new();

    let mut lines = Vec::new();
    for line in input_lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let point1: Vec<&str> = parts[0].split(',').collect();
        let point2: Vec<&str> = parts[2].split(',').collect();

        lines.push(Line {
            start: Point {
                x: point1[0].parse()?,
                y: point1[1].parse()?,
            },
            end: Point {
                x: point2[0].parse()?,
                y: point2[1].parse()?,
            },
        });
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        if line.start.x > max_x {
            max_x = line.start.x;
        }
        if line.start.y > max_y {
            max_y = line.start.y;
        }

        if (line.start.y != line.end.y && line.start.x != line.end.x) {
            let mut x = line.start.x;
            let mut y = line.start.y;
            *map.entry(Point { x, y }).or_insert(0) += 1;
            while (x != line.end.x) {
                if x > line.end.x {
                    x -= 1;
                } else {
                    x += 1;
                }
                if y > line.end.y {
                    y -= 1;
                } else {
                    y += 1;
                }
                *map.entry(Point { x, y }).or_insert(0) += 1;
            }

            continue;
        }

        let (start_y, end_y) = if line.start.y <= line.end.y {
            (line.start.y, line.end.y)
        } else {
            (line.end.y, line.start.y)
        };

        let (start_x, end_x) = if line.start.x <= line.end.x {
            (line.start.x, line.end.x)
        } else {
            (line.end.x, line.start.x)
        };

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                *map.entry(Point { x, y }).or_insert(0) += 1;
            }
        }
    }

    /*
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", map.get(&Point { x, y }).unwrap_or(&0));
        }
        println!("");
    }
    */

    let mut total = 0;
    for (point, count) in map {
        if count >= 2 {
            total += 1;
        }
    }

    println!("{}", total);

    part2(input_lines)
}
