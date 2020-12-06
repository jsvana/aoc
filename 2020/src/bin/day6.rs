use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use structopt::StructOpt;

use aoc_2020::Args;

#[derive(Debug)]
struct Group {
    member_count: usize,
    questions: BTreeMap<char, usize>,
}

impl Group {
    fn new() -> Self {
        Self {
            member_count: 0,
            questions: BTreeMap::new(),
        }
    }

    fn add_questions(&mut self, questions: BTreeSet<char>) {
        self.member_count += 1;

        for question in questions.into_iter() {
            *self.questions.entry(question).or_insert(0) += 1;
        }
    }

    fn question_count(&self) -> usize {
        self.questions.len()
    }

    fn all_question_count(&self) -> usize {
        let mut total = 0;
        for answer_count in self.questions.values() {
            if *answer_count == self.member_count {
                total += 1;
            }
        }

        total
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let contents = std::fs::read_to_string(args.filename)?;

    let mut groups = Vec::new();
    let mut current_group = Group::new();

    for line in contents.split("\n") {
        let line = line.trim();

        if line.is_empty() {
            groups.push(current_group);
            current_group = Group::new();

            continue;
        }

        current_group.add_questions(line.chars().collect());
    }

    groups.push(current_group);

    let mut total = 0;
    for group in groups.into_iter() {
        total += group.all_question_count();
    }

    println!("{}", total);

    Ok(())
}
