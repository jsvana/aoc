use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use anyhow::{format_err, Result};
use structopt::StructOpt;
use thiserror::Error;

use aoc_2020::{read_lines, Args};

#[derive(Clone, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn run(&self, state: ProgramState) -> Result<ProgramState> {
        let starting_program_counter = state.program_counter;
        let mut program_counter = state.program_counter;
        let mut accumulator = state.accumulator;

        match self {
            Instruction::Acc(value) => {
                accumulator += value;
                program_counter += 1;
            }
            Instruction::Jmp(value) => {
                program_counter += value;
            }
            Instruction::Nop(_) => {
                program_counter += 1;
            }
        }

        if starting_program_counter == program_counter {
            Err(format_err!(
                "program counter is unchanged after running instruction \"{}\"",
                self
            ))
        } else {
            Ok(ProgramState::from_values(program_counter, accumulator))
        }
    }
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("instruction \"{0}\" is missing its instruction name")]
    MissingInstructionName(String),
    #[error("instruction \"{0}\" has an invalid instruction name")]
    InvalidInstructionName(String),
    #[error("instruction \"{0}\" is missing its value")]
    MissingValue(String),
    #[error("instruction \"{0}\" has an invalid value")]
    InvalidValue(String),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();

        let value = parts
            .get(1)
            .ok_or_else(|| InstructionParseError::MissingValue(line.to_string()))?
            .parse()
            .map_err(|_| InstructionParseError::InvalidValue(line.to_string()))?;

        match parts.get(0) {
            Some(&"acc") => Ok(Instruction::Acc(value)),
            Some(&"jmp") => Ok(Instruction::Jmp(value)),
            Some(&"nop") => Ok(Instruction::Nop(value)),
            Some(instruction_name) => Err(InstructionParseError::InvalidInstructionName(
                instruction_name.to_string(),
            )),
            _ => Err(InstructionParseError::MissingInstructionName(
                line.to_string(),
            )),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Acc(value) => write!(f, "acc:{}", value),
            Instruction::Jmp(value) => write!(f, "jmp:{}", value),
            Instruction::Nop(value) => write!(f, "nop:{}", value),
        }
    }
}

#[derive(Debug)]
struct ProgramState {
    program_counter: i32,
    accumulator: i32,
}

impl ProgramState {
    fn new() -> Self {
        Self {
            program_counter: 0,
            accumulator: 0,
        }
    }

    fn from_values(program_counter: i32, accumulator: i32) -> Self {
        Self {
            program_counter,
            accumulator,
        }
    }
}

#[derive(Debug)]
enum Termination {
    HitRepeated(i32),
    Completed(i32),
}

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    fn run_until_repeated_instruction(&self) -> Result<Termination> {
        let mut state = ProgramState::new();

        let mut seen_program_counters = BTreeSet::new();

        while state.program_counter >= 0 && state.program_counter < self.instructions.len() as i32 {
            if seen_program_counters.contains(&state.program_counter) {
                return Ok(Termination::HitRepeated(state.accumulator));
            }

            seen_program_counters.insert(state.program_counter);

            let instruction = self
                .instructions
                .get(state.program_counter as usize)
                .ok_or_else(|| {
                    format_err!(
                        "program counter {} is somehow outside instruction bounds [0:{})",
                        state.program_counter,
                        self.instructions.len()
                    )
                })?;

            state = instruction.run(state)?;
        }

        Ok(Termination::Completed(state.accumulator))
    }
}

fn part1(program: Program) -> Result<()> {
    if let Termination::HitRepeated(value) = program.run_until_repeated_instruction()? {
        println!("Part 1: {}", value);
    }

    Ok(())
}

fn part2(program: Program) -> Result<()> {
    for (i, instruction) in program.instructions.iter().enumerate() {
        let replacement_instruction = match instruction {
            Instruction::Jmp(value) => Instruction::Nop(*value),
            Instruction::Nop(value) => Instruction::Jmp(*value),
            _ => continue,
        };

        let mut new_instructions = Vec::new();
        new_instructions.extend_from_slice(&program.instructions[..i]);
        new_instructions.push(replacement_instruction);
        new_instructions.extend_from_slice(&program.instructions[i + 1..]);

        let new_program = Program::new(new_instructions);
        if let Termination::Completed(value) = new_program.run_until_repeated_instruction()? {
            println!("Part 2: {}", value);
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let program = Program::new(read_lines(&args.filename)?);

    part1(program.clone())?;
    part2(program)?;

    Ok(())
}
