use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use anyhow::{format_err, Result};
use itertools::Itertools;
use log::{debug, error, trace};
use structopt::StructOpt;
use thiserror::Error;

use aoc_2020::Args;

#[derive(Debug)]
enum Token {
    Char(char),
    Rule(i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Char(c) => write!(f, "c:{}", c),
            Token::Rule(r) => write!(f, "r:{}", r),
        }
    }
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

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.tokens.iter().map(|t| t.to_string()).join(",")
        )
    }
}

impl Pattern {
    fn matches(
        &self,
        rules: &HashMap<i32, Rule>,
        string: &str,
        start_index: usize,
    ) -> Result<usize> {
        trace!("  checking idx {}", start_index);

        let mut string_index = start_index;

        for item in self.tokens.iter() {
            match item {
                Token::Rule(rule_id) => {
                    string_index = rules[rule_id].matches(rules, string, string_index)?;
                }
                Token::Char(c) => {
                    if string_index >= string.len() {
                        error!(
                            "    {} longer than {}. input {}",
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
                        trace!("    char {}, no match", c);
                        return Err(format_err!("no match"));
                    }
                }
            }
        }

        trace!(
            "    matches, [str:{}, pattern:{}]",
            &string[string_index..],
            self
        );

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

    let (mut rules, to_match) = build_rules(&args.filename)?;

    /*
    let mut count = 0;
    for string in to_match.iter() {
        debug!("test {}", string);
        if let Ok(true) = rules_match(&rules, string) {
            debug!("matches");
            count += 1;
        }
    }

    println!("Part 1: {}", count);
    */

    rules.insert(8, "8: 42 | 42 8".parse()?);
    rules.insert(11, "11: 42 31 | 42 11 31".parse()?);

    let mut count = 0;
    for string in to_match[2..].iter() {
        debug!("test {}", string);
        if let Ok(true) = rules_match(&rules, string) {
            debug!("matches");
            count += 1;
        }
        break;
    }

    println!("Part 2: {}", count);

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

    #[test]
    fn test_second_example_without_mod() -> Result<()> {
        let rules: HashMap<i32, Rule> = hashmap! {
            42 => "42: 9 14 | 10 1".parse()?,
            9 => "9: 14 27 | 1 26".parse()?,
            10 => "10: 23 14 | 28 1".parse()?,
            1 => "1: \"a\"".parse()?,
            11 => "11: 42 31".parse()?,
            5 => "5: 1 14 | 15 1".parse()?,
            19 => "19: 14 1 | 14 14".parse()?,
            12 => "12: 24 14 | 19 1".parse()?,
            16 => "16: 15 1 | 14 14".parse()?,
            31 => "31: 14 17 | 1 13".parse()?,
            6 => "6: 14 14 | 1 14".parse()?,
            2 => "2: 1 24 | 14 4".parse()?,
            0 => "0: 8 11".parse()?,
            13 => "13: 14 3 | 1 12".parse()?,
            15 => "15: 1 | 14".parse()?,
            17 => "17: 14 2 | 1 7".parse()?,
            23 => "23: 25 1 | 22 14".parse()?,
            28 => "28: 16 1".parse()?,
            4 => "4: 1 1".parse()?,
            20 => "20: 14 14 | 1 15".parse()?,
            3 => "3: 5 14 | 16 1".parse()?,
            27 => "27: 1 6 | 14 18".parse()?,
            14 => "14: \"b\"".parse()?,
            21 => "21: 14 1 | 1 14".parse()?,
            25 => "25: 1 1 | 1 14".parse()?,
            22 => "22: 14 14".parse()?,
            8 => "8: 42".parse()?,
            26 => "26: 14 22 | 1 20".parse()?,
            18 => "18: 15 15".parse()?,
            7 => "7: 14 5 | 1 21".parse()?,
            24 => "24: 14 1".parse()?,
        };

        let should_match = vec!["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"];

        for string in should_match.into_iter() {
            assert!(rules_match(&rules, string)?);
        }

        let should_not_match = vec![
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        for string in should_not_match.into_iter() {
            assert!(!rules_match(&rules, string)?);
        }

        Ok(())
    }

    #[test]
    fn test_second_example_with_mod() -> Result<()> {
        pretty_env_logger::init();

        let rules: HashMap<i32, Rule> = hashmap! {
            42 => "42: 9 14 | 10 1".parse()?,
            9 => "9: 14 27 | 1 26".parse()?,
            10 => "10: 23 14 | 28 1".parse()?,
            1 => "1: \"a\"".parse()?,
            11 => "11: 42 31 | 42 11 31".parse()?,
            5 => "5: 1 14 | 15 1".parse()?,
            19 => "19: 14 1 | 14 14".parse()?,
            12 => "12: 24 14 | 19 1".parse()?,
            16 => "16: 15 1 | 14 14".parse()?,
            31 => "31: 14 17 | 1 13".parse()?,
            6 => "6: 14 14 | 1 14".parse()?,
            2 => "2: 1 24 | 14 4".parse()?,
            0 => "0: 8 11".parse()?,
            13 => "13: 14 3 | 1 12".parse()?,
            15 => "15: 1 | 14".parse()?,
            17 => "17: 14 2 | 1 7".parse()?,
            23 => "23: 25 1 | 22 14".parse()?,
            28 => "28: 16 1".parse()?,
            4 => "4: 1 1".parse()?,
            20 => "20: 14 14 | 1 15".parse()?,
            3 => "3: 5 14 | 16 1".parse()?,
            27 => "27: 1 6 | 14 18".parse()?,
            14 => "14: \"b\"".parse()?,
            21 => "21: 14 1 | 1 14".parse()?,
            25 => "25: 1 1 | 1 14".parse()?,
            22 => "22: 14 14".parse()?,
            8 => "8: 42 | 42 8".parse()?,
            26 => "26: 14 22 | 1 20".parse()?,
            18 => "18: 15 15".parse()?,
            7 => "7: 14 5 | 1 21".parse()?,
            24 => "24: 14 1".parse()?,
        };

        let should_match = vec![
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        for string in should_match.into_iter() {
            debug!("testing {}", string);
            assert!(rules_match(&rules, string)?);
        }

        let should_not_match = vec![
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "aaaabbaaaabbaaa",
            "babaaabbbaaabaababbaabababaaab",
        ];

        for string in should_not_match.into_iter() {
            assert!(!rules_match(&rules, string)?);
        }

        Ok(())
    }
}
