use std::collections::{BTreeMap, BTreeSet, VecDeque};

use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::Args;

type BagMap = BTreeMap<String, Vec<BagCount>>;

fn name_from_string(input: &str) -> Result<BagCount> {
    let parts: Vec<&str> = input.split(" ").collect();
    match parts.len() {
        3 => {
            let name = format!(
                "{} {}",
                parts
                    .get(0)
                    .ok_or_else(|| format_err!("\"{}\" missing adjective"))?,
                parts
                    .get(1)
                    .ok_or_else(|| format_err!("\"{}\" missing color"))?
            );

            Ok(BagCount { count: 1, name })
        }
        4 => {
            let count: usize = parts
                .get(0)
                .ok_or_else(|| format_err!("\"{}\" missing count"))?
                .parse()?;

            let name = format!(
                "{} {}",
                parts
                    .get(1)
                    .ok_or_else(|| format_err!("\"{}\" missing adjective"))?,
                parts
                    .get(2)
                    .ok_or_else(|| format_err!("\"{}\" missing color"))?
            );

            Ok(BagCount { count, name })
        }
        _ => Err(format_err!("bad input \"{}\"", input)),
    }
}

#[derive(Debug)]
struct BagCount {
    count: usize,
    name: String,
}

fn build_map(filename: String) -> Result<BagMap> {
    let contents = std::fs::read_to_string(filename)?;

    let mut bags = BTreeMap::new();
    for line in contents.split("\n") {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" contain ").collect();

        let outer = name_from_string(
            parts
                .get(0)
                .ok_or_else(|| format_err!("\"{}\" missing outer bag name", line))?,
        )?;

        let inner_parts: Vec<&str> = parts
            .get(1)
            .ok_or_else(|| format_err!("\"{}\" missing inner bag names", line))?
            .split(", ")
            .collect();

        let mut inner = Vec::new();
        for inner_part in inner_parts.iter() {
            inner.push(name_from_string(inner_part)?);
        }

        bags.insert(outer.name, inner);
    }

    Ok(bags)
}

fn chain_contains_gold(bags: &BagMap, start: &str) -> Result<bool> {
    let mut to_visit = VecDeque::new();

    to_visit.push_back(start.to_string());

    let mut visited = BTreeSet::new();

    while !to_visit.is_empty() {
        let next = to_visit
            .pop_back()
            .ok_or_else(|| format_err!("no more items to pop despite stack being non-empty"))?;

        if visited.contains(&next) {
            continue;
        }

        visited.insert(next.clone());

        if let Some(counts) = bags.get(&next) {
            for count in counts.iter() {
                if count.name == "shiny gold" {
                    return Ok(true);
                }

                to_visit.push_back(count.name.clone());
            }
        }
    }

    Ok(false)
}

fn count_chains_containing_gold(bags: &BagMap) -> Result<usize> {
    let mut count = 0;
    for name in bags.keys() {
        if chain_contains_gold(bags, name)? {
            count += 1;
        }
    }

    Ok(count)
}

fn sum_children(bags: &BagMap, start: &str) -> usize {
    match bags.get(start) {
        Some(counts) => {
            let mut sum = 1;
            for count in counts.iter() {
                sum += count.count * sum_children(bags, &count.name);
            }
            sum
        }
        None => 0,
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let bags = build_map(args.filename)?;

    println!("Part 1: {}", count_chains_containing_gold(&bags)?);
    println!("Part 2: {}", sum_children(&bags, "shiny gold") - 1);

    Ok(())
}
