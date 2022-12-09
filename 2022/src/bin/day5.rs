use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Debug)]
struct Yard {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Movement {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(' ').collect();

        let quantity: usize = parts
            .get(1)
            .ok_or_else(|| anyhow!("Missing quantity in movement \"{}\"", value))?
            .parse()?;
        let source: usize = parts
            .get(3)
            .ok_or_else(|| anyhow!("Missing source stack in movement \"{}\"", value))?
            .parse()?;
        let destination: usize = parts
            .get(5)
            .ok_or_else(|| anyhow!("Missing destination stack in movement \"{}\"", value))?
            .parse()?;

        Ok(Movement {
            quantity,
            source: source - 1,
            destination: destination - 1,
        })
    }
}

impl Yard {
    fn execute_movement(&mut self, movement: &Movement) -> Result<()> {
        for _ in 0..movement.quantity {
            let value = self.stacks[movement.source].pop().ok_or_else(|| {
                anyhow!(
                    "No remaining values in source stack index {}",
                    movement.source
                )
            })?;
            self.stacks[movement.destination].push(value);
        }

        Ok(())
    }

    fn execute_movement_part2(&mut self, movement: Movement) {
        let new_length = self.stacks[movement.source].len() - movement.quantity;

        let mut section: Vec<char> = self.stacks[movement.source][new_length..]
            .iter()
            .map(|c| *c)
            .collect();

        self.stacks[movement.destination].append(&mut section);
        self.stacks[movement.source].truncate(new_length);
    }

    fn top_values(&self) -> String {
        let mut out = Vec::new();
        for stack in self.stacks.iter() {
            if let Some(c) = stack.last() {
                out.push(c);
            }
        }
        out.into_iter().collect()
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let data = read_to_string(&args.filename)?;

    let mut parsing_movements = false;
    let mut movements = Vec::new();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        if parsing_movements {
            movements.push(line.parse::<Movement>()?);
        } else {
            if line.is_empty() {
                parsing_movements = true;
                continue;
            }

            for (i, c) in line.chars().enumerate() {
                if !c.is_alphabetic() {
                    continue;
                }

                let stack_index = (i - 1) / 4;

                match stacks.get_mut(stack_index) {
                    Some(stack) => {
                        if stack.is_empty() {
                            stack.push(c);
                        } else {
                            stack.insert(0, c);
                        }
                    }
                    None => {
                        if stack_index > stacks.len() {
                            stacks.resize_with(stack_index + 1, Vec::new);
                        }
                        stacks.insert(stack_index, vec![c]);
                    }
                }
            }
        }
    }

    let mut yard = Yard {
        stacks: stacks.clone(),
    };

    for movement in movements.iter() {
        yard.execute_movement(movement)?;
    }

    println!("{}", yard.top_values());

    let mut yard = Yard { stacks };

    for movement in movements {
        yard.execute_movement_part2(movement);
    }

    println!("{}", yard.top_values());

    Ok(())
}
