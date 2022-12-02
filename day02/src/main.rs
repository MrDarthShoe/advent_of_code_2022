use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Choice {
    Scissors,
    Paper,
    Rock,
}

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => return Err(anyhow!("Invalid choice")),
        })
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    my: Choice,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        let opponent = iter.next().unwrap().parse()?;
        let my = iter.next().unwrap().parse()?;

        Ok(Round { opponent, my })
    }
}

impl From<&NeedsToEnd> for Round {
    fn from(n: &NeedsToEnd) -> Self {
        match (&n.opponent, &n.outcome) {
            (Choice::Scissors, Outcome::Loose) => Round {
                opponent: Choice::Scissors,
                my: Choice::Paper,
            },
            (Choice::Scissors, Outcome::Draw) => Round {
                opponent: Choice::Scissors,
                my: Choice::Scissors,
            },
            (Choice::Scissors, Outcome::Win) => Round {
                opponent: Choice::Scissors,
                my: Choice::Rock,
            },
            (Choice::Paper, Outcome::Loose) => Round {
                opponent: Choice::Paper,
                my: Choice::Rock,
            },
            (Choice::Paper, Outcome::Draw) => Round {
                opponent: Choice::Paper,
                my: Choice::Paper,
            },
            (Choice::Paper, Outcome::Win) => Round {
                opponent: Choice::Paper,
                my: Choice::Scissors,
            },
            (Choice::Rock, Outcome::Loose) => Round {
                opponent: Choice::Rock,
                my: Choice::Scissors,
            },
            (Choice::Rock, Outcome::Draw) => Round {
                opponent: Choice::Rock,
                my: Choice::Rock,
            },
            (Choice::Rock, Outcome::Win) => Round {
                opponent: Choice::Rock,
                my: Choice::Paper,
            },
        }
    }
}

impl Round {
    fn outcome(&self) -> u64 {
        match (&self.opponent, &self.my) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => 6,
            (Choice::Scissors, Choice::Scissors)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Rock, Choice::Rock) => 3,
            (Choice::Scissors, Choice::Paper)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Rock, Choice::Scissors) => 0,
        }
    }

    fn my_shape(&self) -> u64 {
        match &self.my {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn get_score(&self) -> u64 {
        self.my_shape() + self.outcome()
    }
}

#[derive(Debug)]
struct NeedsToEnd {
    opponent: Choice,
    outcome: Outcome,
}

#[derive(Debug)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Outcome::Loose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(anyhow!("Invalid choice")),
        })
    }
}

impl FromStr for NeedsToEnd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        let opponent = iter.next().unwrap().parse()?;
        let outcome = iter.next().unwrap().parse()?;

        Ok(NeedsToEnd { opponent, outcome })
    }
}

fn puzzle_1(input: &[Round]) -> u64 {
    input.iter().map(|r| r.get_score()).sum()
}

fn puzzle_2(input: &[NeedsToEnd]) -> u64 {
    input.iter().map(|n| Round::from(n).get_score()).sum()
}

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);
    let (input_1, input_2): (Vec<_>, Vec<_>) = r
        .lines()
        .flat_map(|l| -> Result<(Round, NeedsToEnd)> {
            let line = l?;
            Ok((line.parse()?, line.parse()?))
        })
        .unzip();

    let output_1 = puzzle_1(&input_1);
    println!("puzzle1: {output_1}");
    assert_eq!(11475, output_1);

    let output_2 = puzzle_2(&input_2);
    println!("puzzle2: {output_2}");
    assert_eq!(16862, output_2);

    Ok(())
}
