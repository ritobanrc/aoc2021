use crate::Part;

fn simulate_fish(lanternfish: Vec<u8>, days: usize) -> usize {
    let mut counts = [0; 9];
    for fish in lanternfish {
        counts[fish as usize] += 1;
    }

    for _day in 0..days {
        let new_fish = counts[0];
        for left in 0..counts.len() - 1 {
            counts[left] = counts[left + 1];
        }
        counts[8] = new_fish;
        counts[6] += new_fish;
    }

    counts.iter().sum()
}

pub fn solutions(input: &str, part: Part) -> usize {
    let lanternfish = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let days = match part {
        Part::Part1 => 80,
        Part::Part2 => 256,
    };
    simulate_fish(lanternfish, days)
}

#[test]
fn day6_test() {
    let input = vec![3, 4, 3, 1, 2];
    assert_eq!(simulate_fish(input.clone(), 18), 26);
    assert_eq!(simulate_fish(input, 80), 5934);
}
