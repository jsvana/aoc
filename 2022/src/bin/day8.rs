use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

fn is_visible(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || y == matrix.len() - 1 || x == matrix[0].len() - 1 {
        return true;
    }

    let value = matrix[y][x];

    let mut y_above_visible = true;
    for i in 0..y {
        if matrix[i][x] >= value {
            y_above_visible = false;
            break;
        }
    }

    let mut y_below_visible = true;
    for i in y + 1..matrix.len() {
        if matrix[i][x] >= value {
            y_below_visible = false;
            break;
        }
    }

    let mut x_above_visible = true;
    for j in 0..x {
        if matrix[y][j] >= value {
            x_above_visible = false;
            break;
        }
    }

    let mut x_below_visible = true;
    for j in x + 1..matrix[y].len() {
        if matrix[y][j] >= value {
            x_below_visible = false;
            break;
        }
    }

    y_above_visible || y_below_visible || x_above_visible || x_below_visible
}

fn scenic_score(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = matrix[y][x];

    let mut total_y_up = 0;
    for i in 0..y {
        total_y_up += 1;
        if matrix[y - i - 1][x] >= value {
            break;
        }
    }

    let mut total_y_down = 0;
    for i in 0..matrix.len() - y - 1 {
        total_y_down += 1;
        if matrix[y + i + 1][x] >= value {
            break;
        }
    }

    let mut total_x_up = 0;
    for j in 0..x {
        total_x_up += 1;
        if matrix[y][x - j - 1] >= value {
            break;
        }
    }

    let mut total_x_down = 0;
    for j in 0..matrix[y].len() - x - 1 {
        total_x_down += 1;
        if matrix[y][x + j + 1] >= value {
            break;
        }
    }

    total_y_up * total_y_down * total_x_up * total_x_down
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut matrix = Vec::new();
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        matrix.push(
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("Unknown digit '{}'", c))
                })
                .collect::<Result<Vec<u32>>>()?,
        );
    }

    let mut visible = HashMap::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let key = (i, j);
            if visible.contains_key(&key) {
                continue;
            }

            visible.insert(key, is_visible(&matrix, j, i));
        }
    }

    let mut total_visible = 0;
    for tile_is_visible in visible.into_values() {
        if tile_is_visible {
            total_visible += 1;
        }
    }

    println!("{}", total_visible);

    let mut max_score = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            max_score = std::cmp::max(max_score, scenic_score(&matrix, j, i));
        }
    }

    println!("{}", max_score);

    Ok(())
}
