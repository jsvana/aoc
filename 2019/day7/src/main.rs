use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

use anyhow::{format_err, Context, Error, Result};
use itertools::Itertools;
use log::{debug, info, trace};

fn string_to_vec(input: &str) -> Result<Tape> {
    let mut ret = Vec::new();
    for num in input.trim().split(",") {
        ret.push(num.parse()?);
    }
    Ok(Tape::new(&ret))
}

fn read_input(filename: &str) -> Result<Tape> {
    let data = std::fs::read_to_string(filename)?;

    string_to_vec(&data)
}

#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
}

impl OpCode {
    fn argument_count(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 3,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Input | OpCode::Output => 1,
            OpCode::Terminate => 0,
        }
    }
}

#[derive(Clone)]
struct Tape {
    program: Vec<i64>,
}

impl Tape {
    fn new(program: &Vec<i64>) -> Self {
        Tape {
            program: program.clone(),
        }
    }

    fn get(&self, offset: usize) -> Option<i64> {
        self.program.get(offset).cloned()
    }

    fn set(&mut self, offset: usize, value: i64) -> Result<()> {
        if offset >= self.program.len() {
            return Err(format_err!("Attempted to set offset at a larger offset than the tape contains (attempted set of {}, length {})", offset, self.program.len()));
        }

        trace!("[SET] [{}] = {}", offset, value);

        self.program[offset] = value;

        Ok(())
    }
}

#[derive(Debug)]
enum FetchMode {
    Immediate,
    Position,
}

impl TryFrom<char> for FetchMode {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(FetchMode::Position),
            '1' => Ok(FetchMode::Immediate),
            _ => Err(format_err!("Unknown mode {}", value)),
        }
    }
}

#[derive(Debug)]
struct Argument {
    mode: FetchMode,
    value: i64,
}

impl Argument {
    fn get(&self, tape: &Tape) -> Option<i64> {
        match self.mode {
            FetchMode::Immediate => Some(self.value),
            FetchMode::Position => tape.get(self.value as usize),
        }
    }

    fn get_for_set(&self) -> i64 {
        self.value
    }
}

#[derive(Debug)]
struct Instruction {
    position: usize,
    opcode: OpCode,
    arguments: Vec<Argument>,
}

impl Instruction {
    fn new(tape: &Tape, offset: usize) -> Result<Self> {
        let code = format!(
            "{:0>2}",
            tape.get(offset)
                .ok_or(format_err!("No opcode found at offset {}", offset))?
        );

        let opcode: OpCode = code[code.len() - 2..code.len()]
            .try_into()
            .with_context(|| format!("Failed to parse opcode \"{}\"", code))?;

        let argument_count = opcode.argument_count();
        let mut arguments = Vec::new();
        for (i, c) in format!(
            "{:0>width$}",
            &code[..code.len() - 2],
            width = argument_count,
        )
        .chars()
        .rev()
        .enumerate()
        {
            arguments.push(Argument {
                mode: c
                    .try_into()
                    .with_context(|| format!("Failed to parse mode \"{}\"", c))?,
                value: tape.get(offset + i + 1).unwrap(),
            })
        }

        let instruction = Instruction {
            position: offset,
            opcode,
            arguments,
        };

        instruction
            .validate()
            .context("Instruction failed validation")?;

        Ok(instruction)
    }

    fn validate(&self) -> Result<()> {
        let expected_argument_count = self.opcode.argument_count();

        if self.arguments.len() != expected_argument_count {
            return Err(format_err!(
                "Expected {} argument(s), got {}",
                expected_argument_count,
                self.arguments.len()
            ));
        }

        Ok(())
    }

    fn get_argument(&self, index: usize) -> Result<&Argument> {
        self.arguments.get(index).ok_or(format_err!(
            "Argument {} not found for opcode {:?}",
            index + 1,
            self.opcode
        ))
    }

    fn get_argument_value(&self, tape: &Tape, index: usize) -> Result<i64> {
        self.get_argument(index)?.get(tape).ok_or(format_err!(
            "Argument {} for opcode {:?} is None",
            index + 1,
            self.opcode
        ))
    }

    fn get_argument_value_for_set(&self, index: usize) -> Result<i64> {
        Ok(self.get_argument(index)?.get_for_set())
    }

