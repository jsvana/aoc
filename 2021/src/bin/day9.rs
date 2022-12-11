use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use aoc_2021::{read_matrix, Args, Matrix, Point};
use structopt::StructOpt;

fn flood_fill(
    matrix: &Matrix<usize>,
    basin_id: usize,
    point: Point,
    basin_ids: &mut HashMap<Point, usize>,
) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(point);

    while let Some(next_point) = to_visit.pop_front() {
        if basin_ids.contains_key(&next_point) {
            continue;
        }

        let value = match matrix.get(next_point.x, next_point.y) {
            Some(value) => value,
            None => {
                continue;
            }
        };

        if value == 9 {
            continue;
        }

        for neighbor in matrix.cardinal_neighbor_coordinates(next_point.x, next_point.y) {
            to_visit.push_back(neighbor);
        }

        basin_ids.insert(next_point, basin_id);
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let matrix: Matrix<usize> = read_matrix(&args.filename)?;

    let mut risk_level = 0;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            let value = matrix
                .get(j, i)
                .ok_or_else(|| anyhow!("No value found in matrix at ({}, {})", j, i))?;

            let mut all_lower = true;
            for neighbor in matrix.cardinal_neighbors(j, i) {
                if value >= neighbor {
                    all_lower = false;
                }
            }

            if all_lower {
                risk_level += value + 1;
            }
        }
    }

    println!("{}", risk_level);

    let mut basin_ids = HashMap::new();
    let mut current_basin_id = 0;

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            let point = Point::new(j, i);
            if let Some(9) = matrix.get(j, i) {
                continue;
            }

            if !basin_ids.contains_key(&point) {
                flood_fill(&matrix, current_basin_id, point, &mut basin_ids);
                current_basin_id += 1;
            }
        }
    }

    /*
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            let id = basin_ids
                .get(&Point::new(j, i))
                .map(|i| i + 65)
                .unwrap_or(64) as u32;
            print!("{}", char::from_u32(id).unwrap());
        }
        println!("");
    }
    */

    let mut basin_sizes = HashMap::new();
    for id in basin_ids.into_values() {
        *basin_sizes.entry(id).or_insert(0) += 1;
    }

    let mut sorted_sizes: Vec<usize> = basin_sizes.into_values().collect();
    sorted_sizes.sort_by(|a, b| b.cmp(a));

    println!("{}", sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2]);

    Ok(())
}
