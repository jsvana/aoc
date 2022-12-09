use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

fn character_priority(c: char) -> Result<usize> {
    if !c.is_ascii_alphabetic() {
        return Err(anyhow!("Unexpected character '{}'", c));
    }

    if c.is_ascii_lowercase() {
        Ok((c as usize) - 96)
    } else {
        Ok((c as usize) - 38)
    }
}

fn line_priority(line: &str) -> Result<usize> {
    if line.len() % 2 != 0 {
        return Err(anyhow!("Invalid line--odd length"));
    }

    let container_size = line.len() / 2;

    let first_container: HashSet<char> = line.chars().take(container_size).collect();
    let second_container: HashSet<char> = line
        .chars()
        .skip(container_size)
        .take(container_size)
        .collect();

    let mut overlap: Vec<char> = first_container
        .intersection(&second_container)
        .map(|c| *c)
        .collect();

    if overlap.len() > 1 {
        return Err(anyhow!("More than one character overlap found"));
    }

    let overlap_char = overlap.pop().ok_or_else(|| anyhow!("No overlap found"))?;

    character_priority(overlap_char)
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut score = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        score += line_priority(line)?;
    }

    println!("{}", score);

    let mut score = 0;
    for lines in std::fs::read_to_string(&args.filename)?
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
    {
        let first_container: HashSet<char> = lines[0].chars().collect();
        let second_container: HashSet<char> = lines[1].chars().collect();
        let third_container: HashSet<char> = lines[2].chars().collect();

        let mut overlap: Vec<char> = first_container
            .intersection(&second_container)
            .map(|c| *c)
            .collect::<HashSet<char>>()
            .intersection(&third_container)
            .map(|c| *c)
            .collect();

        if overlap.len() > 1 {
            return Err(anyhow!("More than one character overlap found"));
        }

        let overlap_char = overlap.pop().ok_or_else(|| anyhow!("No overlap found"))?;

        score += character_priority(overlap_char)?;
    }

    println!("{}", score);

    Ok(())
}
