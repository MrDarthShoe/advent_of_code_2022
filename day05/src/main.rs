use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let mut stack_lines = String::new();

    let r = BufReader::new(f);
    let input = r
        .lines()
        .map(|l| -> Result<(u64, u64, u64, u64)> { l? })
        .collect::<Result<Vec<(u64, u64, u64, u64)>>>()?;

    // let output_1 = puzzle_1(&input);
    // println!("puzzle1: {output_1}");
    // assert_eq!(562, output_1);

    // let output_2 = puzzle_2(&input);
    // println!("puzzle2: {output_2}");
    // assert_eq!(924, output_2);

    Ok(())
}

struct CratesStack {
    s: Vec<char>,
}

struct Movement {
    crates_no: usize,
    from: usize,
    to: usize,
}
