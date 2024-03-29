use itertools::Itertools;
use std::convert::TryFrom;

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
    use once_cell::sync::Lazy;
    static LIGHTS: Lazy<[u8; 10]> = Lazy::new(|| {
        [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ]
        .map(|x| parse_lights(x))
    });

    LIGHTS.iter().position(|&x| x == display)
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

            use permutator::heap_permutation;
            let mut range = [0, 1, 2, 3, 4, 5, 6];
            let mut final_perm = None;
            heap_permutation(&mut range, |perm| {
                if final_perm.is_none()
                    && digits
                        .iter()
                        .all(|digit| lights_to_digit(permute_bits(perm, *digit)).is_some())
                {
                    final_perm = Some(<[usize; 7]>::try_from(perm).unwrap());
                }
            });

            num.split_whitespace()
                .map(|n| parse_lights(n))
                .rev()
                .enumerate()
                .map(|(i, digit)| {
                    10usize.pow(i as u32)
                        * lights_to_digit(permute_bits(final_perm.as_ref().unwrap(), digit))
                            .unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn day8_test() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    assert_eq!(5353, part2(input));
}
