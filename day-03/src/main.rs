use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file);
    contents
}

fn get_common_type(backpacks: &Vec<&str>) -> Option<char> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for backpack in backpacks.iter() {
        let set: HashSet<char> = HashSet::from_iter(backpack.chars());

        for item_type in set.iter() {
            map.entry(*item_type).and_modify(|t| *t += 1).or_insert(1);
        }
    }

    for (key, count) in map.iter() {
        if *count == 3 {
            return Some(*key);
        }
    }

    None
}

fn get_priority(item_type: &char) -> i32 {
    let char_code = *item_type as i32;
    if char_code >= 97 && char_code <= 122 {
        return char_code - 96;
    }

    if char_code >= 65 && char_code <= 90 {
        return char_code - 38;
    }

    0
}

fn main() {
    let contents = read_file("input.txt").expect("Error loading the input.txt file.");

    let mut item_types: Vec<char> = Vec::new();

    let mut group: Vec<&str> = Vec::new();

    for line in contents.lines() {
        group.push(line);
        if group.len() == 3 {
            println!("{:?}", group);
            match get_common_type(&group) {
                Some(item_type) => {
                    println!("Common: {}", item_type);
                    item_types.push(item_type);
                }
                None => println!("No common found"),
            }
            group.clear();
        }
    }

    let mut total = 0;
    for item_type in item_types.iter() {
        total += get_priority(item_type);
    }

    println!("Total: {}", total);
}
