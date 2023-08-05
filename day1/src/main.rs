use std::fs;

fn main() {
    let file = "input.txt";

    let mut text = fs::read_to_string(file).expect("input.txt file not found.");

    // We add a new line to force the for loop bellow to loop one more time
    // and check the sum of the last elf
    text.push_str("\n");

    let mut top_sums: [u32; 4] = [0, 0, 0, 0];
    let mut sum: u32 = 0;

    for line in text.lines() {
        if line.len() > 0 {
            sum += match line.parse() {
                Ok(v) => v,
                Err(_) => 0,
            };
            continue;
        }

        // Using a "reversed bubble sort" to keep the [top_sums] array
        // sorted in a single pass

        // The current sum is placed in the last position and swapped with the
        // previous item if it is greater
        top_sums[3] = sum;

        let mut i = 3;
        while i > 0_usize && top_sums[i - 1] < top_sums[i] {
            let temp = top_sums[i - 1];
            top_sums[i - 1] = top_sums[i];
            top_sums[i] = temp;
            i -= 1;
        }

        // Reset the calories sum before proceeding to the next elf
        sum = 0;
    }

    // Only get the sum of the 3 first numbers
    let mut all_sum = 0;
    for i in 0..3 {
        all_sum += top_sums[i];
    }

    println!("Sum: {all_sum}");
}
