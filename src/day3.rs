use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub fn priority(letter: char) -> Option<i32> {
    if letter.is_ascii_alphabetic() {
        Some(
            letter.to_digit(36)? as i32 - 9
                + if letter.is_uppercase() { 26 } else { 0 },
        )
    } else {
        None
    }
}

pub fn find_duplicate(line: &str) -> Option<char> {
    let mut letters = HashSet::new();
    let length = line.len();
    line[..length / 2].chars().for_each(|c| {
        letters.insert(c);
    });
    line[length / 2..].chars().find(|c| letters.contains(c))
}

pub fn find_shared(x: &str, y: &str, z: &str) -> Option<char> {
    let mut x_letters = HashSet::new();
    let mut x_y_letters = HashSet::new();

    x.chars().for_each(|c| {
        x_letters.insert(c);
    });

    y.chars().filter(|c| x_letters.contains(c)).for_each(|c| {
        x_y_letters.insert(c);
    });

    z.chars().find(|c| x_y_letters.contains(c))
}

pub fn puzzle1(path: &str) -> Result<i32, Box<dyn Error>> {
    Ok({
        let mut acc = 0;
        for line in fs::read_to_string(path)?.lines() {
            acc += match find_duplicate(line).map(priority) {
                Some(Some(x)) => Ok(x),
                Some(None) => Err(format!("{line}: Not a letter!")),
                None => Err(format!("{line}: No duplicates!")),
            }?;
        }
        acc
    })
}

pub fn puzzle2(path: &str) -> Result<i32, Box<dyn Error>> {
    Ok({
        let mut acc = 0;
        let text = fs::read_to_string(path)?;
        let mut lines = text.lines();
        while let (Some(x), Some(y), Some(z)) =
            (lines.next(), lines.next(), lines.next())
        {
            acc += match find_shared(x, y, z).map(priority) {
                Some(Some(x)) => Ok(x),
                Some(None) => Err(format!("{x}, {y}, {z}: Not a letter!")),
                None => Err(format!("{x}, {y}, {z}: No shared letters!")),
            }?;
        }
        acc
    })
}

#[cfg(test)]
pub mod tests {
    use crate::day3;

    #[test]
    fn can_prioritize() {
        assert_eq!(Some(1), day3::priority('a'));
        assert_eq!(Some(26), day3::priority('z'));
        assert_eq!(Some(27), day3::priority('A'));
        assert_eq!(Some(52), day3::priority('Z'));
    }
}
