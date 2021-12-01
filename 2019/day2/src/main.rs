use std::convert::{TryInto, TryFrom};

use anyhow::{format_err, Error, Result};

enum OpCode {
    Add,
    Multiply,
    Terminate,
}

impl TryFrom<i64> for OpCode {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            99 => Ok(OpCode::Terminate),
            _ => Err(format_err!("Unknown opcode {}", value)),
        }
    }
}

enum InstructionResult {
    Continue,
    Terminate,
}

fn string_to_vec(input: &str) -> Result<Vec<i64>> {
    let mut ret = Vec::new();
    for num in input.trim().split(",") {
        ret.push(num.parse()?);
    }
    Ok(ret)
}

fn read_input(filename: &str) -> Result<Vec<i64>> {
    let data = std::fs::read_to_string(filename)?;

    string_to_vec(&data)
}

fn get_position(program: &Vec<i64>, position: usize) -> i64 {
    *program.get(position).unwrap()
}

fn get_value(program: &Vec<i64>, position: usize) -> i64 {
    *program.get(get_position(program, position) as usize).unwrap()
}

fn run_instruction(program: &mut Vec<i64>, pc: usize) -> Result<InstructionResult> {
    match program.get(pc) {
        Some(opcode) => match (*opcode).try_into()? {
            OpCode::Add => {
                let result_offset = get_position(program, pc + 3) as usize;
                program[result_offset] =
                    get_value(program, pc + 1) + get_value(program, pc + 2);

                Ok(InstructionResult::Continue)
            },
            OpCode::Multiply => {
                let result_offset = get_position(program, pc + 3) as usize;
                program[result_offset] =
                    get_value(program, pc + 1) * get_value(program, pc + 2);

                Ok(InstructionResult::Continue)
            },
            OpCode::Terminate => {
                Ok(InstructionResult::Terminate)
            },
        },
        None => Err(format_err!("Program counter is out of bounds")),
    }
}

fn run_program(program: &Vec<i64>, input_a: i64, input_b: i64) -> Result<i64> {
    let mut program = program.clone();

    program[1] = input_a;
    program[2] = input_b;

    let mut pc = 0;

    loop {
        match run_instruction(&mut program, pc)? {
            InstructionResult::Continue => pc += 4,
            InstructionResult::Terminate => break,
        }
    }

    Ok(program[0])
}

fn main() -> Result<()> {
    let program = read_input("input.txt")?;

    for i in 0..100 {
        for j in 0..100 {
            if run_program(&program, i, j)? == 19690720 {
                println!("Found! ({}, {}), value is {}", i, j, 100 * i + j);
            }
        }
    }

    Ok(())
}

