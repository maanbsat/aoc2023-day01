use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Iter;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

const PATH: &str = "input.txt";
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref DIGIT_MAPPING: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("1", 1);
        m.insert("two", 2);
        m.insert("2", 2);
        m.insert("three", 3);
        m.insert("3", 3);
        m.insert("four", 4);
        m.insert("4", 4);
        m.insert("five", 5);
        m.insert("5", 5);
        m.insert("six", 6);
        m.insert("6", 6);
        m.insert("seven", 7);
        m.insert("7", 7);
        m.insert("eight", 8);
        m.insert("8", 8);
        m.insert("nine", 9);
        m.insert("9", 9);
        // m.insert("0", 0);
        m
    };
}

// We create an iterator over matches of the regex on a given input line. This
// iterator looks for overlapping matches.
struct MatchIterator {
    line: String,
    curr: usize,
}

impl Iterator for MatchIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let m = RE.find(&self.line[self.curr..]);
        if let Some(r) = m {
            self.curr += r.start() + 1;
            return Some(*DIGIT_MAPPING.get(r.as_str()).unwrap());
        } else {
            return None;
        }
    }
}

fn line_matcher(line: &str) -> MatchIterator {
    MatchIterator {
        line: line.into(),
        curr: 0,
    }
}

fn get_code(line: &str) -> Result<usize> {
    let mut matcher = line_matcher(line);
    let first = matcher
        .next()
        .context(format!("Can't get first digit: {}", line))?;
    let last = matcher
        .last()
        .or(Some(first)) // if there isn't a second value, take the first one again
        .context(format!("Can't get last digit: {}", line))?;

    let res = first * 10 + last;
    println!("{} -> {} + {} -> {}", line, first, last, res);

    Ok(res)
}

fn main() -> Result<()> {
    let f = File::open(PATH)?;
    let lines = io::BufReader::new(f).lines();
    let mut res: usize = 0;
    for l in lines {
        res += get_code(&l?)?;
    }

    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(super::get_code("eighthree").unwrap(), 83);
        assert_eq!(super::get_code("sevenine").unwrap(), 79);
        assert_eq!(super::get_code("oneight").unwrap(), 18);
    }
}
