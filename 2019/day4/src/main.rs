use std::collections::BTreeMap;

use anyhow::Result;

fn valid_password(password: i64) -> bool {
    let mut digits: Vec<i64> = Vec::new();
    for password_char in format!("{}", password).chars() {
        digits.push(password_char.to_digit(10).unwrap() as i64);
    }

    let mut counts = BTreeMap::new();

    counts.insert(digits[0], 1);

    for i in 1..digits.len() {
        let digit = digits[i];
        if digit < digits[i - 1] {
            return false;
        }
        *counts.entry(digit).or_insert(0) += 1;
    }

    for count in counts.values() {
        if *count == 2 {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    let mut valid_count = 0;
    for i in 245182..790573 {
        if valid_password(i) {
            valid_count += 1;
        }
    }

    println!("Valid: {}", valid_count);

    Ok(())
}
