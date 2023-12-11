use std::{collections::HashSet, fs};

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file);
    contents
}

fn get_assigments(assigments_str: &str) -> ((i32, i32), (i32, i32)) {
    let assigments: Vec<&str> = assigments_str.split(',').collect();

    let a: Vec<&str> = assigments[0].split('-').collect();
    let b: Vec<&str> = assigments[1].split('-').collect();

    let a1: i32 = a[0].parse().unwrap();
    let a2: i32 = a[1].parse().unwrap();
    let b1: i32 = b[0].parse().unwrap();
    let b2: i32 = b[1].parse().unwrap();

    ((a1, a2), (b1, b2))
}

fn is_overlapping(range1: (i32, i32), range2: (i32, i32)) -> bool {
    let mut locations: HashSet<i32> = HashSet::new();
    for i in range1.0..=range1.1 {
        locations.insert(i);
    }

    for i in range2.0..=range2.1 {
        locations.insert(i);
    }

    let expected_len = range1.1 - range1.0 + range2.1 - range2.0 + 2;

    (locations.len() as i32).lt(&expected_len)
}

fn main() {
    let file = "input.txt";

    let contents = read_file(file).expect("Error reading the input.txt file.");

    let mut overlapping_pairs_count = 0;

    for line in contents.lines() {
        let (elf2, elf1) = get_assigments(line);
        if is_overlapping(elf1, elf2) {
            overlapping_pairs_count += 1;
        }
    }

    println!("Number of overlapping pairs: {}", overlapping_pairs_count);
}
