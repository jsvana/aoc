use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use aoc_2021::Args;
use lazy_static::lazy_static;
use maplit::hashmap;
use structopt::StructOpt;

lazy_static! {
    static ref TRIVIAL: HashMap<usize, usize> = hashmap! {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
    };
}

fn translate(key: &HashMap<char, usize>, part: &str) -> HashSet<usize> {
    let mut output = HashSet::new();
    for c in part.chars() {
        output.insert(key[&c]);
    }
    output
}

fn digit(segments: HashSet<usize>) -> Result<usize> {
    let mut sorted_digits: Vec<usize> = segments.into_iter().collect();
    sorted_digits.sort();
    match &sorted_digits[..] {
        &[0, 1, 2, 4, 5, 6] => Ok(0),
        &[2, 5] => Ok(1),
        &[0, 2, 3, 4, 6] => Ok(2),
        &[0, 2, 3, 5, 6] => Ok(3),
        &[1, 2, 3, 5] => Ok(4),
        &[0, 1, 3, 5, 6] => Ok(5),
        &[0, 1, 3, 4, 5, 6] => Ok(6),
        &[0, 2, 5] => Ok(7),
        &[0, 1, 2, 3, 4, 5, 6] => Ok(8),
        &[0, 1, 2, 3, 5, 6] => Ok(9),
        _ => Err(anyhow!("Unknown digit")),
    }
}

fn solve_digits(input_parts: Vec<&str>, output_parts: Vec<&str>) -> Result<usize> {
    let mut simple_digits = HashMap::new();
    let mut input_part_counts = HashMap::new();
    for part in input_parts {
        match TRIVIAL.get(&part.len()) {
            Some(digit) => {
                simple_digits.insert(digit, part.chars().collect::<HashSet<char>>());
            }
            None => {
                let mut sorted_chars: Vec<char> = part.chars().collect();
                sorted_chars.sort();

                input_part_counts
                    .entry(part.len())
                    .or_insert_with(HashSet::new)
                    .insert(sorted_chars.into_iter().collect::<String>());
            }
        }
    }

    let top: HashSet<char> = simple_digits[&7]
        .difference(&simple_digits[&1])
        .copied()
        .collect();

    let bottom = {
        let mut ret: Option<HashSet<char>> = None;
        for part in input_part_counts[&6].iter() {
            let possible: HashSet<char> = part
                .chars()
                .collect::<HashSet<char>>()
                .difference(&simple_digits[&4])
                .copied()
                .collect::<HashSet<char>>()
                .difference(&top)
                .copied()
                .collect();
            if possible.len() == 1 {
                ret = Some(possible);
                break;
            }
        }
        ret.ok_or_else(|| anyhow!("No bottom wire found"))?
    };

    let (middle, bottom_left) = {
        let mut middle_ret: Option<HashSet<char>> = None;
        let mut bottom_left_ret: Option<HashSet<char>> = None;

        for part in input_part_counts[&5].iter() {
            let subtracted: HashSet<char> = part
                .chars()
                .collect::<HashSet<char>>()
                .difference(&top)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&bottom)
                .copied()
                .collect();
            match middle_ret.as_mut() {
                Some(middle) => {
                    middle_ret = Some(
                        middle
                            .intersection(&subtracted)
                            .copied()
                            .collect::<HashSet<char>>(),
                    );
                }
                None => {
                    middle_ret = Some(subtracted.clone());
                }
            }

            let possible: HashSet<char> =
                subtracted.difference(&simple_digits[&4]).copied().collect();
            if possible.len() == 1 {
                bottom_left_ret = Some(possible);
            }
        }

        (
            middle_ret.ok_or_else(|| anyhow!("No middle wire found"))?,
            bottom_left_ret.ok_or_else(|| anyhow!("No bottom left wire found"))?,
        )
    };

    let top_left = {
        let mut ret: Option<HashSet<char>> = None;
        for part in input_part_counts[&5].iter() {
            let possible: HashSet<char> = part
                .chars()
                .collect::<HashSet<char>>()
                .difference(&top)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&bottom)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&middle)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&simple_digits[&1])
                .copied()
                .collect::<HashSet<char>>()
                .difference(&bottom_left)
                .copied()
                .collect();
            if possible.len() == 1 {
                ret = Some(possible);
                break;
            }
        }

        ret.ok_or_else(|| anyhow!("No top left wire found"))?
    };

    let bottom_right = {
        let mut ret: Option<HashSet<char>> = None;
        for part in input_part_counts[&6].iter() {
            let possible: HashSet<char> = part
                .chars()
                .collect::<HashSet<char>>()
                .difference(&top)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&middle)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&bottom)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&top_left)
                .copied()
                .collect::<HashSet<char>>()
                .difference(&bottom_left)
                .copied()
                .collect();
            if possible.len() == 1 {
                ret = Some(possible);
                break;
            }
        }

        ret.ok_or_else(|| anyhow!("No bottom right wire found"))?
    };

    let top_right: HashSet<char> = simple_digits[&1]
        .difference(&bottom_right)
        .copied()
        .collect();

    let key: HashMap<char, usize> = hashmap! {
        top.into_iter().collect::<Vec<char>>()[0] => 0,
        top_left.into_iter().collect::<Vec<char>>()[0] => 1,
        top_right.into_iter().collect::<Vec<char>>()[0] => 2,
        middle.into_iter().collect::<Vec<char>>()[0] => 3,
        bottom_left.into_iter().collect::<Vec<char>>()[0] => 4,
        bottom_right.into_iter().collect::<Vec<char>>()[0] => 5,
        bottom.into_iter().collect::<Vec<char>>()[0] => 6,
    };

    let mut output = 0;
    for (i, part) in output_parts.iter().enumerate() {
        let output_digit = digit(translate(&key, part))?;
        output += output_digit * (10_u32.pow(3 - (i as u32)) as usize);
    }

    Ok(output)
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut count = 0;
    let mut total = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let parts: Vec<&str> = line.split(" | ").collect();
        let left = parts
            .get(0)
            .ok_or_else(|| anyhow!("Missing left side of line \"{}\"", line))?;
        let right = parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing right side of line \"{}\"", line))?;

        total += solve_digits(
            left.split_whitespace().collect::<Vec<&str>>(),
            right.split_whitespace().collect::<Vec<&str>>(),
        )?;

        for part in right.split_whitespace().collect::<Vec<&str>>() {
            if TRIVIAL.contains_key(&part.len()) {
                count += 1;
            }
        }
    }

    println!("{}", count);
    println!("{}", total);

    Ok(())
}
