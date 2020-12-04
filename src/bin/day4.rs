use std::collections::{BTreeMap, BTreeSet};

use anyhow::{format_err, Result};
use maplit::btreeset;
use structopt::StructOpt;

use aoc_2020::Args;

fn is_valid_byr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }

    let value: i32 = match value.parse() {
        Ok(value) => value,
        Err(_) => {
            return false;
        }
    };

    value >= 1920 && value <= 2002
}

fn is_valid_iyr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }

    let value: i32 = match value.parse() {
        Ok(value) => value,
        Err(_) => {
            return false;
        }
    };

    value >= 2010 && value <= 2020
}

fn is_valid_eyr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }

    let value: i32 = match value.parse() {
        Ok(value) => value,
        Err(_) => {
            return false;
        }
    };

    value >= 2020 && value <= 2030
}

fn is_valid_hgt(value: &str) -> bool {
    if value.ends_with("cm") {
        match value[..value.len() - 2].parse::<i32>() {
            Ok(value) => value >= 150 && value <= 193,
            Err(_) => false,
        }
    } else if value.ends_with("in") {
        match value[..value.len() - 2].parse::<i32>() {
            Ok(value) => value >= 59 && value <= 76,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn is_valid_hcl(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }

    let mut chars = value.chars();

    if chars.next() != Some('#') {
        return false;
    }

    for c in chars {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}

fn is_valid_ecl(value: &str) -> bool {
    let valid_ecls = btreeset!{
        "amb", "blu", "brn", "gry", "grn", "hzl", "oth",
    };

    valid_ecls.contains(value)
}

fn is_valid_pid(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    for c in value.chars().next() {
        if !c.is_ascii_digit() {
            return false;
        }
    }

    true
}

#[derive(Debug)]
struct Passport {
    fields: BTreeMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Self { fields: BTreeMap::new() }
    }

    fn add_field(&mut self, field: &str, value: &str) {
        self.fields.insert(field.to_string(), value.to_string());
    }

    fn has_valid_fields(&self) -> bool {
        let required_fields = btreeset!{
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
        };

        let optional_fields = btreeset!{"cid".to_string()};

        let fields: BTreeSet<String> = self.fields.keys().cloned().collect::<BTreeSet<String>>().difference(&optional_fields).cloned().collect();

        fields == required_fields
    }

    fn is_valid(&self) -> bool {
        if !self.has_valid_fields() {
            return false;
        }

        match self.fields.get("byr") {
            Some(value) => {
                if !is_valid_byr(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("iyr") {
            Some(value) => {
                if !is_valid_iyr(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("eyr") {
            Some(value) => {
                if !is_valid_eyr(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("hgt") {
            Some(value) => {
                if !is_valid_hgt(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("hcl") {
            Some(value) => {
                if !is_valid_hcl(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("ecl") {
            Some(value) => {
                if !is_valid_ecl(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        match self.fields.get("pid") {
            Some(value) => {
                if !is_valid_pid(value) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        true
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut passports = Vec::new();

    let contents = std::fs::read_to_string(args.filename)?;

    let mut current_passport = Passport::new();
    for line in contents.split("\n") {
        if line.trim().is_empty() {
            passports.push(current_passport);
            current_passport = Passport::new();

            continue;
        }

        for token in line.split_whitespace() {
            let parts: Vec<&str> = token.split(":").collect();
            let field = parts.get(0).ok_or_else(|| format_err!("token \"{}\" missing field", token))?;
            let value = parts.get(1).ok_or_else(|| format_err!("token \"{}\" missing value", token))?;

            current_passport.add_field(field, value);
        }
    }

    let mut valid_passport_count = 0;
    for passport in passports.into_iter() {
        if passport.is_valid() {
            valid_passport_count += 1;
        }
    }

    println!("{}", valid_passport_count);

    Ok(())
}
