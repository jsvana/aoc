use std::error::Error;
use std::str::FromStr;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    pub filename: String,
}

pub fn read_lines<T>(filename: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + Error + Send + Sync,
{
    let contents = std::fs::read_to_string(filename)?;

    let mut values = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        values.push(line.parse()?);
    }

    Ok(values)
}
