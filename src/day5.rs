use std::collections::VecDeque;
use std::error::Error;
use std::fs;

/// ```
/// # use advent_of_code_2022::day5::parse_cargo;
/// assert_eq!(parse_cargo("\
/// [A]     [D]
/// [B] [C] [E]
///  1   2   3"),
/// vec![vec!['B', 'A'], vec!['C'], vec!['E', 'D']]
/// );
/// ```
pub fn parse_cargo(cargo: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let mut lines = cargo.lines().rev();
    if let Some(x) = lines.next() {
        x.split_whitespace().for_each(|_| stacks.push(Vec::new()));
    }
    lines.for_each(|x| {
        x.char_indices()
            .filter(|(i, c)| i % 4 == 1 && *c != ' ')
            .for_each(|(i, c)| {
                stacks[i / 4].push(c);
            });
    });
    stacks
}

/// ```
/// # use advent_of_code_2022::day5::parse_instruction;
/// assert_eq!(parse_instruction("move 1 from 6 to 2").unwrap(), (1, 6, 2));
/// ```
pub fn parse_instruction(
    instruction: &str,
) -> Result<(i32, usize, usize), Box<dyn Error>> {
    let mut iter = instruction
        .split_whitespace()
        .filter(|&s| s.chars().next().unwrap_or(' ').is_ascii_digit());
    let err = "Could not parse instruction";
    Ok((
        iter.next().ok_or(err)?.parse()?,
        iter.next().ok_or(err)?.parse()?,
        iter.next().ok_or(err)?.parse()?,
    ))
}

pub fn perform_instruction(
    instruction: &str,
    cargo: &mut [Vec<char>],
) -> Result<(), Box<dyn Error>> {
    let (amount, from, to) = parse_instruction(instruction)?;
    let err = "tried to remove from empty stack";
    for _ in 0..amount {
        let top = cargo[from - 1].pop().ok_or(err)?;
        cargo[to - 1].push(top);
    }
    Ok(())
}

pub fn perform_instruction_grouped(
    instruction: &str,
    cargo: &mut [Vec<char>],
) -> Result<(), Box<dyn Error>> {
    let (amount, from, to) = parse_instruction(instruction)?;
    let err = "tried to remove from empty stack";
    let mut temp = VecDeque::new();
    for _ in 0..amount {
        temp.push_front(cargo[from - 1].pop().ok_or(err)?);
    }
    for item in temp {
        cargo[to - 1].push(item);
    }
    Ok(())
}

pub fn get_tops(cargo: &mut [Vec<char>]) -> Result<String, Box<dyn Error>> {
    let mut tops = String::new();
    for stack in cargo {
        tops.push(*stack.last().ok_or("empty stack")?);
    }
    Ok(tops)
}

/// ```
/// # use advent_of_code_2022::day5::puzzle1;
/// assert_eq!(puzzle1("day5.txt").unwrap(), "NTWZZWHFV");
pub fn puzzle1(path: &str) -> Result<String, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let err = "need to separate cargo and instructions with a new line";
    let separator = text.find("\n\r\n").ok_or(err)?;
    let cargo = &text[..separator];
    let instructions = &text[separator + 3..];
    let mut cargo = parse_cargo(cargo);
    for line in instructions.lines() {
        perform_instruction(line, &mut cargo)?;
    }
    get_tops(&mut cargo)
}

/// ```
/// # use advent_of_code_2022::day5::puzzle2;
/// assert_eq!(puzzle2("day5.txt").unwrap(), "BRZGFVBTJ");
pub fn puzzle2(path: &str) -> Result<String, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let err = "need to separate cargo and instructions with a new line";
    let separator = text.find("\n\r\n").ok_or(err)?;
    let cargo = &text[..separator];
    let instructions = &text[separator + 3..];
    let mut cargo = parse_cargo(cargo);
    for line in instructions.lines() {
        perform_instruction_grouped(line, &mut cargo)?;
    }
    get_tops(&mut cargo)
}
