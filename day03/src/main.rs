use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);
    let input = r
        .lines()
        .map(|l| -> Result<Vec<char>> {
            Ok(l?.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        })
        .collect::<Result<Vec<Vec<char>>>>()?;

    let output_1 = puzzle_1(&input)?;
    println!("puzzle1: {output_1}");
    assert_eq!(8243, output_1);

    // let output_2 = puzzle_2(&input);
    // println!("puzzle2: {output_2}");
    // assert_eq!(207576, output_2);

    Ok(())
}

fn puzzle_1(input: &[Vec<char>]) -> Result<u64> {
    let mut sum = 0;
    for content in input {
        'outer: for i in 0..content.len() / 2 {
            for j in content.len() / 2..content.len() {
                if content[i] == content[j] {
                    sum += get_point(content[i])?;

                    break 'outer;
                }
            }
        }
    }

    Ok(sum)
}

fn get_point(c: char) -> Result<u64> {
    if c.is_ascii_lowercase() {
        Ok(c as u64 - 96)
    } else if c.is_uppercase() {
        Ok(c as u64 - 38)
    } else {
        Err(anyhow!("Invalid character"))
    }
}
