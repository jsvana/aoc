use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use structopt::StructOpt;

use aoc_2021::{read_lines, Args};

fn generate_ratings(lines: &[String]) -> (Vec<char>, Vec<char>) {
    let mut oxygen_generator_rating = Vec::new();
    let mut co2_scrubber_rating = Vec::new();

    let digit_counts = digit_frequencies(lines);

    for (position, chars) in digit_counts.iter() {
        if chars.get(&'0').unwrap_or(&0) > chars.get(&'1').unwrap_or(&0) {
            oxygen_generator_rating.push('0');
            co2_scrubber_rating.push('1');
        } else {
            oxygen_generator_rating.push('1');
            co2_scrubber_rating.push('0');
        }
    }

    (oxygen_generator_rating, co2_scrubber_rating)
}

fn part2(lines: Vec<String>) -> Result<()> {
    let mut oxygen_lines = lines.clone();

    let mut index = 0;
    while oxygen_lines.len() > 1 {
        let (oxygen_generator_rating, _) = generate_ratings(&oxygen_lines);

        let c = oxygen_generator_rating[index];

        let mut new_lines = Vec::new();
        for line in oxygen_lines.iter() {
            if line.chars().nth(index).unwrap() == c {
                new_lines.push(line.to_string());
            }
        }
        oxygen_lines = new_lines;

        index += 1;
    }

    let oxygen_rating = isize::from_str_radix(&oxygen_lines[0], 2)?;

    let mut co2_lines = lines.clone();

    let mut index = 0;
    while co2_lines.len() > 1 {
        let (_, co2_scrubber_rating) = generate_ratings(&co2_lines);

        let c = co2_scrubber_rating[index];

        let mut new_lines = Vec::new();
        for line in co2_lines.iter() {
            if line.chars().nth(index).unwrap() == c {
                new_lines.push(line.to_string());
            }
        }
        co2_lines = new_lines;

        index += 1;
    }

    let co2_rating = isize::from_str_radix(&co2_lines[0], 2)?;

    println!("{}", oxygen_rating * co2_rating);

    Ok(())
}

fn digit_frequencies(lines: &[String]) -> BTreeMap<usize, BTreeMap<char, usize>> {
    let mut digit_counts = BTreeMap::new();
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            *digit_counts
                .entry(i)
                .or_insert_with(BTreeMap::new)
                .entry(c)
                .or_insert(0) += 1;
        }
    }
    digit_counts
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let lines: Vec<String> = read_lines(&args.filename)?;

    let digit_counts = digit_frequencies(&lines);

    let mut most_frequent = Vec::new();
    let mut least_frequent = Vec::new();
    for (position, chars) in digit_counts.iter() {
        if chars[&'0'] > chars[&'1'] {
            most_frequent.push('0');
            least_frequent.push('1');
        } else {
            most_frequent.push('1');
            least_frequent.push('0');
        }
    }

    let gamma = isize::from_str_radix(&most_frequent.into_iter().collect::<String>(), 2)?;
    let epsilon = isize::from_str_radix(&least_frequent.into_iter().collect::<String>(), 2)?;

    println!("{}", gamma * epsilon);

    part2(lines)
}
