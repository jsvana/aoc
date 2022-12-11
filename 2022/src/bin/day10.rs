use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Clone, Debug)]
enum Instruction {
    NoOp,
    AddX(i64),
}

impl Instruction {
    fn name(&self) -> String {
        match self {
            Instruction::NoOp => "noop".to_string(),
            Instruction::AddX(_) => "addx".to_string(),
        }
    }

    fn cycle_count(&self) -> usize {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn new_x_value(&self, current_x: i64) -> i64 {
        match self {
            Instruction::NoOp => current_x,
            Instruction::AddX(amount) => current_x + amount,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let command = parts
            .get(0)
            .ok_or_else(|| anyhow!("Instruction \"{}\" missing command", value))?;

        match command {
            &"noop" => Ok(Instruction::NoOp),
            &"addx" => {
                let x: i64 = parts
                    .get(1)
                    .ok_or_else(|| anyhow!("addx missing value"))?
                    .parse()?;
                Ok(Instruction::AddX(x))
            }
            _ => Err(anyhow!("Unknown command for instruction \"{}\"", value)),
        }
    }
}

struct RunningInstruction {
    instruction: Instruction,
    ticks_left: usize,
}

struct VirtualMachine {
    x: i64,
    cycle: usize,
    current_instruction: Option<RunningInstruction>,
}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            x: 1,
            cycle: 1,
            current_instruction: None,
        }
    }

    fn set_next_instruction(&mut self, instruction: Instruction) -> Result<()> {
        match self.current_instruction {
            Some(_) => Err(anyhow!("Instruction already running")),
            None => {
                let ticks = instruction.cycle_count();
                self.current_instruction = Some(RunningInstruction {
                    instruction,
                    ticks_left: ticks,
                });

                Ok(())
            }
        }
    }

    fn has_instruction(&self) -> bool {
        self.current_instruction.is_some()
    }

    fn last_cycle(&self) -> usize {
        self.cycle - 1
    }

    fn pixel(&self) -> char {
        let cycle = self.last_cycle() as i64 % 40;
        if cycle >= self.x - 1 && cycle <= self.x + 1 {
            '#'
        } else {
            '.'
        }
    }

    fn tick(&mut self) -> Result<i64> {
        self.cycle += 1;

        match self.current_instruction.as_mut() {
            Some(running_instruction) => {
                let x = self.x;
                running_instruction.ticks_left -= 1;
                if running_instruction.ticks_left == 0 {
                    self.x = running_instruction.instruction.new_x_value(self.x);
                    self.current_instruction = None;
                }
                Ok(x)
            }
            None => {
                return Err(anyhow!("No instruction to execute"));
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let mut instructions = Vec::new();
    for line in std::fs::read_to_string(&args.filename)?.lines() {
        instructions.push(line.parse::<Instruction>()?);
    }

    let mut total = 0;
    let mut vm = VirtualMachine::new();

    for instruction in instructions.iter() {
        vm.set_next_instruction(instruction.clone());
        while vm.has_instruction() {
            let x = vm.tick()?;
            let cycle = vm.last_cycle();

            if cycle == 20
                || cycle == 60
                || cycle == 100
                || cycle == 140
                || cycle == 180
                || cycle == 220
            {
                total += x * cycle as i64;
            }
        }
    }

    println!("{}", total);

    let mut vm = VirtualMachine::new();

    for instruction in instructions {
        vm.set_next_instruction(instruction);
        while vm.has_instruction() {
            vm.tick()?;

            let cycle = vm.last_cycle();

            print!("{}", vm.pixel());

            if cycle % 40 == 0 {
                println!("");
            }
        }
    }

    Ok(())
}
