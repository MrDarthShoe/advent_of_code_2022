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
        .map(|l| -> Result<(char, usize)> {
            let line = l?;
            let mut s = line.split_ascii_whitespace();
            let direction = s.next().ok_or_else(|| anyhow!("invalid char"))?.parse()?;
            let count = s.next().ok_or_else(|| anyhow!("invalid usize"))?.parse()?;
            Ok((direction, count))
        })
        .collect::<Result<Vec<(char, usize)>>>()?;

    let output_1 = puzzle_1(&input)?;
    println!("puzzle1: {output_1}");
    assert_eq!(5878, output_1);

    let output_2 = puzzle_2(&input)?;
    println!("puzzle2: {output_2}");
    assert_eq!(2405, output_2);

    Ok(())
}

fn puzzle_1(input: &[(char, usize)]) -> Result<usize> {
    let mut tail_visied: HashSet<(i32, i32)> = HashSet::new();

    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);

    tail_visied.insert(t);

    for &(direction, count) in input {
        for _ in 0..count {
            match direction {
                'R' => h = (h.0 + 1, h.1),
                'L' => h = (h.0 - 1, h.1),
                'U' => h = (h.0, h.1 + 1),
                'D' => h = (h.0, h.1 - 1),
                _ => return Err(anyhow!("invalid direction")),
            }

            t = move_tail(h, t);

            tail_visied.insert(t);
        }
    }

    Ok(tail_visied.len())
}

fn puzzle_2(input: &[(char, usize)]) -> Result<usize> {
    let mut tail_visied: HashSet<(i32, i32)> = HashSet::new();

    let mut rope = vec![(0, 0); 10];

    tail_visied.insert(rope[9]);

    for &(direction, count) in input {
        for _ in 0..count {
            match direction {
                'R' => rope[0] = (rope[0].0 + 1, rope[0].1),
                'L' => rope[0] = (rope[0].0 - 1, rope[0].1),
                'U' => rope[0] = (rope[0].0, rope[0].1 + 1),
                'D' => rope[0] = (rope[0].0, rope[0].1 - 1),
                _ => return Err(anyhow!("invalid direction")),
            }

            for i in 1..rope.len() {
                rope[i] = move_tail(rope[i - 1], rope[i]);
            }

            tail_visied.insert(rope[9]);
        }
    }

    Ok(tail_visied.len())
}

fn move_tail(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    let dx: i32 = h.0 - t.0;
    let dy: i32 = h.1 - t.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        t
    } else {
        (t.0 + dx.signum(), t.1 + dy.signum())
    }
}
