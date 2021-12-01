use anyhow::Result;
use structopt::StructOpt;

use aoc_2021::{read_lines, Args};

fn part2(numbers: Vec<i32>) -> Result<()> {
    let mut windows = Vec::new();
    for (i, number) in numbers.iter().enumerate() {
        if i >= numbers.len() - 2 {
            break;
        }

        windows.push(vec![*number, numbers[i + 1], numbers[i + 2]]);
    }

    let mut count = 0;
    let mut previous: Option<i32> = None;
    for window in windows {
        let sum = window[0] + window[1] + window[2];
        if let Some(prev) = previous {
            if prev < sum {
                count += 1;
            }
        }
        previous = Some(sum);
    }

    println!("{}", count);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let numbers: Vec<i32> = read_lines(&args.filename)?;

    let mut count = 0;
    let mut previous: Option<i32> = None;
    for number in numbers.iter() {
        if let Some(prev) = previous {
            if prev < *number {
                count += 1;
            }
        }
        previous = Some(*number);
    }

    println!("{}", count);

    part2(numbers);

    Ok(())
}
