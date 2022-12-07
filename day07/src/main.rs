use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let f = File::open("input")?;

    let r = BufReader::new(f);

    let mut directories: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<String> = vec![];

    for line in r.lines() {
        let line = line?;
        let split = line.split(' ').collect::<Vec<_>>();

        match split[0] {
            "$" => match split[1] {
                "cd" => match split[2] {
                    ".." => {
                        current_path.pop();
                    }
                    directory => {
                        current_path.push(directory.into());
                        directories.insert(current_path.iter().cloned().collect::<String>(), 0);
                    }
                },
                "ls" => {}
                _ => {}
            },
            "dir" => {}
            size => {
                let mut current_key = String::new();
                for path in &current_path {
                    current_key.push_str(path);
                    if let Some(current_size) = directories.get_mut(&current_key) {
                        *current_size += size.parse::<usize>()?;
                    };
                }
            }
        }
    }

    let output_1 = puzzle_1(&directories);
    println!("puzzle1: {output_1}");
    assert_eq!(1141028, output_1);

    let output_2 = puzzle_2(&directories)?;
    println!("puzzle2: {output_2}");
    assert_eq!(8278005, output_2);

    Ok(())
}

fn puzzle_1(directories: &HashMap<String, usize>) -> usize {
    let mut sum = 0;
    for &size in directories.values() {
        if size <= 100_000 {
            sum += size
        }
    }
    sum
}

fn puzzle_2(directories: &HashMap<String, usize>) -> Result<usize> {
    let used = directories
        .get("/")
        .ok_or_else(|| anyhow!("unable to locate root"))?;

    let needed_space = 30_000_000 - (70_000_000 - used);

    let mut smallest = usize::MAX;

    for &size in directories.values() {
        if size > needed_space && size < smallest {
            smallest = size;
        }
    }

    Ok(smallest)
}
