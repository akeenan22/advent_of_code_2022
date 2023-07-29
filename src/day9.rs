use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Default)]
pub struct Tracker {
    pos: Pos,
    set: HashSet<Pos>,
}

impl Tracker {
    pub fn new(pos: Pos) -> Tracker {
        let mut set = HashSet::new();
        set.insert(pos.clone());
        Tracker { pos, set }
    }

    pub fn follow(&mut self, other: &Pos) {
        self.pos.follow_set(other, &mut self.set);
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Default, Ord, PartialOrd)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn do_move(&mut self, dir: &str, dist: i32) {
        match dir {
            "R" => self.x += dist,
            "L" => self.x -= dist,
            "U" => self.y += dist,
            "D" => self.y -= dist,
            _ => {}
        }
    }

    pub fn is_in_tension(&self, other: &Pos) -> bool {
        self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2) > 2
    }

    pub fn follow_set(&mut self, other: &Pos, set: &mut HashSet<Pos>) {
        set.insert(self.clone());
        if self.is_in_tension(other) {
            match self.x.cmp(&other.x) {
                Greater => self.x -= 1,
                Less => self.x += 1,
                Equal => {}
            }
            match self.y.cmp(&other.y) {
                Greater => self.y -= 1,
                Less => self.y += 1,
                Equal => {}
            }
            self.follow_set(other, set);
        }
    }
    pub fn follow(&mut self, other: &Pos) {
        if self.is_in_tension(other) {
            match self.x.cmp(&other.x) {
                Greater => self.x -= 1,
                Less => self.x += 1,
                Equal => {}
            }
            match self.y.cmp(&other.y) {
                Greater => self.y -= 1,
                Less => self.y += 1,
                Equal => {}
            }
            self.follow(other);
        }
    }
}

pub fn parse(line: &str) -> Option<(String, i32)> {
    let (dir, dist) = line.split_whitespace().collect_tuple()?;
    let dist = dist.parse::<i32>().ok()?;
    match dir {
        "R" | "L" | "U" | "D" => Some((dir.to_string(), dist)),
        _ => None,
    }
}

/// ```
/// # use advent_of_code_2022::day9::puzzle1;
/// assert_eq!(puzzle1("day9a.txt").unwrap(), 10usize);
/// assert_eq!(puzzle1("day9b.txt").unwrap(), 13usize);
/// assert_eq!(puzzle1("day9.txt").unwrap(), 6464usize);
pub fn puzzle1(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut head = Pos::default();
    let mut tail = Tracker::default();
    for line in fs::read_to_string(path)?.lines().map(parse) {
        let (dir, dist) = line.ok_or("Invalid move")?;
        head.do_move(&dir, dist);
        tail.follow(&head);
    }
    Ok(tail.set.len())
}

/// ```
/// # use advent_of_code_2022::day9::puzzle2;
/// assert_eq!(puzzle2("day9b.txt").unwrap(), 1usize);
/// assert_eq!(puzzle2("day9c.txt").unwrap(), 36usize);
/// assert_eq!(puzzle2("day9.txt").unwrap(), 2604usize);
pub fn puzzle2(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut head = Vec::new();
    for _ in 0..9 {
        head.push(Pos::default());
    }
    let mut tail = Tracker::default();
    for line in fs::read_to_string(path)?.lines().map(parse) {
        let (dir, dist) = line.ok_or("Invalid move")?;
        for _ in 0..dist {
            head[0].do_move(&dir, 1);
            for i in 1..9 {
                let temp = head[i - 1].clone();
                head[i].follow(&temp);
            }
            tail.follow(&head[8]);
        }
    }
    Ok(tail.set.len())
}
