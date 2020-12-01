use std::collections::BTreeSet;

use anyhow::Result;

fn read_numbers(filename: &str) -> Result<Vec<i32>> {
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
    // TODO(jsvana): make this a commandline argument
    let numbers = read_numbers("input.txt")?;

    let mut missing = Vec::new();
    for number in numbers.iter() {
        missing.push(2020 - number);
    }

    let all: BTreeSet<_> = numbers.iter().collect();
    for (i, missing_number) in missing.into_iter().enumerate() {
        if all.contains(&missing_number) {
            println!("{}", missing_number * numbers[i]);
            break;
        }
    }

    Ok(())
}
