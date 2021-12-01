use std::cmp::max;

use anyhow::Result;

fn fuel_requirement(mass: i64) -> i64 {
    if mass <= 0 {
        return 0;
    }

    let fuel = max(0, mass / 3 - 2);
    fuel + fuel_requirement(fuel)
}

fn read_input(filename: &str) -> Result<Vec<i64>> {
    let file_str = std::fs::read_to_string(filename)?;

    let mut numbers = Vec::new();
    for line in file_str.split("\n").filter(|l| l.len() > 0) {
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn main() -> Result<()> {
    let numbers = read_input("input.txt")?;

    let mut total = 0;
    for number in numbers.iter() {
        total += fuel_requirement(*number);
    }

    println!("Total: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirement() {
        let test_cases = vec![
            (14, 2),
            (1969, 966),
            (100756, 50346),
        ];

        for (input, expected) in test_cases.iter() {
            assert_eq!(fuel_requirement(*input), *expected);
        }
    }
}
