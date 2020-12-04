use anyhow::Result;
use structopt::StructOpt;

use aoc_2020::{Args, read_lines};

fn main() -> Result<()> {
    let args = Args::from_args();

    let numbers: Vec<i32> = read_lines(&args.filename)?;

    for (i, first_number) in numbers.iter().enumerate() {
        for (j, second_number) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }

            for (k, third_number) in numbers.iter().enumerate() {
                if i == k {
                    continue;
                }

                if first_number + second_number + third_number == 2020 {
                    println!("{}", first_number * second_number * third_number);
                    return Ok(())
                }
            }
        }
    }

    Ok(())
}
