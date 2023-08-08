use itertools::Itertools;
use std::{error::Error, fs, str::FromStr};

pub struct Monkey {
    pub id: usize,
    pub items: Vec<i64>,
    pub operation: Box<dyn Fn(i64) -> i64>,
    pub test_divisor: i64,
    pub true_monkey: usize,
    pub false_monkey: usize,
    pub inspections: usize,
}

impl Monkey {
    /// ```
    /// # use advent_of_code_2022::day11::Monkey;
    /// let monkey1 = Monkey {
    ///     id: 0,
    ///     items: vec![84, 72, 58, 51],
    ///     operation: Box::new(|x| x * 3),
    ///     test_divisor: 13,
    ///     true_monkey: 1,
    ///     false_monkey: 7,
    ///     inspections: 0,
    /// };
    /// let text = "\
    /// Monkey 0:
    /// Starting items: 84, 72, 58, 51
    /// Operation: new = old * 3
    /// Test: divisible by 13
    ///   If true: throw to monkey 1
    ///   If false: throw to monkey 7";
    /// let monkey2 = Monkey::from(text).unwrap();
    ///
    /// assert_eq!(monkey1.id, monkey2.id);
    /// assert_eq!(monkey1.items, monkey2.items);
    /// assert_eq!(monkey1.test_divisor, monkey2.test_divisor);
    /// assert_eq!(monkey1.true_monkey, monkey2.true_monkey);
    /// assert_eq!(monkey1.false_monkey, monkey2.false_monkey);
    /// assert_eq!(monkey1.inspect(6), monkey2.inspect(6));
    /// assert_eq!(monkey1.inspect(6), 18);
    pub fn from(text: &str) -> Result<Monkey, Box<dyn Error>> {
        let mut lines = text.lines();
        let err = "not enough lines";
        let id = find_int(lines.next().ok_or(err)?)?;
        let items: Vec<i64> = lines
            .next()
            .ok_or(err)?
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let operation = {
            match lines
                .next()
                .ok_or(err)?
                .split('=')
                .last()
                .ok_or("no operation")?
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .skip(1)
                .take(2)
                .collect_tuple::<(_, _)>()
                .ok_or("not enough arguments in operation")?
            {
                ("*", "old") => {
                    let f: Box<dyn Fn(i64) -> i64> = Box::new(|x| x * x);
                    f
                }
                ("+", "old") => {
                    let f: Box<dyn Fn(i64) -> i64> = Box::new(|x| x + x);
                    f
                }
                ("*", num) => {
                    let z: i64 = num.parse()?;
                    let f: Box<dyn Fn(i64) -> i64> = Box::new(move |x| x * z);
                    f
                }
                ("+", num) => {
                    let z: i64 = num.parse()?;
                    let f: Box<dyn Fn(i64) -> i64> = Box::new(move |x| x + z);
                    f
                }
                (..) => None.ok_or("not a valid operation")?,
            }
        };
        let test_divisor = find_int(lines.next().ok_or(err)?)?;
        let true_monkey = find_int(lines.next().ok_or(err)?)?;
        let false_monkey = find_int(lines.next().ok_or(err)?)?;
        Ok(Monkey {
            id,
            items,
            operation,
            test_divisor,
            true_monkey,
            false_monkey,
            inspections: 0,
        })
    }

    pub fn get_destination(&self, item: i64) -> usize {
        if item % self.test_divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }

    pub fn inspect(&self, item: i64) -> i64 {
        (self.operation)(item)
    }
}

/// ```
/// # use advent_of_code_2022::day11::find_int;
/// assert_eq!(find_int::<i32>("fweufwe334fewufw1").unwrap(), 334);
/// assert_eq!(find_int::<i32>("123").unwrap(), 123);
/// assert_eq!(find_int::<i32>("I am 1.0 years old").unwrap(), 1);
pub fn find_int<T>(line: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    Ok(line
        .split(|c: char| !c.is_numeric())
        .find(|s| !s.is_empty())
        .ok_or("no number found")?
        .parse::<T>()?)
}

pub fn round(monkeys: &mut [Monkey], divisor: i64, modulo: i64) {
    let mut new_items = vec![vec![]; monkeys.len()];
    for monkey in &mut *monkeys {
        monkey.inspections += monkey.items.len() + new_items[monkey.id].len();
        for &item in &new_items[monkey.id] {
            monkey.items.push(item);
        }
        new_items[monkey.id] = Vec::new();
        for &item in &monkey.items {
            let item = (monkey.inspect(item) % modulo) / divisor;
            new_items[monkey.get_destination(item)].push(item);
        }
    }
    for (monkey, items) in monkeys.iter_mut().zip(new_items) {
        monkey.items = items;
    }
}

/// ```
/// # use advent_of_code_2022::day11::puzzle1;
/// assert_eq!(puzzle1("day11a.txt").unwrap(), 10605);
pub fn puzzle1(path: &str) -> Result<usize, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in text.split("\n\r\n").map(Monkey::from) {
        monkeys.push(monkey?);
    }
    let modulo = monkeys.iter().map(|x| x.test_divisor).product();
    for _ in 0..20 {
        round(&mut monkeys, 3, modulo);
    }
    Ok(monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product())
}

pub fn puzzle2(path: &str) -> Result<usize, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in text.split("\n\r\n").map(Monkey::from) {
        monkeys.push(monkey?);
    }
    let modulo = monkeys.iter().map(|x| x.test_divisor).product();
    for _ in 0..10_000 {
        round(&mut monkeys, 1, modulo);
    }
    Ok(monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product())
}
