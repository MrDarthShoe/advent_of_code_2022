use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);
    let input = r
        .lines()
        .map(|l| -> Result<(u64, u64, u64, u64)> {
            l?.split(&[',', '-'])
                .flat_map(|v| -> Result<u64> { Ok(v.parse::<u64>()?) })
                .next_tuple()
                .ok_or_else(|| anyhow!("Invalid input"))
        })
        .collect::<Result<Vec<(u64, u64, u64, u64)>>>()?;

    let output_1 = puzzle_1(&input);
    println!("puzzle1: {output_1}");
    assert_eq!(562, output_1);

    let output_2 = puzzle_2(&input);
    println!("puzzle2: {output_2}");
    assert_eq!(924, output_2);

    Ok(())
}

fn puzzle_1(input: &[(u64, u64, u64, u64)]) -> u64 {
    let mut sum_containing = 0;
    for (x, y, a, b) in input {
        if x <= a && y >= b || x >= a && y <= b {
            sum_containing += 1;
        }
    }

    sum_containing
}

fn puzzle_2(input: &[(u64, u64, u64, u64)]) -> u64 {
    let mut sum_overlapping = input.len() as u64;
    for (x, y, a, b) in input {
        if (a > x && a > y) || (b < x && b < y) {
            sum_overlapping -= 1;
        }
    }

    sum_overlapping
}
