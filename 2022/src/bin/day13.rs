use std::fmt;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use serde_json::Value;
use structopt::StructOpt;

#[derive(Clone, Debug)]
enum Packet {
    Number(u64),
    Packet(Vec<Packet>),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::Packet(values) => {
                write!(
                    f,
                    "[{}]",
                    values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CompareResult {
    Correct,
    Continue,
    OutOfOrder,
    IncorrectlySized,
}

impl Packet {
    fn compare(&self, other: &Packet, indent: usize) -> CompareResult {
        let indent_str: String = std::iter::repeat(" ").take(indent).collect();

        let mut left = self.clone();
        let mut right = other.clone();

        if let (Packet::Number(_), Packet::Packet(_)) = (self, other) {
            let left_list = Packet::Packet(vec![self.clone()]);
            println!(
                "{}- Mixed types; convert left to {} and retry comparison",
                indent_str, left_list
            );
            left = left_list;
        }

        if let (Packet::Packet(_), Packet::Number(_)) = (self, other) {
            let right_list = Packet::Packet(vec![other.clone()]);
            println!(
                "{}- Mixed types; convert right to {} and retry comparison",
                indent_str, right_list
            );
            right = right_list;
        }

        println!("{}- Compare {} vs {}", indent_str, left, right);

        match (left, right) {
            (Packet::Number(left), Packet::Number(right)) => {
                if left < right {
                    println!(
                        "{}- Left side is smaller, so inputs are in the right order",
                        std::iter::repeat(" ").take(indent + 2).collect::<String>()
                    );
                    CompareResult::Correct
                } else if left == right {
                    CompareResult::Continue
                } else {
                    println!(
                        "{}- Right side is smaller, so inputs are not in the right order",
                        std::iter::repeat(" ").take(indent + 2).collect::<String>()
                    );
                    CompareResult::OutOfOrder
                }
            }
            (Packet::Packet(left_list), Packet::Packet(right_list)) => {
                for i in 0..std::cmp::max(left_list.len(), right_list.len()) {
                    if i >= left_list.len() {
                        println!(
                            "{}- Left side ran out of items, so inputs are in the right order",
                            std::iter::repeat(" ").take(indent + 2).collect::<String>()
                        );
                        return CompareResult::Correct;
                    }

                    if i >= right_list.len() {
                        println!(
                            "{}- Right side ran out of items, so inputs are not in the right order",
                            std::iter::repeat(" ").take(indent + 2).collect::<String>()
                        );
                        return CompareResult::IncorrectlySized;
                    }

                    let left = &left_list[i];
                    let right = &right_list[i];

                    let result = left.compare(&right, indent + 2);
                    if result != CompareResult::Continue {
                        return result;
                    }
                }

                CompareResult::Continue
            }
            _ => {
                panic!("lol");
            }
        }
    }
}

impl TryFrom<Value> for Packet {
    type Error = anyhow::Error;

    fn try_from(other: Value) -> Result<Self> {
        match other {
            Value::Number(num) => Ok(Packet::Number(
                num.as_u64()
                    .ok_or_else(|| anyhow!("Invalid u64 in input packet"))?,
            )),
            Value::Array(values) => {
                let mut transformed_values = Vec::new();
                for value in values {
                    transformed_values.push(value.try_into()?);
                }
                Ok(Packet::Packet(transformed_values))
            }
            _ => Err(anyhow!("Unexpected datatype in input line")),
        }
    }
}

struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn compare(&self) -> CompareResult {
        self.left.compare(&self.right, 0)
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let contents = std::fs::read_to_string(&args.filename)?;
    let mut lines = contents.lines().peekable();

    let mut pairs = Vec::new();
    while lines.peek().is_some() {
        if let Some(&"") = lines.peek() {
            lines.next();
        }

        let left: Packet = {
            let value: Value = serde_json::from_str(lines.next().unwrap())?;
            value.try_into()?
        };
        let right: Packet = {
            let value: Value = serde_json::from_str(lines.next().unwrap())?;
            value.try_into()?
        };

        pairs.push(Pair { left, right });
    }

    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        println!("== Pair {} ==", i + 1);
        let result = pair.compare();
        if let CompareResult::Correct = result {
            sum += i + 1;
        }
        println!("");
    }

    println!("{}", sum);

    Ok(())
}
