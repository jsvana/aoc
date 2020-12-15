use std::collections::hash_map::Entry;
use std::collections::HashMap;

use anyhow::{format_err, Result};
use maplit::hashmap;

struct TurnPresence {
    last: Option<usize>,
    second_to_last: Option<usize>,
}

impl TurnPresence {
    fn new() -> Self {
        Self {
            last: None,
            second_to_last: None,
        }
    }

    fn new_with_initial_turn(turn: usize) -> Self {
        Self {
            last: Some(turn),
            second_to_last: None,
        }
    }

    fn add_number(&mut self, number: usize) {
        self.second_to_last = self.last;
        self.last = Some(number);
    }

    fn turn_difference(&self) -> Option<usize> {
        self.last.and_then(|l| self.second_to_last.map(|s| l - s))
    }
}

fn main() -> Result<()> {
    let numbers: Vec<i64> = vec![11, 0, 1, 10, 5, 19];

    let mut said = hashmap! {
        11 => TurnPresence::new_with_initial_turn(1),
        0 => TurnPresence::new_with_initial_turn(2),
        1 => TurnPresence::new_with_initial_turn(3),
        10 => TurnPresence::new_with_initial_turn(4),
        5 => TurnPresence::new_with_initial_turn(5),
        19 => TurnPresence::new_with_initial_turn(6),
    };
    let mut said_count: HashMap<i64, usize> = hashmap! {
        11 => 1,
        0 => 1,
        1 => 1,
        10 => 1,
        5 => 1,
        19 => 1,
    };

    let mut last_number = numbers[numbers.len() - 1];
    for i in numbers.len() + 1..=30000000 {
        if let Entry::Occupied(entry) = said_count.entry(last_number) {
            let count = *entry.get();
            if count == 1 {
                said.entry(0).or_insert(TurnPresence::new()).add_number(i);
                *said_count.entry(0).or_insert(0) += 1;
                last_number = 0;
            } else {
                let turns = said
                    .get(&last_number)
                    .ok_or_else(|| format_err!("num {} missing", last_number))?;

                let difference = turns
                    .turn_difference()
                    .ok_or_else(|| format_err!("only one turn found"))?;
                said.entry(difference as i64)
                    .or_insert(TurnPresence::new())
                    .add_number(i);
                *said_count.entry(difference as i64).or_insert(0) += 1;
                last_number = difference as i64;
            }
        }

        if i == 2020 {
            println!("Part 1: {}", last_number);
        } else if i == 30000000 {
            println!("Part 2: {}", last_number);
        }
    }

    Ok(())
}
