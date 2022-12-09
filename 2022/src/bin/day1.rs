use anyhow::Result;
use aoc_2022::Args;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut max_calories = 0;
    let mut running_count = 0;

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        if line.is_empty() {
            if running_count > max_calories {
                max_calories = running_count;
            }

            running_count = 0;

            continue;
        }

        let calories: usize = line.parse()?;
        running_count += calories;
    }

    if running_count > max_calories {
        max_calories = running_count;
    }

    println!("{}", max_calories);

    let mut calories = Vec::new();
    let mut running_count = 0;

    for line in std::fs::read_to_string(&args.filename)?.lines() {
        if line.is_empty() {
            calories.push(running_count);
            running_count = 0;

            continue;
        }

        let calories: usize = line.parse()?;
        running_count += calories;
    }

    calories.push(running_count);

    calories.sort();

    println!(
        "{}",
        calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap()
    );

    Ok(())
}
