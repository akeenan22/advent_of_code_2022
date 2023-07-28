use std::error::Error;
use std::fs;

#[derive(Default, Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct Elf {
    pub cals: i32,
}

impl Elf {
    pub fn add(&mut self, amount: i32) {
        self.cals += amount;
    }

    pub fn new() -> Elf {
        Elf { cals: 0 }
    }

    pub fn from(cals: i32) -> Elf {
        Elf { cals }
    }
}

pub fn ints_to_elves(ints: Vec<Option<i32>>) -> Vec<Elf> {
    let mut elves = vec![Elf::new()];

    for int in ints {
        match int {
            Some(num) => match elves.last_mut() {
                Some(elf) => elf.add(num),
                None => continue,
            },
            None => elves.push(Elf::new()),
        }
    }
    elves
}

pub fn parse_lines(input: &str) -> Vec<Option<i32>> {
    let mut output = Vec::new();
    for line in input.lines() {
        match line.parse() {
            Ok(x) => output.push(Some(x)),
            Err(_) => output.push(None),
        }
    }
    output
}

pub fn max_elf(elves: &[Elf]) -> Elf {
    elves.iter().max().unwrap().clone()
}

pub fn top_3_elves(elves: Vec<Elf>) -> Result<Vec<Elf>, &'static str> {
    let mut start = elves;
    let mut end = Vec::new();
    start.sort();
    for _ in 0..3 {
        end.push(match start.pop() {
            Some(x) => x,
            None => return Err("Not enough elves"),
        });
    }
    Ok(end)
}

pub fn sum_elves(elves: Vec<Elf>) -> i32 {
    elves
        .into_iter()
        .reduce(|mut acc, e| {
            acc.add(e.cals);
            acc
        })
        .unwrap()
        .cals
}

pub fn puzzle1(path: &str) -> Result<i32, Box<dyn Error>> {
    Ok(max_elf(&ints_to_elves(parse_lines(&fs::read_to_string(path)?))).cals)
}

pub fn puzzle2(path: &str) -> Result<i32, Box<dyn Error>> {
    Ok(sum_elves(top_3_elves(ints_to_elves(parse_lines(
        &fs::read_to_string(path)?,
    )))?))
}

#[cfg(test)]
pub mod tests {
    use crate::day1::{self, Elf};

    #[test]
    fn can_parse_lines() {
        let t = "\
1
2

3
4

5
6
7";
        assert_eq!(
            day1::parse_lines(t),
            vec![
                Some(1),
                Some(2),
                None,
                Some(3),
                Some(4),
                None,
                Some(5),
                Some(6),
                Some(7)
            ]
        );
    }

    #[test]
    fn can_get_elves() {
        let nums = vec![
            Some(1),
            Some(2),
            None,
            Some(3),
            Some(4),
            None,
            Some(5),
            Some(6),
            Some(7),
        ];
        assert_eq!(
            day1::ints_to_elves(nums),
            vec![Elf::from(3), Elf::from(7), Elf::from(18)]
        );
    }

    #[test]
    fn can_find_max_elf() {
        assert_eq!(
            day1::max_elf(&[Elf::from(9), Elf::from(12), Elf::from(0)]),
            Elf::from(12)
        );
    }

    #[test]
    fn can_find_top_3() {
        assert_eq!(
            day1::top_3_elves(vec![
                Elf::from(9),
                Elf::from(12),
                Elf::from(0),
                Elf::from(15),
                Elf::from(3)
            ]),
            Ok(vec![Elf::from(15), Elf::from(12), Elf::from(9)])
        );
    }

    #[test]
    fn can_sum_elves() {
        assert_eq!(
            day1::sum_elves(vec![Elf::from(15), Elf::from(12), Elf::from(9)]),
            36
        );
    }
}
