pub fn part1(input: &str) -> u32 {
    let nums: Vec<_> = input
        .lines()
        .map(|x| x.bytes().map(|x| x - b'0').collect::<Vec<_>>())
        .collect();

    let num_size = input.lines().next().unwrap().len();
    let mut gamma = 0;
    let mut epsilon = 0;

    for col in 0..num_size {
        let ones = count_ones(&nums, col);

        if ones >= nums.len() / 2 {
            gamma += 1 << (num_size - col - 1);
        } else {
            epsilon += 1 << (num_size - col - 1);
        }
    }

    gamma * epsilon
}

fn count_ones(nums: &[Vec<u8>], col: usize) -> usize {
    nums.iter().filter(|num| num[col] == 1).count()
}

fn count_zeros(nums: &[Vec<u8>], col: usize) -> usize {
    nums.iter().filter(|num| num[col] == 0).count()
}

fn filter(mut nums: Vec<Vec<u8>>, o2: bool) -> u32 {
    let num_size = nums[0].len();

    let to_retain_if_one = if o2 { 1 } else { 0 };
    let to_retain_if_zero = if o2 { 0 } else { 1 };

    for col in 0..num_size {
        if count_ones(&nums, col) >= count_zeros(&nums, col) {
            nums.retain(|num| num[col] == to_retain_if_one);
        } else {
            nums.retain(|num| num[col] == to_retain_if_zero);
        }

        if nums.len() == 1 {
            break;
        }
    }

    nums[0]
        .iter()
        .enumerate()
        .map(|(i, &x)| (x as u32) << (num_size - i - 1))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let nums: Vec<_> = input
        .lines()
        .map(|x| x.bytes().map(|x| x - b'0').collect::<Vec<_>>())
        .collect();
    let o2 = filter(nums.clone(), true);
    let co2 = filter(nums.clone(), false);

    o2 * co2
}

#[test]
fn test_day_3() {
    let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .trim();

    assert_eq!(198, part1(input));
    assert_eq!(230, part2(input));
}
