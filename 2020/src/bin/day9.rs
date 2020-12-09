use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::{read_lines, Args};

fn first_bad_number(numbers: &[i64], preamble_size: usize) -> Option<i64> {
    for i in 0..numbers.len() - preamble_size {
        let number = numbers[i + preamble_size];

        let mut found = false;
        for j in i..preamble_size + i {
            if found {
                break;
            }

            for k in i..preamble_size + i {
                if j == k {
                    continue;
                }
                //println!("{}: checking {} and {}", number, numbers[j], numbers[k]);

                if number == numbers[j] + numbers[k] {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Some(number);
        }
    }

    None
}

fn find_sequence_sum(numbers: &[i64], target: i64) -> Option<i64> {
    for i in 0..numbers.len() {
        let mut running_numbers = Vec::new();
        let mut running_sum = 0;

        for j in i..numbers.len() {
            running_numbers.push(numbers[j]);
            running_sum += numbers[j];

            if running_sum == target {
                if running_numbers.len() == 1 {
                    running_numbers = Vec::new();
                    continue;
                }

                return running_numbers.iter().min().and_then(|min_num| {
                    running_numbers
                        .iter()
                        .max()
                        .map(|max_num| min_num + max_num)
                });
            }

            if running_sum > target {
                running_numbers = Vec::new();
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let numbers: Vec<i64> = read_lines(&args.filename)?;

    let bad_number =
        first_bad_number(&numbers, 25).ok_or_else(|| format_err!("no bad number found"))?;

    println!("Part 1: {}", bad_number);

    let sum = find_sequence_sum(&numbers, bad_number).ok_or_else(|| format_err!("no sum found"))?;

    println!("Part 2: {}", sum);

    Ok(())
}
