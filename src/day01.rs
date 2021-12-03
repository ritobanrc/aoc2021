use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}
