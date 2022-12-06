use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);
    let input = r
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No lines in input"))??
        .trim()
        .to_string();

    let output_1 = puzzle_1(input.as_bytes())?;
    println!("puzzle1: {output_1}");
    assert_eq!(1198, output_1);

    let output_2 = puzzle_2(input.as_bytes())?;
    println!("puzzle2: {output_2}");
    assert_eq!(3120, output_2);

    Ok(())
}

fn puzzle_1(input: &[u8]) -> Result<usize> {
    const MARKER_SIZE: usize = 4;

    for (i, window) in input.windows(MARKER_SIZE).enumerate() {
        if is_unique(window) {
            return Ok(i + 4);
        }
    }

    Err(anyhow!("No start-of-packet marker received"))
}

fn puzzle_2(input: &[u8]) -> Result<usize> {
    const MARKER_SIZE: usize = 14;

    for (i, window) in input.windows(MARKER_SIZE).enumerate() {
        if is_unique(window) {
            return Ok(i + 14);
        }
    }

    Err(anyhow!("No start-of-packet marker received"))
}

fn is_unique(window: &[u8]) -> bool {
    let mut set = HashSet::new();

    for el in window {
        if !set.insert(el) {
            return false;
        }
    }

    true
}
