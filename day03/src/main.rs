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
        .map(|l| -> Result<Vec<char>> {
            Ok(l?.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        })
        .collect::<Result<Vec<Vec<char>>>>()?;

    let output_1 = puzzle_1(&input)?;
    println!("puzzle1: {output_1}");
    assert_eq!(8243, output_1);

    let output_2 = puzzle_2(&input)?;
    println!("puzzle2: {output_2}");
    assert_eq!(2631, output_2);

    Ok(())
}

fn puzzle_1(input: &[Vec<char>]) -> Result<u64> {
    let mut sum = 0;
    for content in input {
        let common_item = get_common_item(&[
            content[0..content.len() / 2].into(),
            content[content.len() / 2..content.len()].into(),
        ])?;

        sum += get_point(common_item)?;
    }

    Ok(sum)
}

fn puzzle_2(input: &[Vec<char>]) -> Result<u64> {
    let mut sum = 0;

    for group in input.chunks(3) {
        let common_item = get_common_item(group)?;

        sum += get_point(common_item)?;
    }

    Ok(sum)
}

fn get_common_item(groups: &[Vec<char>]) -> Result<char> {
    let sets = groups
        .iter()
        .map(|group| HashSet::<char>::from_iter(group.iter().cloned()))
        .collect::<Vec<_>>();

    let mut common = sets[0].clone();

    for set in sets.iter().skip(1) {
        common = common.intersection(set).cloned().collect();
    }

    common
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No intersection found!"))
}

fn get_point(c: char) -> Result<u64> {
    if c.is_ascii_lowercase() {
        Ok(c as u64 - 'a' as u64 + 1)
    } else if c.is_ascii_uppercase() {
        Ok(c as u64 - 'A' as u64 + 27)
    } else {
        Err(anyhow!("Invalid character"))
    }
}
