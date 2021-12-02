use anyhow::{anyhow, Result};
use structopt::StructOpt;

use aoc_2021::{read_lines, Args};

fn part2(instructions: Vec<String>) -> Result<()> {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let direction = parts[0];
        let magnitude: usize = parts[1].parse()?;

        match direction {
            "forward" => {
                x += magnitude;
                y += magnitude * aim;
            }
            "up" => {
                aim -= magnitude;
            }
            "down" => {
                aim += magnitude;
            }
            _ => {
                return Err(anyhow!("lalala"));
            }
        }
    }

    println!("{}", x * y);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let instructions: Vec<String> = read_lines(&args.filename)?;

    let mut x = 0;
    let mut y = 0;

    for instruction in instructions.iter() {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let direction = parts[0];
        let magnitude: usize = parts[1].parse()?;

        match direction {
            "forward" => {
                x += magnitude;
            }
            "up" => {
                y -= magnitude;
            }
            "down" => {
                y += magnitude;
            }
            _ => {
                return Err(anyhow!("lalala"));
            }
        }
    }

    println!("{}", x * y);

    part2(instructions)
}
