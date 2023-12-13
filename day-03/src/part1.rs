use std::collections::HashSet;

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

fn find_common(a: &str, b: &str) -> Option<char> {
    let set: HashSet<char> = HashSet::from_iter(a.chars());

    for c in b.chars() {
        if set.contains(&c) {
            return Some(c);
        }
    }

    None
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let split_at = l.len() / 2;
            if let Some(common_char) = find_common(&l[0..split_at], &l[split_at..]) {
                return get_priority(&common_char);
            }

            unreachable!();
        })
        .sum()
}
