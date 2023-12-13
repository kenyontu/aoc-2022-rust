pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| {
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            (
                (a1.parse::<u32>().unwrap(), a2.parse::<u32>().unwrap()),
                (b1.parse::<u32>().unwrap(), b2.parse::<u32>().unwrap()),
            )
        })
        .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (a.0 <= b.0 && a.1 >= b.1))
        .count()
}
