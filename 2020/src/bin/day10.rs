use std::collections::{BTreeMap, BTreeSet, VecDeque};

use anyhow::{format_err, Result};
use structopt::StructOpt;

use aoc_2020::{read_lines, Args};

fn part1(adapters: &[i64]) -> i64 {
    let mut differences = BTreeMap::new();

    let mut current_jolt = 0;
    for adapter in adapters.iter() {
        let difference = *adapter - current_jolt;
        if difference < 0 || difference > 3 {
            break;
        }

        *differences.entry(difference).or_insert(0) += 1;
        current_jolt = *adapter;
    }

    differences[&3] * differences[&1]
}

fn make_graph(adapters: &[i64], target: i64) -> Result<BTreeMap<i64, BTreeSet<i64>>> {
    let mut graph = BTreeMap::new();

    let mut adapters_set: BTreeSet<i64> = adapters.iter().cloned().collect();
    adapters_set.insert(target);

    let mut to_find = VecDeque::new();
    to_find.push_back(0);
    let mut visited = BTreeSet::new();

    while !to_find.is_empty() {
        let start = to_find
            .pop_front()
            .ok_or_else(|| format_err!("somehow got an empty find queue"))?;

        if visited.contains(&start) {
            continue;
        }

        visited.insert(start);

        for i in 1..=3 {
            let possible = start + i;
            if adapters_set.contains(&possible) {
                graph
                    .entry(start)
                    .or_insert(BTreeSet::new())
                    .insert(possible);
                to_find.push_back(possible);
            }
        }
    }

    Ok(graph)
}

fn count_paths(
    graph: &BTreeMap<i64, BTreeSet<i64>>,
    memo: &mut BTreeMap<i64, i64>,
    start: i64,
    target: i64,
) -> i64 {
    if start == target {
        return 1;
    }

    let mut total = 0;
    for child in graph[&start].iter() {
        total += match memo.get(&child) {
            Some(value) => *value,
            None => {
                let value = count_paths(graph, memo, *child, target);
                memo.insert(*child, value);
                value
            }
        };
    }

    total
}

fn part2(adapters: &[i64]) -> Result<i64> {
    let target = adapters
        .last()
        .ok_or_else(|| format_err!("must have at least one adapter"))?;

    let graph = make_graph(adapters, *target)?;
    let mut memo = BTreeMap::new();

    Ok(count_paths(&graph, &mut memo, 0, *target))
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut adapters: Vec<i64> = read_lines(&args.filename)?;

    adapters.sort();

    let highest_adapter = adapters
        .last()
        .ok_or_else(|| format_err!("must have at least one adapter"))?
        + 3;

    adapters.push(highest_adapter);

    println!("Part 1: {}", part1(&adapters));
    println!("Part 2: {}", part2(&adapters)?);

    Ok(())
}
