pub fn solve(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|lines_by_elf| {
            lines_by_elf
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
