use crate::Part;
use nalgebra::Vector2;
use std::collections::HashSet;

type IV = Vector2<i64>;

pub fn solutions(input: &str, part: Part) -> usize {
    let mut map = input
        .lines()
        .flat_map(|line| line.bytes().map(|b| b - b'0' as u8))
        .collect::<Vec<_>>();
    let width = input.lines().next().unwrap().len() as i64;

    let mut total_flashes = 0;
    let range = match part {
        Part::Part1 => 0..100,
        Part::Part2 => 0..usize::MAX,
    };

    for step in range {
        let mut to_flash = Vec::new();
        for (i, octupus) in map.iter_mut().enumerate() {
            *octupus += 1;
            if *octupus > 9 {
                to_flash.push(i);
            }
        }

        let mut flashed = HashSet::new();

        while let Some(octopus) = to_flash.pop() {
            total_flashes += 1;

            let pos = IV::new(octopus as i64 % width, octopus as i64 / width);
            flashed.insert(pos);

            for dx in -1..=1 {
                for dy in -1..=1 {
                    let neighbor = pos + IV::new(dx, dy);
                    if neighbor.x >= 0
                        && neighbor.x < width
                        && neighbor.y >= 0
                        && neighbor.y < width
                    {
                        let idx = (neighbor.x + width * neighbor.y) as usize;
                        if idx == octopus {
                            continue;
                        }
                        map[idx] += 1;
                        if map[idx] > 9 && !flashed.contains(&neighbor) && !to_flash.contains(&idx)
                        {
                            to_flash.push(idx);
                        }
                    }
                }
            }
        }

        for octopus in flashed.iter() {
            map[(octopus.x + width * octopus.y) as usize] = 0;
        }

        if part == Part::Part2 && flashed.len() == map.len() {
            return step + 1;
        }
    }

    match part {
        Part::Part1 => total_flashes,
        Part::Part2 => unreachable!(),
    }
}

#[test]
fn day11_test() {
    let input = "
11111
19991
19191
19991
11111"
        .trim();

    part1(input);
}
