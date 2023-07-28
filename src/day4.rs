use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    pub fn from(start: i32, end: i32) -> Option<Range> {
        if start <= end {
            Some(Range { start, end })
        } else {
            None
        }
    }

    pub fn overlap(&self, other: &Range) -> Option<Range> {
        match (self.start.cmp(&other.end), self.end.cmp(&other.start)) {
            (Less, _) | (_, Greater) => None,
            (_, _) => match (self.start.cmp(&other.start), self.end.cmp(&other.end)) {
                (Greater | Equal, Less | Equal) => Some(*self),
                (Less | Equal, Greater | Equal) => Some(*other),
                (Greater, Greater) => Some(Range {
                    start: self.start,
                    end: other.end,
                }),
                (Less, Less) => Some(Range {
                    start: other.start,
                    end: self.end,
                }),
            },
        }
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

pub fn mutual_contains(x: &Range, y: &Range) -> bool {
    x.contains(y) || y.contains(x)
}

pub fn parse_line(line: &str) -> Option<(Range, Range)> {
    let mut halves = line.split(',');
    let (mut left, mut right) = (halves.next()?.split('-'), halves.next()?.split('-'));
    Some((
        Range::from(left.next()?.parse().unwrap(), left.next()?.parse().unwrap())?,
        Range::from(
            right.next()?.parse().unwrap(),
            right.next()?.parse().unwrap(),
        )?,
    ))
}

pub fn puzzle1(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut acc = 0;
    for line in fs::read_to_string(path)?.lines() {
        let Some((left, right)) = parse_line(line) else {
            return Err(format!("Could not parse line: {line}").into());
        };
        if mutual_contains(&left, &right) {
            acc += 1;
        }
    }
    Ok(acc)
}

pub fn puzzle2(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut acc = 0;
    for line in fs::read_to_string(path)?.lines() {
        let Some((left, right)) = parse_line(line) else {
            return Err(format!("Could not parse line: {line}").into());
        };
        if left.overlaps(&right) {
            acc += 1;
        }
    }
    Ok(acc)
}
