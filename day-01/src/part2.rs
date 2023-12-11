pub fn solve(input: &str) -> u32 {
    let mut calories_by_elf = input
        .split_terminator("\n\n")
        .map(|lines_by_elf| {
            lines_by_elf
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();

    calories_by_elf.sort();

    calories_by_elf[calories_by_elf.len() - 3..].iter().sum()
}
