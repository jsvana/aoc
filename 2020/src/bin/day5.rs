use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::Args;

#[derive(Debug)]
enum Movement {
    Back,
    Front,
    Left,
    Right,
}

impl Movement {
    fn list_from_str(value: &str) -> Result<Vec<Movement>> {
        let mut movements = Vec::new();

        for c in value.chars() {
            movements.push(match c {
                'B' => Movement::Back,
                'F' => Movement::Front,
                'L' => Movement::Left,
                'R' => Movement::Right,
                _ => {
                    return Err(format_err!("unknown char {}", c));
                }
            });
        }

        Ok(movements)
    }
}

fn find_seat_id(movements: &[Movement]) -> usize {
    let mut row_low = 0;
    let mut row_high = 128;
    let mut column_low = 0;
    let mut column_high = 8;

    for movement in movements.iter() {
        match movement {
            Movement::Front => {
                row_high = (row_low + row_high) / 2;
            }
            Movement::Back => {
                row_low = (row_low + row_high) / 2;
            }
            Movement::Right => {
                column_low = (column_low + column_high) / 2;
            }
            Movement::Left => {
                column_high = (column_low + column_high) / 2;
            }
        }
    }

    row_low * 8 + column_low
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let contents = std::fs::read_to_string(args.filename)?;

    let mut filled = Vec::with_capacity(1024);
    Vec::resize_with(&mut filled, 1024, || false);

    for line in contents.split("\n") {
        let seat_id = find_seat_id(&Movement::list_from_str(line)?);
        filled[seat_id] = true;
    }

    for i in 0..1024 {
        let current_exists = filled[i];

        let previous_exists = if i > 0 {
            filled[i - 1]
        } else {
            true
        };

        let next_exists = if i < 1023 {
            filled[i + 1]
        } else {
            true
        };

        if !current_exists && previous_exists && next_exists {
            println!("{}", i);
        }
    }

    Ok(())
}
