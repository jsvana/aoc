use std::str::FromStr;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    filename: String,
}

fn read_numbers<T>(filename: &str) -> Result<Vec<T>>
where T: FromStr, <T as FromStr>::Err: 'static + std::error::Error + Send + Sync {
    let contents = std::fs::read_to_string(filename)?;

    let mut numbers = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        numbers.push(line.parse()?);
    }

    Ok(numbers)
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let numbers: Vec<i32> = read_numbers(&args.filename)?;

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