    fn run(
        &self,
        tape: &mut Tape,
        inputs: &mut VecDeque<i64>,
        outputs: &mut VecDeque<i64>,
    ) -> Result<InstructionResult> {
        trace!("{:?}", self);
        let default_next_offset = self.position + self.opcode.argument_count() + 1;
        match self.opcode {
            OpCode::Add => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(2)?;
                let result = arg1 + arg2;

                trace!(
                    "[ADD] {} + {} = {}, [{}]",
                    arg1,
                    arg2,
                    result,
                    result_offset
                );

                tape.set(result_offset as usize, result).with_context(|| {
                    format!(
                        "Failed to set multiplied value {} to tape index {}",
                        result, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::Multiply => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(2)?;

                let result = arg1 * arg2;

                trace!(
                    "[MUL] {} * {} = {}, [{}]",
                    arg1,
                    arg2,
                    result,
                    result_offset
                );

                tape.set(result_offset as usize, result).with_context(|| {
                    format!(
                        "Failed to set multiplied value {} to tape index {}",
                        result, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::Input => {
                let value = inputs
                    .pop_front()
                    .ok_or(format_err!("No input values left to consume"))?;
                let result_offset = self.get_argument_value_for_set(0)?;

                trace!("[INP] {} -> [{}]", value, result_offset);

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set input value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::Output => {
                outputs.push_back(self.get_argument_value(tape, 0)?);

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::JumpIfTrue => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;

                Ok(InstructionResult::Continue {
                    next_offset: if arg1 == 0 {
                        default_next_offset
                    } else {
                        arg2 as usize
                    },
                })
            }
            OpCode::JumpIfFalse => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;

                Ok(InstructionResult::Continue {
                    next_offset: if arg1 == 0 {
                        arg2 as usize
                    } else {
                        default_next_offset
                    },
                })
            }
            OpCode::LessThan => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(2)?;

                let value = if arg1 < arg2 { 1 } else { 0 };

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set less than value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::Equals => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(2)?;

                let value = if arg1 == arg2 { 1 } else { 0 };

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set less than value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                })
            }
            OpCode::Terminate => Ok(InstructionResult::Terminate),
        }
    }
}

impl TryFrom<i64> for OpCode {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            99 => Ok(OpCode::Terminate),
            _ => Err(format_err!("Unknown opcode {}", value)),
        }
    }
}

impl TryFrom<&str> for OpCode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_i: i64 = value
            .parse()
            .with_context(|| format!("Failed to parse opcode string into i64: \"{}\"", value))?;
        value_i.try_into()
    }
}

enum InstructionResult {
    Continue { next_offset: usize },
    Terminate,
}

struct Program {
    tape: Tape,
    pc: usize,
}

impl Program {
    fn new(tape: &Tape) -> Self {
        Program {
            tape: tape.clone(),
            pc: 0,
        }
    }

    fn run_to_next_output(&mut self, inputs: &mut VecDeque<i64>) -> Result<Option<i64>> {
        let mut outputs = VecDeque::new();

        let mut instruction_count = 0;
        loop {
            let starting_len = outputs.len();
            let instruction = Instruction::new(&self.tape, self.pc)
                .with_context(|| format!("Failed to build instruction at offset {}", self.pc))?;

            match instruction
                .run(&mut self.tape, inputs, &mut outputs)
                .with_context(|| format!("Failed to run instruction at offset {}", self.pc))?
            {
                InstructionResult::Continue { next_offset } => {
                    self.pc = next_offset;
                    if outputs.len() > starting_len {
                        break;
                    }
                }
                InstructionResult::Terminate => break,
            }

            instruction_count += 1;
        }

        debug!("Ran {} instruction(s)", instruction_count);

        Ok(outputs.back().cloned())
    }

    fn run(&mut self, inputs: &mut VecDeque<i64>) -> Result<VecDeque<i64>> {
        // TODO(jsvana): make this not duplicated
        let mut outputs = VecDeque::new();

        loop {
            let instruction = Instruction::new(&self.tape, self.pc)
                .with_context(|| format!("Failed to build instruction at offset {}", self.pc))?;

            match instruction
                .run(&mut self.tape, inputs, &mut outputs)
                .with_context(|| format!("Failed to run instruction at offset {}", self.pc))?
            {
                InstructionResult::Continue { next_offset } => {
                    self.pc = next_offset;
                }
                InstructionResult::Terminate => break,
            }
        }

        Ok(outputs)
    }
}

fn run_phase_sequence(tape: &Tape, sequence: &Vec<i64>) -> Result<i64> {
    let mut input_sequence = VecDeque::new();
    let mut programs = Vec::new();

    for start in sequence.iter() {
        input_sequence.push_back(*start);
        programs.push(Program::new(&tape));
    }

    let mut input = 0;
    let mut i = 0;
    loop {
        let mut inputs = VecDeque::new();
        if !input_sequence.is_empty() {
            inputs.push_back(input_sequence.pop_front().unwrap());
        }
        inputs.push_back(input);
        debug!("PROG {}", i);
        debug!("INPT {:?}", inputs);

        let program = programs.get_mut(i).unwrap();

        let output = program.run_to_next_output(&mut inputs.clone())?;

        debug!("OUTP {:?}", output);

        match output {
            Some(output) => input = output,
            None => {
                break;
            }
        }

        i = (i + 1) % sequence.len();
    }

    Ok(input)
}

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let tape = read_input("input.txt")?;

    let mut max = std::i64::MIN;
    for sequence in (5..10).permutations(5) {
        let output = run_phase_sequence(&tape, &sequence)?;

        if output > max {
            max = output;
        }
    }
    info!("Max output: {}", max);

    Ok(())
}
