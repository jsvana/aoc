use std::collections::{BTreeMap, VecDeque};
use std::num::ParseIntError;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use anyhow::{format_err, Context, Error, Result};
use log::{trace};

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
    AdjustRelativeBase,
}

impl OpCode {
    fn argument_count(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 3,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Input | OpCode::Output | OpCode::AdjustRelativeBase => 1,
            OpCode::Terminate => 0,
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
            9 => Ok(OpCode::AdjustRelativeBase),
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

#[derive(Clone)]
pub struct Tape {
    memory: BTreeMap<usize, i64>,
    relative_base: i64,
}

impl Tape {
    fn new(program: &Vec<i64>) -> Self {
        let mut tape = Tape {
            memory: BTreeMap::new(),
            relative_base: 0,
        };
        for (i, item) in program.iter().enumerate() {
            tape.memory.insert(i, *item);
        }

        tape
    }

    fn get(&self, offset: usize) -> Option<i64> {
        self.memory.get(&offset).or(Some(&0)).cloned()
    }

    fn set(&mut self, offset: usize, value: i64) -> Result<()> {
        trace!("[SET] [{}] = {}", offset, value);

        self.memory.insert(offset, value);

        Ok(())
    }

    fn get_relative_base(&self) -> i64 {
        self.relative_base
    }

    fn set_relative_base(&mut self, new_base: i64) {
        self.relative_base = new_base;
    }
}

impl FromStr for Tape {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ret = Vec::new();

        for num in input.trim().split(",") {
            ret.push(num.parse()?);
        }

        Ok(Tape::new(&ret))
    }
}

#[derive(Debug)]
enum FetchMode {
    Immediate,
    Position,
    Relative,
}

impl TryFrom<char> for FetchMode {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(FetchMode::Position),
            '1' => Ok(FetchMode::Immediate),
            '2' => Ok(FetchMode::Relative),
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
    fn get(&self, tape: &Tape, relative_base: i64) -> Option<i64> {
        match self.mode {
            FetchMode::Immediate => Some(self.value),
            FetchMode::Position => tape.get(self.value as usize),
            FetchMode::Relative => tape.get((self.value + relative_base) as usize),
        }
    }

    fn get_for_set(&self, relative_base: i64) -> i64 {
        match self.mode {
            FetchMode::Relative => self.value + relative_base,
            _ => self.value,
        }
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
        self.get_argument(index)?
            .get(tape, tape.get_relative_base())
            .ok_or(format_err!(
                "Argument {} for opcode {:?} is None",
                index + 1,
                self.opcode
            ))
    }

    fn get_argument_value_for_set(&self, tape: &Tape, index: usize) -> Result<i64> {
        Ok(self
            .get_argument(index)?
            .get_for_set(tape.get_relative_base()))
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
                let result_offset = self.get_argument_value_for_set(tape, 2)?;
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
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::Multiply => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(tape, 2)?;

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
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::Input => {
                let value = inputs
                    .pop_front()
                    .ok_or(format_err!("No input values left to consume"))?;
                let result_offset = self.get_argument_value_for_set(tape, 0)?;

                trace!("[INP] {} -> [{}]", value, result_offset);

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set input value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::Output => {
                outputs.push_back(self.get_argument_value(tape, 0)?);

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                    relative_base: tape.get_relative_base(),
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
                    relative_base: tape.get_relative_base(),
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
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::LessThan => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(tape, 2)?;

                let value = if arg1 < arg2 { 1 } else { 0 };

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set less than value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::Equals => {
                let arg1 = self.get_argument_value(tape, 0)?;
                let arg2 = self.get_argument_value(tape, 1)?;
                let result_offset = self.get_argument_value_for_set(tape, 2)?;

                let value = if arg1 == arg2 { 1 } else { 0 };

                tape.set(result_offset as usize, value).with_context(|| {
                    format!(
                        "Failed to set less than value {} to tape index {}",
                        value, result_offset
                    )
                })?;

                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                    relative_base: tape.get_relative_base(),
                })
            }
            OpCode::AdjustRelativeBase => {
                let arg = self.get_argument_value(tape, 0)?;
                let relative_base = tape.get_relative_base() + arg;
                Ok(InstructionResult::Continue {
                    next_offset: default_next_offset,
                    relative_base: relative_base,
                })
            }
            OpCode::Terminate => Ok(InstructionResult::Terminate),
        }
    }
}

