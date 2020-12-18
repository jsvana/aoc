use anyhow::Result;
use structopt::StructOpt;

use aoc_2020::{read_lines, Args};

peg::parser!( grammar equal_arithmetic() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) " * " y:@ { x * y }
        x:(@) " + " y:@ { x + y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

peg::parser!( grammar plus_arithmetic() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) " * " y:@ { x * y }
        --
        x:(@) " + " y:@ { x + y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

fn main() -> Result<()> {
    let args = Args::from_args();

    /*
    let equal_tests = vec![
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];

    for (i, (test, expected)) in equal_tests.into_iter().enumerate() {
        //let test = test.replace(" ", "");
        let actual = equal_arithmetic::calculate(&test)?;
        if actual == expected {
            println!("Test {} passed", i + 1);
        } else {
            println!(
                "Test {} failed: expected {}, got {}",
                i + 1,
                expected,
                actual
            );
        }
    }

    let plus_tests = vec![
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];

    for (i, (test, expected)) in plus_tests.into_iter().enumerate() {
        //let test = test.replace(" ", "");
        let actual = plus_arithmetic::calculate(&test)?;
        if actual == expected {
            println!("Test {} passed", i + 1);
        } else {
            println!(
                "Test {} failed: expected {}, got {}",
                i + 1,
                expected,
                actual
            );
        }
    }
    */

    let expressions: Vec<String> = read_lines(&args.filename)?;

    let mut sum = 0;
    for expression in expressions.iter() {
        sum += equal_arithmetic::calculate(expression)?;
    }
    println!("Part 1: {}", sum);

    let mut sum = 0;
    for expression in expressions.iter() {
        sum += plus_arithmetic::calculate(expression)?;
    }
    println!("Part 2: {}", sum);

    Ok(())
}
