use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};
use aoc_2022::{Args, Matrix, Point};
use structopt::StructOpt;

#[derive(Debug)]
struct Map {
    matrix: Matrix<usize>,
    start: Point,
    end: Point,
}

fn read_map(filename: &str) -> Result<Map> {
    let contents = std::fs::read_to_string(filename)?;

    let mut data: Vec<Vec<usize>> = Vec::new();
    let mut widths = HashSet::new();
    let mut height = 0;
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (i, line) in contents.lines().enumerate() {
        height += 1;

        let mut row: Vec<usize> = Vec::new();
        for (j, mut c) in line.chars().enumerate() {
            if c == 'S' {
                match start {
                    Some(p) => {
                        return Err(anyhow!(
                            "Found second start point (first at {}, second at ({}, {}))",
                            p,
                            j,
                            i
                        ));
                    }
                    None => {
                        start = Some(Point::new(j, i));
                    }
                }
                c = 'a';
            } else if c == 'E' {
                match end {
                    Some(p) => {
                        return Err(anyhow!(
                            "Found second end point (first at {}, second at ({}, {}))",
                            p,
                            j,
                            i
                        ));
                    }
                    None => {
                        end = Some(Point::new(j, i));
                    }
                }
                c = 'z';
            }

            row.push((c as u8 - 97).into());
        }
        widths.insert(row.len());
        data.push(row);
    }

    if widths.len() > 1 {
        return Err(anyhow!("Multiple row widths detected"));
    }

    let width = widths.into_iter().collect::<Vec<usize>>()[0];

    Ok(Map {
        matrix: Matrix {
            data,
            width,
            height,
        },
        start: start.ok_or_else(|| anyhow!("No start found"))?,
        end: end.ok_or_else(|| anyhow!("No end found"))?,
    })
}

fn dijkstras(map: &Map, start: Point, end: &Point) -> Result<Option<usize>> {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start.clone());

    let mut scores = HashMap::new();
    scores.insert(start, 0);

    let mut visited = HashSet::new();

    while let Some(point) = to_visit.pop_front() {
        if visited.contains(&point) {
            continue;
        }

        visited.insert(point.clone());

        let value = map
            .matrix
            .get(point.x, point.y)
            .ok_or_else(|| anyhow!("No value found at {} during traversal", point))?;

        for neighbor in map
            .matrix
            .cardinal_neighbor_coordinates_and_values(point.x, point.y)
        {
            if neighbor.value > value && neighbor.value - value > 1 {
                continue;
            }

            scores.insert(
                neighbor.coordinates.clone(),
                std::cmp::min(
                    *scores
                        .get(&neighbor.coordinates)
                        .unwrap_or(&std::usize::MAX),
                    scores[&point] + 1,
                ),
            );
            to_visit.push_back(neighbor.coordinates);
        }
    }

    Ok(scores.get(end).copied())
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let map = read_map(&args.filename)?;

    println!("{}", dijkstras(&map, map.start.clone(), &map.end)?.unwrap());

    let mut possible = Vec::new();
    for i in 0..map.matrix.height {
        for j in 0..map.matrix.width {
            if let Some(0) = map.matrix.get(j, i) {
                possible.push(Point::new(j, i));
            }
        }
    }

    let mut min_score = std::usize::MAX;
    for start in possible {
        if let Some(length) = dijkstras(&map, start, &map.end)? {
            min_score = std::cmp::min(min_score, length);
        }
    }

    println!("{}", min_score);

    Ok(())
}
