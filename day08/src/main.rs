use anyhow::Result;
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
        .map(|l| -> Result<Vec<u8>> { Ok(l?.as_bytes().iter().map(|b| b - b'0').collect()) })
        .collect::<Result<Vec<Vec<u8>>>>()?;

    let output_1 = puzzle_1(&input);
    println!("puzzle1: {output_1}");
    assert_eq!(1763, output_1);

    let output_2 = puzzle_2(&input);
    println!("puzzle2: {output_2}");
    assert_eq!(671160, output_2);

    Ok(())
}

fn puzzle_1(input: &[Vec<u8>]) -> usize {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let x_size = input.len();
    let y_size = input[0].len();

    //from left
    for x in 0..x_size {
        let mut highest = -1;
        for y in 0..y_size {
            if input[x][y] as i32 > highest {
                visible.insert((x, y));
                highest = input[x][y] as i32;
            }
            if input[x][y] == 9 {
                break;
            }
        }
    }

    //from right
    for x in 0..x_size {
        let mut highest = -1;
        for y in (0..y_size).rev() {
            if input[x][y] as i32 > highest {
                visible.insert((x, y));
                highest = input[x][y] as i32;
            }
            if input[x][y] == 9 {
                break;
            }
        }
    }

    //from top
    for y in 0..y_size {
        let mut highest = -1;
        for x in 0..y_size {
            if input[x][y] as i32 > highest {
                visible.insert((x, y));
                highest = input[x][y] as i32;
            }
            if input[x][y] == 9 {
                break;
            }
        }
    }

    //from bottom
    for y in 0..y_size {
        let mut highest = -1;
        for x in (0..y_size).rev() {
            if input[x][y] as i32 > highest {
                visible.insert((x, y));
                highest = input[x][y] as i32;
            }
            if input[x][y] == 9 {
                break;
            }
        }
    }

    visible.len()
}

fn puzzle_2(input: &[Vec<u8>]) -> usize {
    let mut max_scenic = 0;

    let x_size = input.len();
    let y_size = input[0].len();

    for x in 0..x_size {
        for y in 0..y_size {
            let mut r = 0;
            for dy in y + 1..y_size {
                r += 1;

                if input[x][dy] >= input[x][y] {
                    break;
                }
            }

            let mut l = 0;
            for dy in (0..y).rev() {
                l += 1;

                if input[x][dy] >= input[x][y] {
                    break;
                }
            }

            let mut d = 0;
            for dx in x + 1..x_size {
                d += 1;

                if input[dx][y] >= input[x][y] {
                    break;
                }
            }

            let mut u = 0;
            for dx in (0..x).rev() {
                u += 1;

                if input[dx][y] >= input[x][y] {
                    break;
                }
            }

            max_scenic = max_scenic.max(r * l * u * d);
        }
    }

    max_scenic
}
