use std::{error::Error, fs, num::ParseIntError};
use thiserror;
use InstructionParseError as IPE;

#[derive(Clone, PartialEq, Eq)]
pub enum Instruction {
    Add(i32),
    Noop,
}

#[derive(Debug, thiserror::Error)]
pub enum InstructionParseError {
    #[error("not enough arguments in line to parse instruction")]
    NotEnoughArguments,
    #[error("instruction {0} not valid, use addx [int] or noop")]
    InvalidInstruction(String),
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
}

impl TryFrom<&str> for Instruction {
    type Error = InstructionParseError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut args = line.split_whitespace();
        match args.next().ok_or(IPE::NotEnoughArguments)? {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Add(
                args.next().ok_or(IPE::NotEnoughArguments)?.parse()?,
            )),
            x => Err(IPE::InvalidInstruction(x.to_string())),
        }
    }
}

#[derive(Clone)]
pub struct Computer {
    pub x: i32,
    pub cycle: i32,
    instruction: Instruction,
    wait: bool,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            x: 1,
            cycle: 0,
            instruction: Instruction::Noop,
            wait: false,
        }
    }
}

impl Computer {
    pub fn set_instruction(&mut self, instruction: Instruction) {
        self.wait = false;
        if let Instruction::Add(_) = instruction {
            self.wait = true;
        }
        self.instruction = instruction;
    }

    pub fn cycle_clock(&mut self) -> bool {
        self.cycle += 1;
        match self.instruction {
            Instruction::Add(x) => {
                if self.wait {
                    self.wait = false;
                    false
                } else {
                    self.x += x;
                    true
                }
            }
            Instruction::Noop => true,
        }
    }
}

pub fn puzzle1(path: &str) -> Result<i32, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut instructions = text.lines().map(Instruction::try_from);
    let mut computer = Computer::default();
    let mut sum = 0;
    for i in 0..=220 {
        if (i - 20) % 40 == 0 {
            sum += i * computer.x;
        }
        if computer.cycle_clock() {
            computer.set_instruction(
                instructions.next().ok_or("not enough instructions")??,
            );
        }
    }
    Ok(sum)
}

pub fn puzzle2(path: &str) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut instructions = text.lines().map(Instruction::try_from);
    let mut computer = Computer::default();
    for i in 0..=240 {
        if i != 0 {
            print!("{}", {
                if ((i - 1) % 40i32).abs_diff(computer.x) <= 1 {
                    "@" // # in problem desc. but @ looks nicer
                } else {
                    " " // . in problem desc. but space is more legible
                }
            });
            if i % 40 == 0 {
                println!();
            }
        }

        if i < 240 && computer.cycle_clock() {
            computer.set_instruction(
                instructions.next().ok_or("not enough instructions")??,
            );
        }
    }
    Ok(())
}
