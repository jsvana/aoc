use std::collections::{HashMap, HashSet};
use std::str::Chars;

use anyhow::{anyhow, Result};
use aoc_2021::Args;
use lazy_static::lazy_static;
use maplit::{hashmap, hashset};
use structopt::StructOpt;

lazy_static! {
    static ref VALID_CHARS: HashSet<char> = hashset! {
        '(', '[', '{', '<', ')', ']', '}', '>',
    };
    static ref CLOSING_CHARS: HashMap<char, char> = hashmap! {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
    };
    static ref SCORES: HashMap<char, usize> = hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    };
    static ref COMPLETION_SCORES: HashMap<char, usize> = hashmap! {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
    };
}

#[derive(Debug)]
struct Chunk {
    chunks: Vec<Chunk>,
}

#[derive(Debug)]
enum ParseResult {
    Ok(Chunk),
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn parse_chunk(indent: usize, chars: &mut Chars, ending_char: char) -> Result<ParseResult> {
    let mut chunks = Vec::new();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                return Ok(ParseResult::Incomplete(vec![ending_char]));
            }
        };

        if !VALID_CHARS.contains(&c) {
            return Err(anyhow!("Invalid character {}", c));
        }

        if c == ending_char {
            return Ok(ParseResult::Ok(Chunk { chunks }));
        }

        if CLOSING_CHARS.contains_key(&c) {
            match parse_chunk(indent + 2, chars, CLOSING_CHARS[&c])? {
                ParseResult::Ok(chunk) => {
                    chunks.push(chunk);
                }
                ParseResult::Incomplete(mut chars_to_complete) => {
                    chars_to_complete.push(ending_char);
                    return Ok(ParseResult::Incomplete(chars_to_complete));
                }
                ParseResult::Corrupted(bad_char) => {
                    return Ok(ParseResult::Corrupted(bad_char));
                }
            }
        } else {
            return Ok(ParseResult::Corrupted(c));
        }
    }
}

fn parse_chunks(line: &str) -> Result<ParseResult> {
    if line.len() < 2 {
        return Ok(ParseResult::Incomplete(Vec::new()));
    }

    let mut chars = line.chars();
    let mut chunks = Vec::new();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                break;
            }
        };

        match parse_chunk(0, &mut chars, CLOSING_CHARS[&c])? {
            ParseResult::Ok(chunk) => {
                chunks.push(chunk);
            }
            ParseResult::Incomplete(chars_to_complete) => {
                return Ok(ParseResult::Incomplete(chars_to_complete));
            }
            ParseResult::Corrupted(bad_char) => {
                return Ok(ParseResult::Corrupted(bad_char));
            }
        }
    }

    Ok(ParseResult::Ok(Chunk { chunks }))
}

fn completion_score(chars_to_complete: Vec<char>) -> usize {
    let mut score = 0;
    for c in chars_to_complete {
        score *= 5;
        score += COMPLETION_SCORES[&c];
    }
    score
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut total_score = 0;
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        if let ParseResult::Corrupted(bad_char) = parse_chunks(&line)? {
            total_score += SCORES
                .get(&bad_char)
                .ok_or_else(|| anyhow!("Unknown character {}", bad_char))?;
        }
    }

    println!("{}", total_score);

    let mut scores = Vec::new();
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        if let ParseResult::Incomplete(chars_to_complete) = parse_chunks(&line)? {
            scores.push(completion_score(chars_to_complete));
        }
    }

    scores.sort();

    println!("{}", scores[scores.len() / 2]);

    Ok(())
}
