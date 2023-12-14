use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    const DIFF_LEN: usize = 4;
    let mut set: HashSet<char> = HashSet::new();

    for i in 0..input.len() - DIFF_LEN {
        set.clear();
        set.extend(input.chars().skip(i).take(DIFF_LEN));
        if set.len() == DIFF_LEN {
            return i + DIFF_LEN;
        }
    }

    unreachable!();
}
