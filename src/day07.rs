fn solution<F: Fn(i64) -> i64>(input: &str, calc_fuel: F) -> i64 {
    let positions = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..max)
        .map(|x| positions.iter().map(|i| calc_fuel((i - x).abs())).sum())
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> i64 {
    solution(input, |x| x)
}

pub fn part2(input: &str) -> i64 {
    solution(input, |x| x * (x + 1) / 2)
}
