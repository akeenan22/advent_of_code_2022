use itertools::Itertools;
use std::{collections::VecDeque, error::Error, fs};

pub fn puzzle1(path: &str) -> Result<Option<usize>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut recent = VecDeque::new();
    for (i, c) in text.char_indices() {
        recent.push_front(c);
        if recent.len() > 4 {
            recent.pop_back();
            if recent.iter().all_unique() {
                return Ok(Some(i + 1));
            }
        }
    }
    Ok(None)
}

pub fn puzzle2(path: &str) -> Result<Option<usize>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut recent = VecDeque::new();
    for (i, c) in text.char_indices() {
        recent.push_front(c);
        if recent.len() > 14 {
            recent.pop_back();
            if recent.iter().all_unique() {
                return Ok(Some(i + 1));
            }
        }
    }
    Ok(None)
}
