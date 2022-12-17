use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    pub filename: String,
}

pub fn read_lines<T>(filename: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + Error + Send + Sync,
{
    let contents = std::fs::read_to_string(filename)?;

    let mut values = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        values.push(line.parse()?);
    }

    Ok(values)
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Matrix<T: Copy> {
    data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.data.get(y).and_then(|row| row.get(x).copied())
    }

    pub fn cardinal_neighbor_coordinates(&self, x: usize, y: usize) -> Vec<Point> {
        let mut coordinates = Vec::new();

        if x > 0 {
            coordinates.push(Point::new(x - 1, y));
        }

        if x < self.width - 1 {
            coordinates.push(Point::new(x + 1, y));
        }

        if y > 0 {
            coordinates.push(Point::new(x, y - 1));
        }

        if y < self.height - 1 {
            coordinates.push(Point::new(x, y + 1));
        }

        coordinates
    }

    pub fn cardinal_neighbors(&self, x: usize, y: usize) -> Vec<T> {
        let mut neighbors = Vec::new();

        for coordinates in self.cardinal_neighbor_coordinates(x, y) {
            if let Some(value) = self.get(coordinates.x, coordinates.y) {
                neighbors.push(value);
            }
        }

        neighbors
    }

    pub fn all_neighbor_coordinates(&self, x: usize, y: usize) -> Vec<Point> {
        let mut coordinates = Vec::new();

        for i in y - 1..=y + 1 {
            for j in x - 1..=x + 1 {
                if i == j {
                    continue;
                }

                coordinates.push(Point::new(j, i));
            }
        }

        coordinates
    }

    pub fn all_neighbors(&self, x: usize, y: usize) -> Vec<T> {
        let mut neighbors = Vec::new();

        for coordinates in self.all_neighbor_coordinates(x, y) {
            if let Some(value) = self.get(coordinates.x, coordinates.y) {
                neighbors.push(value);
            }
        }

        neighbors
    }
}

pub fn read_matrix<T>(filename: &str) -> Result<Matrix<T>>
where
    T: TryFrom<u32> + Copy,
    <T as TryFrom<u32>>::Error: 'static + Send + Sync + Error,
{
    let contents = std::fs::read_to_string(filename)?;

    let mut data: Vec<Vec<T>> = Vec::new();
    let mut widths = HashSet::new();
    let mut height = 0;

    for line in contents.lines() {
        height += 1;

        let mut row: Vec<T> = Vec::new();
        for c in line.chars() {
            row.push(T::try_from(
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid digit {}", c))?,
            )?);
        }
        widths.insert(row.len());
        data.push(row);
    }

    if widths.len() > 1 {
        return Err(anyhow!("Multiple row widths detected"));
    }

    let width = widths.into_iter().collect::<Vec<usize>>()[0];

    Ok(Matrix {
        data,
        width,
        height,
    })
}
