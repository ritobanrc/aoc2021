use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, num) = line.split(" | ").collect_tuple().unwrap();
            num.split_whitespace()
                .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

/// Represent the 7-segment display as a bitmask
fn parse_lights(num: &str) -> u8 {
    let mut r = 0;
    num.bytes().for_each(|c| r |= 1 << (c - b'a'));
    r
}

fn lights_to_digit(display: u8) -> Option<usize> {
    let lights = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .map(|x| parse_lights(x));

    lights.iter().position(|&x| x == display)
}

fn permute_bits(permutation: &[usize], num: u8) -> u8 {
    let mut r = 0;
    for (i, p) in permutation.iter().enumerate() {
        if num & (1 << p) != 0 {
            r |= 1 << i;
        }
    }
    r
}

pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (digits, num) = line.split(" | ").collect_tuple().unwrap();
            let digits = digits
                .split_whitespace()
                .map(|n| parse_lights(n))
                .collect::<Vec<_>>();
            let num = num
                .split_whitespace()
                .map(|n| parse_lights(n))
                .collect::<Vec<_>>();

            let perm = (0..7)
                .permutations(7)
                .filter(|perm| {
                    for digit in &digits {
                        let permuted = permute_bits(perm, *digit);
                        if lights_to_digit(permuted).is_none() {
                            return false;
                        }
                    }
                    true
                })
                .next()
                .unwrap();

            let mut r = 0;
            for (i, digit) in num.iter().rev().enumerate() {
                let permuted = permute_bits(&perm, *digit);
                r += 10usize.pow(i as u32) * lights_to_digit(permuted).unwrap();
            }
            r
            //num.split_whitespace()
            //.filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7))
            //.count()
        })
        .sum()
}

#[test]
fn day8_test() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    assert_eq!(5353, part2(input));
}
