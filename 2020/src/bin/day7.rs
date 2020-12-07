use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use structopt::StructOpt;

use aoc_2020::Args;

fn main() -> Result<()> {
    let args = Args::from_args();

    let contents = std::fs::read_to_string(args.filename)?;

    for line in contents.split("\n") {
        let line = line.trim();
    }

    Ok(())
}
