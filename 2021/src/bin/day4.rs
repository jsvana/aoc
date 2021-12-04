use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

use aoc_2021::{read_lines, Args};

#[derive(Debug)]
struct BingoBoard<const SIZE: usize> {
    id: usize,
    spaces: [[i32; SIZE]; SIZE],
    marked: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> BingoBoard<SIZE> {
    fn from_iter<'a, T>(iter: &mut T, id: usize) -> Result<Self>
    where
        T: Iterator<Item = &'a String>,
    {
        let mut spaces: [[i32; SIZE]; SIZE] = [[-1; SIZE]; SIZE];

        for i in 0..SIZE {
            let line_parts = iter
                .next()
                .ok_or_else(|| anyhow!("Missing line {} while constructing bingo board", i))?
                .split_whitespace();

            let line_spaces: Vec<i32> = line_parts
                .map(|i| {
                    i.parse()
                        .map_err(|e| anyhow!("Invalid number in bingo board {} ({})", i, e))
                })
                .collect::<Result<Vec<i32>>>()?;

            if line_spaces.len() > SIZE {
                return Err(anyhow!(
                    "Too many numbers in row {} (expected {}, got {})",
                    i,
                    SIZE,
                    line_spaces.len()
                ));
            }

            for j in 0..SIZE {
                spaces[i][j] = *line_spaces
                    .get(j)
                    .ok_or_else(|| anyhow!("Missing bingo board space ({}, {})", i, j))?;
            }
        }

        Ok(Self {
            id,
            spaces,
            marked: [[false; SIZE]; SIZE],
        })
    }

    fn add_number(&mut self, number: i32) {
        for (i, row) in self.spaces.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if *space == number {
                    self.marked[i][j] = true;
                    break;
                }
            }
        }
    }

    fn row_has_win(&self, row: usize) -> bool {
        if row > SIZE {
            return false;
        }

        for j in 0..SIZE {
            if !self.marked[row][j] {
                return false;
            }
        }

        true
    }

    fn column_has_win(&self, column: usize) -> bool {
        if column > SIZE {
            return false;
        }

        for i in 0..SIZE {
            if !self.marked[i][column] {
                return false;
            }
        }

        true
    }

    fn crosses_have_win(&self) -> bool {
        for i in 0..SIZE {
            if !self.marked[i][i] {
                return false;
            }

            if !self.marked[i][SIZE - i - 1] {
                return false;
            }
        }

        true
    }

    fn has_win(&self) -> bool {
        for i in 0..SIZE {
            if self.row_has_win(i) {
                return true;
            }

            if self.column_has_win(i) {
                return true;
            }
        }

        if self.crosses_have_win() {
            return true;
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if !self.marked[i][j] {
                    sum += self.spaces[i][j];
                }
            }
        }

        sum
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let lines: Vec<String> = read_lines(&args.filename)?;

    let mut lines_iter = lines.iter();

    let inputs: VecDeque<i32> = lines_iter
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|i| {
            i.parse()
                .map_err(|e| anyhow!("Invalid bingo number {} ({})", i, e))
        })
        .collect::<Result<VecDeque<i32>>>()?;

    let mut lines_iter = lines_iter.peekable();

    let mut board_id = 0;
    let mut boards = Vec::new();
    while let Some(_) = lines_iter.peek() {
        boards.push(BingoBoard::<5>::from_iter(&mut lines_iter, board_id)?);
        board_id += 1;
    }

    dbg!(board_id);

    let mut winning_ids = HashSet::new();
    let mut found_last = false;

    for input in inputs {
        for board in &mut boards {
            board.add_number(input);
            if board.has_win() {
                winning_ids.insert(board.id);
                if winning_ids.len() == board_id {
                    println!("{}", board.unmarked_sum() * input);
                    found_last = true;
                    break;
                }
            }
        }

        if found_last {
            break;
        }
    }

    Ok(())
}
