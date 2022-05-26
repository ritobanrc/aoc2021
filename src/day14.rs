use itertools::Itertools;
use std::collections::HashMap;

pub fn count_frequencies(arr: &[char]) -> HashMap<char, usize> {
    let mut freqs = HashMap::new();
    for i in arr {
        freqs.entry(*i).and_modify(|n| *n += 1).or_insert(1);
    }
    freqs
}

pub fn solutions(input: &str, part: crate::Part) -> usize {
    let mut template: Vec<char> = input.lines().next().unwrap().chars().collect();

    let rules: HashMap<_, _> = input
        .lines()
        .skip(2)
        .filter_map(|line| {
            let (a, b) = line.split(" -> ").collect_tuple()?;
            let source: (char, char) = a.chars().collect_tuple()?;
            let new = b.chars().next()?;
            Some((source, new))
        })
        .collect();

    let steps = match part {
        crate::Part::Part1 => 10,
        crate::Part::Part2 => 40,
    };

    for _ in 0..steps {
        let mut i = 0;
        while i < template.len() - 1 {
            let pair = (template[i], template[i + 1]);
            if let Some(&new) = rules.get(&pair) {
                template.insert(i + 1, new);
                i += 1;
            }
            i += 1;
        }
    }

    let freqs = count_frequencies(&template);
    freqs.values().max().unwrap() - freqs.values().min().unwrap()
}
