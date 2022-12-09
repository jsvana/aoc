use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

struct Pair {
    start: usize,
    end: usize,
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let parts: Vec<&str> = value.split('-').collect();
        let start: usize = parts
            .get(0)
            .ok_or_else(|| anyhow!("Missing start of range in \"{}\"", value))?
            .parse()?;
        let end: usize = parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing end of range in \"{}\"", value))?
            .parse()?;
        Ok(Pair { start, end })
    }
}

impl Pair {
    fn contains(&self, other: &Pair) -> bool {
        self.start <= other.start && self.end >= other.end
            || other.start <= self.start && other.end >= self.end
    }

    fn overlaps_with(&self, other: &Pair) -> bool {
        self.start <= other.start && self.end >= other.start
            || other.start <= self.start && other.end >= self.start
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut contains_count = 0;
    let mut overlap_count = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let mut pair_parts: Vec<&str> = line.split(',').collect();
        let first_pair: Pair = pair_parts.remove(0).parse()?;
        let second_pair: Pair = pair_parts.remove(0).parse()?;

        if first_pair.contains(&second_pair) {
            contains_count += 1;
        }

        if first_pair.overlaps_with(&second_pair) {
            overlap_count += 1;
        }
    }

    println!("{}", contains_count);
    println!("{}", overlap_count);

    Ok(())
}
