use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);

    let mut stack_lines = vec![];
    let mut movements = vec![];

    for line in r.lines() {
        let line = line?;

        let mut split = line.split(' ');
        if split.next() == Some("move") {
            movements.push(Movement {
                quantity: split.next().unwrap().parse().unwrap(),
                from: split.nth(1).unwrap().parse().unwrap(),
                to: split.nth(1).unwrap().parse().unwrap(),
            })
        } else {
            stack_lines.push(line);
        }
    }

    //pop empty line
    stack_lines.pop();

    let no_of_stacks = stack_lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; no_of_stacks];

    for line in stack_lines.into_iter().rev() {
        let line_bytes = line.as_bytes();

        for stack_no in 0..no_of_stacks {
            if line_bytes[(stack_no * 4) + 1].is_ascii_alphanumeric() {
                stacks[stack_no].push(line_bytes[(stack_no * 4) + 1] as char);
            }
        }
    }

    let output_1 = puzzle_1(stacks.clone(), &movements);
    println!("puzzle1: {output_1}");
    assert_eq!("FCVRLMVQP", &output_1);

    let output_2 = puzzle_2(stacks, &movements);
    println!("puzzle2: {output_2}");
    assert_eq!("RWLWGJGFD", &output_2);

    Ok(())
}

fn puzzle_1(mut stacks: Vec<Vec<char>>, movements: &[Movement]) -> String {
    for m in movements {
        for _ in 0..m.quantity {
            let cr = stacks[m.from - 1].pop();
            if let Some(cr) = cr {
                stacks[m.to - 1].push(cr);
            }
        }
    }

    let mut result = String::new();

    for mut stack in stacks {
        let cr = stack.pop();
        if let Some(cr) = cr {
            result.push(cr);
        }
    }
    result
}

fn puzzle_2(mut stacks: Vec<Vec<char>>, movements: &[Movement]) -> String {
    for m in movements {
        let mut intermediate_stack = vec![];
        for _ in 0..m.quantity {
            let cr = stacks[m.from - 1].pop();
            if let Some(cr) = cr {
                intermediate_stack.push(cr);
            }
        }

        for cr in intermediate_stack.into_iter().rev() {
            stacks[m.to - 1].push(cr);
        }
    }

    let mut result = String::new();

    for mut stack in stacks {
        let cr = stack.pop();
        if let Some(cr) = cr {
            result.push(cr);
        }
    }
    result
}

struct Movement {
    quantity: usize,
    from: usize,
    to: usize,
}
