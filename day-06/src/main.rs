use std::collections::HashMap;
use std::fs;

fn find_start_of_packet_marker(input: &str, distinct_count: usize) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = 0;

    let chars = input.chars().collect::<Vec<char>>();
    let mut map: HashMap<char, u32> = HashMap::new();

    while right < distinct_count {
        if let Some(c) = chars.get(right) {
            map.entry(*c).and_modify(|count| *count += 1).or_insert(1);
            right += 1;
        }
    }

    if map.len() == distinct_count {
        return Some(right);
    }

    right -= 1;

    while right < input.len() - 1 {
        // For each loop the sliding window slides a sigle position forward, meaning:
        // - The char at the left gets removed from the map
        // - The next char gets added to the map
        //
        // The hash map is used to keep track of the characters within the window and
        // how many times they appear
        let left_c = chars.get(left).unwrap();
        if let Some(count) = map.get(&left_c) {
            if *count > 1 {
                map.entry(*left_c).and_modify(|count| *count -= 1);
            } else {
                map.remove(&left_c);
            }
        }

        left += 1;
        right += 1;

        let right_c = chars.get(right).unwrap();
        map.entry(*right_c)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if map.len() == distinct_count {
            return Some(right + 1);
        }
    }

    None
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect("Error reading the input.txt file");

    if let Some(marker_idx) = find_start_of_packet_marker(&contents, 14) {
        println!("Marker found after position: {}", marker_idx);
    } else {
        println!("Could not find 4 different consecutive letters");
    }
}
