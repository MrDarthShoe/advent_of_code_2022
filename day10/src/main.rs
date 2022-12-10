use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);

    let input = r
        .lines()
        .map(|l| -> Result<Instruction> {
            let line = l?;
            let mut s = line.split_whitespace();

            match s.next() {
                Some("noop") => Ok(Instruction::Noop),
                Some("addx") => Ok(Instruction::AddX(
                    s.next()
                        .ok_or_else(|| anyhow!("Invalid addx value"))?
                        .parse::<i32>()?,
                )),
                _ => Err(anyhow!("Invalid operation")),
            }
        })
        .collect::<Result<Vec<Instruction>>>()?;

    let output_1 = puzzle_1(&input);
    println!("puzzle1: {output_1}");
    assert_eq!(15680, output_1);

    puzzle_2(input);

    Ok(())
}

fn puzzle_1(input: &[Instruction]) -> i32 {
    let mut sum = 0;

    let mut x = 1;
    let mut cycle = 1;

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                println!("{cycle} {x}, noop");
                if is_cycle_interesting(cycle) {
                    sum += cycle * x;
                }
                cycle += 1;
            }
            Instruction::AddX(value) => {
                println!("{cycle} {x}, nop {value}");
                if is_cycle_interesting(cycle) {
                    sum += cycle * x;
                }
                cycle += 1;
                println!("{cycle} {x}, addx {value}");
                if is_cycle_interesting(cycle) {
                    sum += cycle * x;
                }
                x += value;
                cycle += 1;
            }
        }
    }

    sum
}

fn puzzle_2(mut input: Vec<Instruction>) {
    input.reverse();

    let mut x = 1;
    let mut first_half = true;

    for cycle in 1..=240 {
        print_crt(cycle, x);
        if let Some(instruction) = input.last().cloned() {
            match instruction {
                Instruction::Noop => {
                    input.pop();
                }
                Instruction::AddX(value) => {
                    if first_half {
                        first_half = false
                    } else {
                        input.pop();
                        first_half = true;
                        x += value;
                    }
                }
            }
        }
    }
}

fn print_crt(cycle: i32, x: i32) {
    let cursor = (cycle - 1) % 40;
    if cursor == 0 {
        println!();
    }

    if cursor == x + 1 || cursor == x - 1 || cursor == x {
        print!("#");
    } else {
        print!(".");
    }
}

fn is_cycle_interesting(cycle: i32) -> bool {
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];

    interesting_cycles.contains(&cycle)
}
