use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::{read_lines, Args};

/*
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
    }
}
*/

fn main() -> Result<()> {
    let args = Args::from_args();

    //let contents = std::fs::read_to_string(args.filename)?;
    //let numbers: Vec<i32> = read_lines(&args.filename)?;

    Ok(())
}
