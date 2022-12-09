use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn round_score(&self, other: &Choice) -> usize {
        let base_score = match (self, other) {
            (Choice::Rock, Choice::Rock) => 3,
            (Choice::Rock, Choice::Paper) => 0,
            (Choice::Rock, Choice::Scissors) => 6,
            (Choice::Paper, Choice::Paper) => 3,
            (Choice::Paper, Choice::Rock) => 6,
            (Choice::Paper, Choice::Scissors) => 0,
            (Choice::Scissors, Choice::Scissors) => 3,
            (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Scissors, Choice::Paper) => 6,
        };

        base_score + self.score()
    }

    fn correct_choice(&self, round_result: &RoundResult) -> Choice {
        match (self, round_result) {
            (Choice::Rock, RoundResult::Lose) => Choice::Scissors,
            (Choice::Rock, RoundResult::Draw) => Choice::Rock,
            (Choice::Rock, RoundResult::Win) => Choice::Paper,
            (Choice::Paper, RoundResult::Lose) => Choice::Rock,
            (Choice::Paper, RoundResult::Draw) => Choice::Paper,
            (Choice::Paper, RoundResult::Win) => Choice::Scissors,
            (Choice::Scissors, RoundResult::Lose) => Choice::Paper,
            (Choice::Scissors, RoundResult::Draw) => Choice::Scissors,
            (Choice::Scissors, RoundResult::Win) => Choice::Rock,
        }
    }
}

impl TryFrom<char> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(Choice::Rock),
            'B' => Ok(Choice::Paper),
            'C' => Ok(Choice::Scissors),
            'X' => Ok(Choice::Rock),
            'Y' => Ok(Choice::Paper),
            'Z' => Ok(Choice::Scissors),
            _ => Err(anyhow!("Unknown choice '{}'", value)),
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for RoundResult {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'X' => Ok(RoundResult::Lose),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err(anyhow!("Unknown round result '{}'", value)),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut total_score = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let mut parts: Vec<char> = line.chars().collect();
        let opponent: Choice = parts.remove(0).try_into()?;
        let you: Choice = parts.remove(1).try_into()?;

        total_score += you.round_score(&opponent);
    }

    println!("{}", total_score);

    let mut total_score = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        let mut parts: Vec<char> = line.chars().collect();
        let opponent: Choice = parts.remove(0).try_into()?;
        let round_result: RoundResult = parts.remove(1).try_into()?;
        let you = opponent.correct_choice(&round_result);

        total_score += you.round_score(&opponent);
    }

    println!("{}", total_score);

    Ok(())
}
