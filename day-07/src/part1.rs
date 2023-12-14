use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> u32 {
    let mut dir: VecDeque<String> = VecDeque::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        if line == "$ cd /" {
            dir.clear();
            continue;
        }

        if line == "$ cd .." {
            dir.pop_back();
            continue;
        }

        if line.starts_with("$ cd") {
            let dir_name = line.split_whitespace().nth(2).unwrap();
            dir.push_back(String::from(dir_name));
            continue;
        }

        if line.chars().nth(0).unwrap().is_ascii_digit() {
            let (size_str, _) = line.split_once(' ').unwrap();
            let size = size_str.parse::<usize>().unwrap();

            for i in 0..dir.len() {
                let path = dir
                    .range(0..dir.len() - i)
                    .fold(String::new(), |mut acc, d| {
                        acc.push_str("/");
                        acc.push_str(&d);
                        acc
                    });

                dir_sizes
                    .entry(path)
                    .and_modify(|dir_size| *dir_size = *dir_size + size)
                    .or_insert(size);
            }
        }
    }

    dir_sizes
        .values()
        .filter(|size| **size <= 100_000)
        .sum::<usize>() as u32
}