enum InstructionResult {
    Continue {
        next_offset: usize,
        relative_base: i64,
    },
    Terminate,
}

pub enum ProgramState {
    Running,
    Terminated,
}

pub struct Program {
    tape: Tape,
    pc: usize,
    state: ProgramState,
}

impl Program {
    pub fn new(tape: &Tape) -> Self {
        Self {
            tape: tape.clone(),
            pc: 0,
            state: ProgramState::Running,
        }
    }

    pub fn from_file(filename: &str) -> Result<Self> {
        let input = std::fs::read_to_string(filename)?;

        let mut ret = Vec::new();

        for num in input.trim().split(",").filter(|l| l.len() > 0) {
            ret.push(num.parse()?);
        }

        Ok(Self::new(&Tape::new(&ret)))
    }

    pub fn run_to_next_output(&mut self, inputs: &mut VecDeque<i64>) -> Result<Option<i64>> {
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
                InstructionResult::Continue {
                    next_offset,
                    relative_base,
                } => {
                    self.pc = next_offset;
                    self.tape.set_relative_base(relative_base);
                    if outputs.len() > starting_len {
                        break;
                    }
                }
                InstructionResult::Terminate => {
                    self.state = ProgramState::Terminated;
                    break;
                }
            }

            instruction_count += 1;
        }

        trace!("Ran {} instruction(s)", instruction_count);

        Ok(outputs.back().cloned())
    }

    pub fn run_to_next_input(&mut self, inputs: &mut VecDeque<i64>) -> Result<VecDeque<i64>> {
        let mut outputs = VecDeque::new();

        let mut instruction_count = 0;
        loop {
            let instruction = Instruction::new(&self.tape, self.pc)
                .with_context(|| format!("Failed to build instruction at offset {}", self.pc))?;

            if let OpCode::Input = instruction.opcode {
                if inputs.len() == 0 {
                    break;
                }
            }

            match instruction
                .run(&mut self.tape, inputs, &mut outputs)
                .with_context(|| format!("Failed to run instruction at offset {}", self.pc))?
            {
                InstructionResult::Continue {
                    next_offset,
                    relative_base,
                } => {
                    self.pc = next_offset;
                    self.tape.set_relative_base(relative_base);
                }
                InstructionResult::Terminate => {
                    self.state = ProgramState::Terminated;
                    break;
                }
            }

            instruction_count += 1;
        }

        trace!("Ran {} instruction(s)", instruction_count);

        Ok(outputs.clone())
    }

    pub fn run(&mut self, inputs: &mut VecDeque<i64>) -> Result<VecDeque<i64>> {
        // TODO(jsvana): make this not duplicated
        let mut outputs = VecDeque::new();

        loop {
            let instruction = Instruction::new(&self.tape, self.pc)
                .with_context(|| format!("Failed to build instruction at offset {}", self.pc))?;

            match instruction
                .run(&mut self.tape, inputs, &mut outputs)
                .with_context(|| format!("Failed to run instruction at offset {}", self.pc))?
            {
                InstructionResult::Continue {
                    next_offset,
                    relative_base,
                } => {
                    self.pc = next_offset;
                    self.tape.set_relative_base(relative_base);
                }
                InstructionResult::Terminate => {
                    self.state = ProgramState::Terminated;
                    break;
                }
            }
        }

        Ok(outputs)
    }

    pub fn get_state(&self) -> &ProgramState {
        &self.state
    }

    pub fn set_memory_value(&mut self, location: usize, value: i64) -> Result<()> {
        self.tape.set(location, value)?;

        Ok(())
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ret = Vec::new();

        for num in input.trim().split(",") {
            ret.push(num.parse()?);
        }

        Ok(Program::new(&Tape::new(&ret)))
    }
}
