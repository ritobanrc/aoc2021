use itertools::Itertools;
use std::collections::HashMap;

pub fn count_frequencies<I>(arr: I) -> HashMap<I::Item, usize>
where
    I: Iterator,
    I::Item: std::hash::Hash + std::cmp::Eq,
{
    let mut freqs = HashMap::new();
    for i in arr {
        freqs.entry(i).and_modify(|n| *n += 1).or_insert(1);
    }
    freqs
}

fn get_rules(input: &str) -> HashMap<(u8, u8), u8> {
    input
        .lines()
        .skip(2)
        .filter_map(|line| {
            let (a, b) = line.split(" -> ").collect_tuple()?;
            let source: (u8, u8) = a.bytes().collect_tuple()?;
            let new = b.bytes().next()?;
            Some((source, new))
        })
        .collect()
}

pub fn _solutions_naive(input: &str, part: crate::Part) -> usize {
    let mut template: Vec<u8> = input.lines().next().unwrap().bytes().collect();

    let rules = get_rules(input);

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

    let freqs = count_frequencies(template.into_iter());
    freqs.values().max().unwrap() - freqs.values().min().unwrap()
}

pub fn solutions(input: &str, part: crate::Part) -> usize {
    let template: Vec<u8> = input.lines().next().unwrap().bytes().collect();

    let first = template[0];
    let last = template[template.len() - 1];

    let rules = get_rules(input);

    let mut pair_freqs: HashMap<(u8, u8), usize> =
        count_frequencies(template.into_iter().tuple_windows());

    let steps = match part {
        crate::Part::Part1 => 10,
        crate::Part::Part2 => 40,
    };

    let mut next = HashMap::<(u8, u8), usize>::new();

    for _step in 0..steps {
        for (pair, count) in pair_freqs.iter() {
            if let Some(&new) = rules.get(pair) {
                let new_pair_1 = (new, pair.1);
                let new_pair_2 = (pair.0, new);

                next.entry(new_pair_1)
                    .and_modify(|n| *n += *count)
                    .or_insert(*count);

                next.entry(new_pair_2)
                    .and_modify(|n| *n += *count)
                    .or_insert(*count);
            } else {
                next.entry(*pair).and_modify(|n| *n += 1).or_insert(1);
            }
        }

        std::mem::swap(&mut next, &mut pair_freqs);
        next.clear();
    }

    let mut freqs = HashMap::<u8, usize>::new();
    let mut add_count = |letter, count| {
        freqs
            .entry(letter)
            .and_modify(|n| *n += count)
            .or_insert(count);
    };

    for (pair, &count) in pair_freqs.iter() {
        add_count(pair.0, count);
        add_count(pair.1, count);
    }

    add_count(first, 1);
    add_count(last, 1);

    let max = freqs.values().max().unwrap() / 2;
    let min = freqs.values().min().unwrap() / 2;

    max - min
}

#[test]
fn day14_test() {
    const INPUT: &str = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    println!("{:?}", solutions(INPUT.trim(), crate::Part::Part1));
}
