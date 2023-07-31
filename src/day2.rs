use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn score(&self, that: &RPS) -> i32 {
        match self {
            RPS::Rock => {
                1 + match that {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                }
            }
            RPS::Paper => {
                2 + match that {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0,
                }
            }
            RPS::Scissors => {
                3 + match that {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3,
                }
            }
        }
    }

    pub fn from(letter: char) -> Option<RPS> {
        match letter {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }
}

pub fn parse_line(line: &str) -> Result<(RPS, RPS), &'static str> {
    let mut iterator = line
        .chars()
        .filter(|x| "ABCXYZ".contains(*x))
        .map(|x| RPS::from(x).unwrap());
    Ok((
        match iterator.next() {
            Some(x) => x,
            None => return Err("Not enough arguments in line!"),
        },
        match iterator.next() {
            Some(x) => x,
            None => return Err("Not enough arguments in line!"),
        },
    ))
}

pub fn score_line(line: &str) -> Result<i32, &'static str> {
    let mut iterator = line.chars().filter(|x| "ABCXYZ".contains(*x));
    let (opponent, player) = (iterator.next(), iterator.next());
    let (Some(opponent), Some(player)) = (opponent, player) else {
        return Err("Not enough arguments in line.");
    };
    Ok(match (opponent, player) {
        ('A', 'X') => 3,
        ('B', 'X') => 1,
        ('C', 'X') => 2,
        ('A', 'Y') => 4,
        ('B', 'Y') => 5,
        ('C', 'Y') => 6,
        ('A', 'Z') => 8,
        ('B', 'Z') => 9,
        ('C', 'Z') => 7,
        _ => return Err("Not a valid turn."),
    })
}

pub fn score(line: &(RPS, RPS)) -> i32 {
    line.1.score(&line.0)
}

pub fn puzzle1(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for line in fs::read_to_string(path)?.lines() {
        sum += score(&parse_line(line)?);
    }
    Ok(sum)
}

pub fn puzzle2(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    for line in fs::read_to_string(path)?.lines() {
        sum += score_line(line)?;
    }
    Ok(sum)
}

#[cfg(test)]
pub mod tests {
    use crate::day2::{self, RPS};

    #[test]
    fn score_game() {
        assert_eq!(9, RPS::Scissors.score(&RPS::Paper));
    }

    #[test]
    fn can_score() {
        assert_eq!(8, day2::score(&(RPS::Rock, RPS::Paper)));
    }

    #[test]
    fn can_parse_line() {
        assert_eq!(
            (RPS::Rock, RPS::Scissors),
            day2::parse_line("A Z").unwrap()
        );
    }

    #[test]
    fn cannot_parse_invalid_line() {
        assert_eq!(
            Err("Not enough arguments in line!"),
            day2::parse_line("FGA432HL KNM43")
        );
    }
}
