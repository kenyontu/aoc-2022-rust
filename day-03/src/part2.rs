use std::collections::{HashMap, HashSet};

fn get_common_type(backpacks: &[&str]) -> Option<char> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for backpack in backpacks[0..2].iter() {
        let set: HashSet<char> = HashSet::from_iter(backpack.chars());

        for item_type in set.iter() {
            map.entry(*item_type).and_modify(|t| *t += 1).or_insert(1);
        }
    }

    for c in backpacks[2].chars() {
        if map.contains_key(&c) && map[&c] == 2 {
            return Some(c);
        }
    }

    None
}

fn get_priority(item_type: &char) -> u32 {
    let char_code = *item_type as u32;
    if char_code >= 97 && char_code <= 122 {
        return char_code - 96;
    }

    if char_code >= 65 && char_code <= 90 {
        return char_code - 38;
    }

    0
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|backpacks| {
            if let Some(common_char) = get_common_type(backpacks) {
                return get_priority(&common_char);
            }

            unreachable!();
        })
        .sum()
}
