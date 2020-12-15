use std::collections::BTreeMap;

use anyhow::{format_err, Result};

fn main() -> Result<()> {
    let numbers: Vec<i64> = vec![11, 0, 1, 10, 5, 19];

    let mut said: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    let mut said_count: BTreeMap<i64, usize> = BTreeMap::new();

    let mut last_number = numbers[0];
    for i in 1..=30000000 {
        //for i in 1..=9 {
        if i - 1 < numbers.len() {
            let number = numbers[i - 1];
            said.entry(number).or_insert(Vec::new()).push(i);
            *said_count.entry(number).or_insert(0) += 1;
            last_number = number;

            continue;
        }

        // Generate next number
        match said_count.get(&last_number) {
            Some(count) => {
                if *count == 1 {
                    said.entry(0).or_insert(Vec::new()).push(i);
                    *said_count.entry(0).or_insert(0) += 1;
                    last_number = 0;
                } else {
                    let turns = said
                        .get(&last_number)
                        .ok_or_else(|| format_err!("num {} missing", last_number))?;

                    let difference = turns[turns.len() - 1] - turns[turns.len() - 2];
                    //println!("  turn {} - last turn {} = {}", i, turn, difference);
                    said.entry(difference as i64).or_insert(Vec::new()).push(i);
                    *said_count.entry(difference as i64).or_insert(0) += 1;
                    last_number = difference as i64;
                }
            }
            None => {
                todo!();
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
