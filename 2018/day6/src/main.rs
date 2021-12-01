use std::fs::read_to_string;
use std::collections::{VecDeque, BTreeMap, BTreeSet};

use anyhow::{format_err, Result};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Owner {
    nearest_point: usize,
    distance_to_nearest: i32,
}

fn read_input(filename: &str) -> Result<Vec<Point>> {
    let contents = read_to_string(filename)?;

    let mut points = Vec::new();
    for line in contents.split("\n").collect::<Vec<&str>>() {
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(", ").collect();
        points.push(Point {
            x: parts.get(0).ok_or_else(|| format_err!("Missing first coordinate"))?.parse()?,
            y: parts.get(1).ok_or_else(|| format_err!("Missing second coordinate"))?.parse()?,
        });
    }

    Ok(points)
}

fn get_extents(points: &[Point]) -> Result<(Point, Point)> {
    let min_x = points.iter().map(|p| p.x).min().ok_or_else(|| format_err!("No points found"))?;
    let max_x = points.iter().map(|p| p.x).max().ok_or_else(|| format_err!("No points found"))?;
    let min_y = points.iter().map(|p| p.y).min().ok_or_else(|| format_err!("No points found"))?;
    let max_y = points.iter().map(|p| p.y).max().ok_or_else(|| format_err!("No points found"))?;

    Ok((Point{x: min_x, y: min_y}, Point{x: max_x, y: max_y}))
}

fn flood_point(mut map: BTreeMap<Point, Vec<Owner>>, point: Point, point_id: usize, min_point: &Point, max_point: &Point) -> Result<BTreeMap<Point, Vec<Owner>>> {
    let mut points = VecDeque::new();
    points.push_back((point, 0));

    let mut visited: BTreeSet<Point> = BTreeSet::new();

    while !points.is_empty() {
        let (next_point, distance) = points.pop_front().ok_or_else(|| format_err!("No points to pop"))?;
        if visited.contains(&next_point) {
            continue;
        }

        visited.insert(next_point.clone());

        if next_point.x < min_point.x || next_point.x > max_point.x || next_point.y < min_point.y || next_point.y > max_point.y {
            continue;
        }

        map.entry(next_point.clone()).or_insert(Vec::new()).push(Owner { nearest_point: point_id, distance_to_nearest: distance });
        /*
        match map.get(&next_point) {
            Some(other_owner) => {
                if distance <= other_owner.distance_to_nearest {
                    map.entry(&next_point).or_insert(Vec::new()).push(Owner { nearest_point: point_id, distance_to_nearest: distance });
                }
            }
            None => {
                map.insert(next_point.clone(), Owner { nearest_point: point_id, distance_to_nearest: distance });
            }
        }
        */

        points.push_back((Point{x: next_point.x - 1, y: next_point.y}, distance + 1));
        points.push_back((Point{x: next_point.x + 1, y: next_point.y}, distance + 1));
        points.push_back((Point{x: next_point.x, y: next_point.y - 1}, distance + 1));
        points.push_back((Point{x: next_point.x, y: next_point.y + 1}, distance + 1));
    }

    Ok(map)
}

fn fill(min_point: &Point, max_point: &Point, points: Vec<Point>) -> Result<BTreeMap<Point, Vec<Owner>>> {
    let mut map = BTreeMap::new();

    for (i, point) in points.into_iter().enumerate() {
        map = flood_point(map, point, i, &min_point, &max_point)?;
    }

    Ok(map)
}

fn main() -> Result<()> {
    let points = read_input("input.txt")?;

    let (min_point, max_point) = get_extents(&points)?;

    println!("pre fill");
    let map = fill(&min_point, &max_point, points)?;
    println!("post fill");

    let names = vec!["A", "B", "C", "D", "E", "F", "G", "H"];

    let mut counts = BTreeMap::new();
    let mut lines = Vec::new();
    for y in min_point.y..max_point.y+1 {
        let mut line = "".to_string();
        for x in min_point.x..max_point.x+1 {
            match map.get(&Point{x, y}) {
                Some(owners) => {
                    if owners.is_empty() {
                        continue;
                    }

                    let mut nearest_point = usize::MAX;
                    let mut nearest_distance = i32::MAX;

                    for owner in owners.iter() {
                        if owner.distance_to_nearest < nearest_distance {
                            nearest_point = owner.nearest_point;
                            nearest_distance = owner.distance_to_nearest;
                        }
                    }

                    if nearest_point == usize::MAX {
                        continue;
                    }

                    *counts.entry(nearest_point).or_insert(0) += 1;

                    let letter = names[nearest_point];
                    if nearest_distance == 0 {
                        line += &format!("{}", letter);
                    } else {
                        line += &format!("{}", letter.to_lowercase());
                    }
                }
                None => {
                    line += ".";
                }
            }
        }
        lines.push(line);
    }

    for line in lines.into_iter() {
        println!("{}", line);
    }

    dbg!(counts);

    Ok(())
}
