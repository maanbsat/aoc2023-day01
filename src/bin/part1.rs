use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};

const PATH: &str = "input.txt";

fn get_code(line: &str) -> Result<usize> {
    let first = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .context("can't find first digit")?;
    // println!("{}", first);
    let last = line
        .chars()
        .rfind(|c| c.is_ascii_digit())
        .context("can't find last digit")?;
    let ret = format!("{}{}", first, last);
    // println!("{}", ret);
    Ok(ret.parse()?)
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
