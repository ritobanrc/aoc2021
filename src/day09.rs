use nalgebra::Vector2;
use std::collections::HashSet;

type IV = Vector2<i64>;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_lowest_points(heightmap: &[Vec<u8>]) -> Vec<IV> {
    let dimension = IV::new(heightmap[0].len() as i64, heightmap.len() as i64);
    let index = move |idx: IV| {
        heightmap
            .get(idx.y as usize)
            .and_then(|line| line.get(idx.x as usize))
            .copied()
    };

    let mut lowest_points = Vec::new();

    for y in 0..dimension.y {
        for x in 0..dimension.x {
            let pos = IV::new(x, y);
            let height = index(pos).unwrap();
            let neighbors = [
                pos + IV::ith_axis(0).into_inner(),
                pos - IV::ith_axis(0).into_inner(),
                pos + IV::ith_axis(1).into_inner(),
                pos - IV::ith_axis(1).into_inner(),
            ];
            if neighbors.iter().all(|&neighbor| {
                if let Some(x) = index(neighbor) {
                    return height < x;
                }
                true
            }) {
                lowest_points.push(pos);
                // this is a low point
            };
        }
    }

    lowest_points
}

pub fn part1(input: &str) -> u64 {
    let heightmap = parse_input(input);
    get_lowest_points(&heightmap)
        .iter()
        .map(|pos| heightmap[pos.y as usize][pos.x as usize] as u64 + 1)
        .sum()
}

fn flood_fill(heightmap: &[Vec<u8>], start: IV) -> usize {
    let mut stack = Vec::new();
    stack.push(start);

    let mut visited = HashSet::new();

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        let height = heightmap[pos.y as usize][pos.x as usize];
        if height == 9 {
            continue;
        }
        visited.insert(pos);

        let neighbors = [
            pos + IV::ith_axis(0).into_inner(),
            pos - IV::ith_axis(0).into_inner(),
            pos + IV::ith_axis(1).into_inner(),
            pos - IV::ith_axis(1).into_inner(),
        ];

        for neighbor in neighbors {
            if !visited.contains(&neighbor)
                && neighbor.x >= 0
                && neighbor.y >= 0
                && neighbor.x < heightmap[0].len() as i64
                && neighbor.y < heightmap.len() as i64
            {
                stack.push(neighbor);
            }
        }
    }

    visited.len()
}

pub fn part2(input: &str) -> usize {
    let heightmap = parse_input(input);
    let lowest_points = get_lowest_points(&heightmap);
    let mut sizes = lowest_points
        .iter()
        .map(|start| flood_fill(&heightmap, *start))
        .collect::<Vec<_>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0..3].iter().product()
}

#[test]
fn day9_test() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    assert_eq!(15, part1(input));
    assert_eq!(1134, part2(input));
}
