use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);
    let input = r
        .lines()
        .map(|l| -> Result<Option<u64>> {
            let line = l?;
            if let Ok(v) = line.parse() {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<Option<u64>>>>()?;

    let output_1 = puzzle_1(&input);
    println!("puzzle1: {output_1}");

    let output_2 = puzzle_2(&input);
    println!("puzzle2: {output_2}");

    assert_eq!(69883, output_1);
    assert_eq!(207576, output_2);

    Ok(())
}

fn puzzle_1(input: &[Option<u64>]) -> u64 {
    let mut most_calories = 0;
    let mut elf_accumulator = 0;

    for v in input {
        match v {
            Some(calories) => {
                elf_accumulator += calories;
            }
            None => {
                if most_calories < elf_accumulator {
                    most_calories = elf_accumulator;
                }
                elf_accumulator = 0;
            }
        }
    }

    most_calories
}

fn puzzle_2(input: &[Option<u64>]) -> u64 {
    let mut most_calories_1 = 0;
    let mut most_calories_2 = 0;
    let mut most_calories_3 = 0;
    let mut elf_accumulator = 0;

    for v in input {
        match v {
            Some(calories) => {
                elf_accumulator += calories;
            }
            None => {
                if most_calories_1 < elf_accumulator {
                    most_calories_3 = most_calories_2;
                    most_calories_2 = most_calories_1;
                    most_calories_1 = elf_accumulator;
                } else if most_calories_2 < elf_accumulator {
                    most_calories_3 = most_calories_2;
                    most_calories_2 = elf_accumulator;
                } else if most_calories_3 < elf_accumulator {
                    most_calories_3 = elf_accumulator;
                }
                elf_accumulator = 0;
            }
        }
    }

    most_calories_1 + most_calories_2 + most_calories_3
}
