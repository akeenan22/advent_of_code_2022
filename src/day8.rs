use std::{error::Error, fs};

/// ```
/// # use advent_of_code_2022::day8::vec_2d;
/// assert_eq!(vec_2d("13\n24"), vec![vec![1, 3], vec![2, 4]]);
pub fn vec_2d(text: &str) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    for row in text.lines() {
        rows.push(
            row.chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    rows
}

/// [Source](https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust)
/// ```
/// # use advent_of_code_2022::day8::transpose;
/// assert_eq!(transpose(&[vec![1, 2], vec![3, 4]]), vec![vec![1, 3], vec![2, 4]]);
pub fn transpose<T: Clone>(rect: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..rect[0].len())
        .map(|i| rect.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

/// ```
/// # use advent_of_code_2022::day8::hidden;
/// assert_eq!(hidden(&vec![3, 5, 2, 4, 1]), vec![false, false, true, false, false]);
pub fn hidden(line: &Vec<u8>) -> Vec<bool> {
    directional_hidden(line)
        .iter()
        .zip(directional_hidden(&reverse(line)).iter().rev())
        .map(|(&l, &r)| l && r)
        .collect()
}

pub fn directional_hidden(line: &Vec<u8>) -> Vec<bool> {
    assert!(!line.is_empty());
    let mut out = vec![false];
    let mut max = line[0];
    for &height in line.iter().skip(1) {
        out.push(height <= max);
        if height > max {
            max = height;
        }
    }
    out
}

pub fn view(line: &Vec<u8>) -> Vec<i32> {
    directional_view(line)
        .iter()
        .zip(directional_view(&reverse(line)).iter().rev())
        .map(|(l, r)| l * r)
        .collect()
}

/// ```
/// # use advent_of_code_2022::day8::directional_view;
/// assert_eq!(directional_view(&vec![3, 3, 5, 4, 9]), vec![0, 1, 2, 1, 4]);
/// assert_eq!(directional_view(&vec![0, 1, 2, 3, 4]), vec![0, 1, 2, 3, 4]);
/// assert_eq!(directional_view(&vec![0, 0, 0, 0, 0]), vec![0, 1, 1, 1, 1]);
pub fn directional_view(line: &Vec<u8>) -> Vec<i32> {
    assert!(!line.is_empty());
    let mut out = vec![0];
    line.iter().enumerate().skip(1).for_each(|(i, height)| {
        out.push(
            line.iter()
                .take(i)
                .skip(1)
                .rev()
                .take_while(|x| x < &height)
                .count() as i32
                + 1,
        );
    });
    out
}

/// ```
/// # use advent_of_code_2022::day8::reverse;
/// assert_eq!(reverse(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1])
pub fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    let mut reversed = Vec::new();
    v.iter().rev().for_each(|x| reversed.push(x.clone()));
    reversed
}

pub fn puzzle1(path: &str) -> Result<usize, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let vec_2d = vec_2d(&text);
    let transposed_hidden =
        transpose(&transpose(&vec_2d).iter().map(hidden).collect::<Vec<_>>());
    Ok(vec_2d
        .iter()
        .map(hidden)
        .zip(transposed_hidden.iter())
        .map(|(r, c)| {
            r.iter()
                .zip(c.iter())
                .map(|(&x, &y)| x && y)
                .filter(|&x| !x)
                .count()
        })
        .sum())
}

pub fn puzzle2(path: &str) -> Result<i32, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let vec_2d = vec_2d(&text);
    let transposed_views =
        transpose(&transpose(&vec_2d).iter().map(view).collect::<Vec<_>>());
    Ok(vec_2d
        .iter()
        .map(view)
        .zip(transposed_views.iter())
        .map(|(r, c)| r.iter().zip(c.iter()).map(|(x, y)| x * y).max().unwrap())
        .max()
        .unwrap())
}
