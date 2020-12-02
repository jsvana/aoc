use anyhow::{Result, format_err};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    filename: String,
}

#[derive(Debug)]
struct Policy {
    lower_count: usize,
    higher_count: usize,
    letter: char,
}

impl Policy {
    fn from_str(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split("-").collect();
        let lower_count: usize = parts.get(0).ok_or_else(|| format_err!("{} missing lower", line))?.parse()?;
        let parts: Vec<&str> = parts.get(1).ok_or_else(|| format_err!("{} missing rest", line))?.split(" ").collect();
        let higher_count: usize = parts.get(0).ok_or_else(|| format_err!("{} missing higher", line))?.parse()?;
        let parts: Vec<&str> = parts.get(1).ok_or_else(|| format_err!("{} missing rest", line))?.split(":").collect();
        let letter: char = parts.get(0).ok_or_else(|| format_err!("{} missing letter", line))?.parse()?;
        Ok(Self {lower_count, higher_count, letter})
    }
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    password: String,
}

fn str_has_char_at(haystack: &str, needle: char, index: usize) -> bool {
    if let Some(letter) = haystack.chars().nth(index - 1) {
        if letter == needle {
            return true;
        }
    }

    false
}

impl Password {
    fn from_str(line: &str) -> Result<Self> {
        let policy = Policy::from_str(line)?;

        let parts: Vec<&str> = line.split(" ").collect();
        let password = parts.get(2).ok_or_else(|| format_err!("{} missing password", line))?.to_string();

        Ok(Self {policy, password})
    }

    fn passes(&self) -> bool {
        let mut letter_count = 0;

        let first_okay = str_has_char_at(&self.password, self.policy.letter, self.policy.lower_count);
        let second_okay = str_has_char_at(&self.password, self.policy.letter, self.policy.higher_count);

        first_okay ^ second_okay
    }
}

fn read_numbers(filename: &str) -> Result<Vec<Password>> {
    let contents = std::fs::read_to_string(filename)?;

    let mut values = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        values.push(Password::from_str(line)?);
    }

    Ok(values)
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let passwords = read_numbers(&args.filename)?;

    let mut count = 0;
    for password in passwords.into_iter() {
        if password.passes() {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
