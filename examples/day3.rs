use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::Args;

#[derive(Debug)]
enum Tile {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn from_str(text: &str) -> Result<Self> {
        let mut tiles = Vec::new();

        let mut width: Option<usize> = None;

        let mut line_count = 0;
        for line in text.split("\n") {
            if line.is_empty() {
                continue;
            }

            width = match width {
                Some(width) => {
                    if width != line.len() {
                        return Err(format_err!("widths {} and {} don't match", width, line.len()));
                    }

                    Some(width)
                }
                None => Some(line.len()),
            };

            for tile_char in line.chars() {
                tiles.push(
                    match tile_char {
                        '.' => Tile::Empty,
                        '#' => Tile::Tree,
                        _ => {
                            return Err(format_err!("unknown char {}", tile_char));
                        }
                    }
                );
            }

            line_count += 1;
        }

        Ok(Self {width: width.ok_or_else(|| format_err!("no width found"))?, height: line_count, tiles})
    }

    fn at(&self, x: usize, y: usize) -> Option<&Tile> {
        let x = x % self.width;

        self.tiles.get(y * self.width + x)
    }
}

fn check_movement(map: &Map, x_delta: usize, y_delta: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut running = true;

    let mut tree_count = 0;

    while running {
        match map.at(x, y) {
            Some(Tile::Tree) => {
                tree_count += 1;
                x += x_delta;
                y += y_delta;
            }
            Some(_) => {
                x += x_delta;
                y += y_delta;
            }
            None => {
                running = false;
            }
        }
    }

    tree_count
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let map = Map::from_str(&std::fs::read_to_string(args.filename)?)?;

    let deltas = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut multiple = 1;
    for (x_delta, y_delta) in deltas.into_iter() {
        multiple *= check_movement(&map, x_delta, y_delta);
    }

    println!("{}", multiple);

    Ok(())
}
