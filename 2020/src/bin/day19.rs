use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{format_err, Result};
use log::{debug, error, trace};
use structopt::StructOpt;
use thiserror::Error;

use aoc_2020::Args;

#[derive(Debug)]
enum Token {
    Char(char),
    Rule(i32),
}

#[derive(Debug)]
struct Pattern {
    tokens: Vec<Token>,
}

#[derive(Debug, Error)]
enum ParsePatternError {
    #[error(transparent)]
    NonInteger(#[from] std::num::ParseIntError),
    #[error("malformed character rule")]
    MalformedChar,
}

impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let mut tokens = Vec::new();
        for part in parts.into_iter() {
            if part.starts_with("\"") {
                tokens.push(Token::Char(
                    part.chars()
                        .nth(1)
                        .ok_or_else(|| ParsePatternError::MalformedChar)?,
                ));
            } else {
                tokens.push(Token::Rule(part.parse()?));
            }
        }

        Ok(Self { tokens })
    }
}

impl Pattern {
    fn matches(
        &self,
        rules: &HashMap<i32, Rule>,
        string: &str,
        start_index: usize,
    ) -> Result<usize> {
        trace!("checking idx {}", start_index);

        let mut string_index = start_index;

        for item in self.tokens.iter() {
            match item {
                Token::Rule(rule_id) => {
                    string_index = rules[rule_id].matches(rules, string, string_index)?;
                }
                Token::Char(c) => {
                    if string_index >= string.len() {
                        error!(
                            "  {} longer than {}. input {}",
                            string_index,
                            string.len(),
                            string
                        );
                        return Err(format_err!("idx longer than input"));
                    }

                    if string.chars().nth(string_index).ok_or_else(|| {
                        format_err!(
                            "no char found at idx {} for string {}",
                            string_index,
                            string
                        )
                    })? == *c
                    {
                        string_index += 1;
                    } else {
                        trace!("  char {}, no match", c);
                        return Err(format_err!("no match"));
                    }
                }
            }
        }

        trace!("  matches, idx {}", string_index);

        Ok(string_index)
    }
}

#[derive(Debug)]
enum RulePattern {
    Simple { pattern: Pattern },
    Complex { left: Pattern, right: Pattern },
}

#[derive(Debug)]
struct Rule {
    id: i32,
    pattern: RulePattern,
}

#[derive(Debug, Error)]
enum ParseRuleError {
    #[error("no ID found in the rule")]
    NoIdFound,
    #[error(transparent)]
    NonInteger(#[from] std::num::ParseIntError),
    #[error(transparent)]
    InvalidPattern(#[from] ParsePatternError),
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.find(':').ok_or_else(|| ParseRuleError::NoIdFound)?;

        let id = s[..index].parse()?;

        match s.find('|') {
            Some(pipe_index) => Ok(Rule {
                id,
                pattern: RulePattern::Complex {
                    left: s[index + 1..pipe_index - 1].parse()?,
                    right: s[pipe_index + 2..].parse()?,
                },
            }),
            None => Ok(Rule {
                id,
                pattern: RulePattern::Simple {
                    pattern: s[index + 1..].parse()?,
                },
            }),
        }
    }
}

impl Rule {
    fn matches(
        &self,
        rules: &HashMap<i32, Rule>,
        string: &str,
        start_index: usize,
    ) -> Result<usize> {
        match &self.pattern {
            RulePattern::Simple { pattern } => {
                trace!("[id:{}] checking simple rule idx {}", self.id, start_index);
                pattern.matches(rules, string, start_index)
            }
            RulePattern::Complex { left, right } => {
                trace!("[id:{}] checking complex rule idx {}", self.id, start_index);
                if let Ok(new_index) = left.matches(rules, string, start_index) {
                    trace!("  [id:{}] left matches, idx {}", self.id, new_index);
                    return Ok(new_index);
                }

                if let Ok(new_index) = right.matches(rules, string, start_index) {
                    trace!("  [id:{}] right matches, idx {}", self.id, new_index);
                    return Ok(new_index);
                }

                trace!("  [id:{}] no match", self.id);

                Err(format_err!("no match"))
            }
        }
    }
}

fn build_rules(filename: &str) -> Result<(HashMap<i32, Rule>, Vec<String>)> {
    let contents = std::fs::read_to_string(filename)?;

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut to_match = Vec::new();

    let mut i = 0;
    let mut rules = HashMap::new();

    while i < lines.len() {
        let line = lines[i];

        i += 1;

        if line.is_empty() {
            for line in lines[i..].iter() {
                if !line.is_empty() {
                    to_match.push(line.to_string());
                }
            }
            break;
        }

        let rule: Rule = line.parse()?;
        rules.insert(rule.id, rule);
    }

    Ok((rules, to_match))
}

fn rules_match(rules: &HashMap<i32, Rule>, string: &str) -> Result<bool> {
    let start_rule = rules
        .get(&0)
        .ok_or_else(|| format_err!("rules missing key 0"))?;

    match start_rule.matches(rules, string, 0) {
        Ok(index) => Ok(index == string.len()),
        Err(_) => Ok(false),
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::from_args();

    let (rules, to_match) = build_rules(&args.filename)?;

    let mut count = 0;
    for string in to_match.into_iter() {
        debug!("test {}", string);
        if let Ok(true) = rules_match(&rules, &string) {
            debug!("matches");
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashmap;

    #[test]
    fn test_simple() -> Result<()> {
        let rules: HashMap<i32, Rule> = hashmap! {
            0 => "0: 1 3".parse()?,
            1 => "1: \"a\"".parse()?,
            3 => "3: \"b\"".parse()?,
        };

        assert!(rules_match(&rules, "ab")?);

        assert!(!rules_match(&rules, "ba")?);
        assert!(!rules_match(&rules, "")?);
        assert!(!rules_match(&rules, "a")?);
        assert!(!rules_match(&rules, "b")?);

        Ok(())
    }

    #[test]
    fn test_complex() -> Result<()> {
        let rules: HashMap<i32, Rule> = hashmap! {
            0 => "0: 1 2".parse()?,
            1 => "1: \"a\"".parse()?,
            2 => "2: 1 3 | 3 1".parse()?,
            3 => "3: \"b\"".parse()?,
        };

        assert!(rules_match(&rules, "aab")?);
        assert!(rules_match(&rules, "aba")?);

        assert!(!rules_match(&rules, "ba")?);
        assert!(!rules_match(&rules, "")?);
        assert!(!rules_match(&rules, "a")?);
        assert!(!rules_match(&rules, "b")?);

        Ok(())
    }

    #[test]
    fn test_first_example() -> Result<()> {
        let rules: HashMap<i32, Rule> = hashmap! {
            0 => "0: 4 1 5".parse()?,
            1 => "1: 2 3 | 3 2".parse()?,
            2 => "2: 4 4 | 5 5".parse()?,
            3 => "3: 4 5 | 5 4".parse()?,
            4 => "4: \"a\"".parse()?,
            5 => "5: \"b\"".parse()?,
        };

        assert!(rules_match(&rules, "ababbb")?);
        assert!(rules_match(&rules, "abbbab")?);

        assert!(!rules_match(&rules, "bababa")?);
        assert!(!rules_match(&rules, "aaabbb")?);
        assert!(!rules_match(&rules, "aaaabbb")?);

        Ok(())
    }
}
