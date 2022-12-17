use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::iter::Peekable;
use std::str::{FromStr, Lines};

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    expression: Expression,
    divisor: i64,
    true_path: usize,
    false_path: usize,
    inspections: usize,
}

#[derive(Debug)]
struct InspectionResult {
    new_monkey: usize,
    value: i64,
}

impl Monkey {
    fn has_next_item(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect_next(&mut self) -> Result<InspectionResult> {
        let next_value = self
            .items
            .pop_front()
            .ok_or_else(|| anyhow!("Monkey {} has no items left", self.id))?;

        let value = self.expression.evaluate(next_value) / 3;

        let new_monkey = if value % self.divisor == 0 {
            self.true_path
        } else {
            self.false_path
        };

        self.inspections += 1;

        Ok(InspectionResult { new_monkey, value })
    }

    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }

    fn parse(lines: &mut Peekable<Lines>) -> Result<Self> {
        let line = {
            let line = lines.next().unwrap();
            if line.is_empty() {
                lines.next().unwrap()
            } else {
                line
            }
        };

        let monkey_id_parts = line.split_whitespace().collect::<Vec<&str>>();
        let monkey_id_str = monkey_id_parts[1];
        let monkey_id: usize = monkey_id_str
            .chars()
            .take(monkey_id_str.find(':').unwrap())
            .collect::<String>()
            .parse()?;

        let line = lines.next().unwrap();
        let items: VecDeque<i64> = line.split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let line = lines.next().unwrap();
        let right_part = line.split(" = ").collect::<Vec<&str>>()[1];
        let parts: Vec<&str> = right_part.split_whitespace().collect();
        let left: Operand = parts[0].parse()?;
        let operation: Operation = parts[1].parse()?;
        let right: Operand = parts[2].parse()?;

        let expression = Expression {
            left,
            operation,
            right,
        };

        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let divisor: i64 = parts[parts.len() - 1].parse()?;

        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let true_path: usize = parts[parts.len() - 1].parse()?;

        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let false_path: usize = parts[parts.len() - 1].parse()?;

        Ok(Monkey {
            id: monkey_id,
            items,
            expression,
            divisor,
            true_path,
            false_path,
            inspections: 0,
        })
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.id,
            self.items
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant(i64),
}

impl Operand {
    fn value(&self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Constant(v) => *v,
        }
    }
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        if value == "old" {
            return Ok(Operand::Old);
        }

        Ok(Operand::Constant(value.parse()?))
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn evaluate(&self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Multiply => left * right,
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(anyhow!("Unknown operation {}", value)),
        }
    }
}

#[derive(Debug)]
struct Expression {
    left: Operand,
    operation: Operation,
    right: Operand,
}

impl Expression {
    fn evaluate(&self, old: i64) -> i64 {
        self.operation
            .evaluate(self.left.value(old), self.right.value(old))
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut monkeys = HashMap::new();
    let data = std::fs::read_to_string(&args.filename)?;
    let mut lines = data.lines().peekable();

    loop {
        if lines.peek().is_some() {
            let monkey = Monkey::parse(&mut lines)?;
            monkeys.insert(monkey.id, monkey);
        } else {
            break;
        }
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut new_destinations: HashMap<usize, VecDeque<i64>> = HashMap::new();

            while monkeys[&i].has_next_item() {
                let result = monkeys.get_mut(&i).unwrap().inspect_next()?;
                new_destinations
                    .entry(result.new_monkey)
                    .or_insert_with(VecDeque::new)
                    .push_back(result.value);
            }

            for (monkey_id, values) in new_destinations {
                for value in values {
                    monkeys.get_mut(&monkey_id).unwrap().add_item(value);
                }
            }
        }

        /*
        println!("");
        println!("Round {}", round + 1);
        for monkey in monkeys.values() {
            println!("{}", monkey);
        }
        */
    }

    let mut all_inspections: Vec<usize> = monkeys.values().map(|m| m.inspections).collect();
    all_inspections.sort_by(|a, b| b.cmp(a));

    println!("{}", all_inspections[0] * all_inspections[1]);

    Ok(())
}
