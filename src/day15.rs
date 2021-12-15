use std::collections::{HashMap, HashSet};

fn dist(a: (i64, i64), b: (i64, i64)) -> usize {
    let d0 = a.0 - b.0;
    let d1 = a.1 - b.1;
    ((d0 * d0 + d1 * d1) as f64).sqrt().round() as usize
}

fn pathfind(map: &[Vec<u8>]) -> usize {
    let start = (0, 0);

    let mut open_set = Vec::new();
    open_set.push(start);
    let mut prev = HashMap::<(i64, i64), _>::new();

    let mut g_score = HashMap::<(i64, i64), _>::new();
    let mut f_score = HashMap::<(i64, i64), _>::new();

    g_score.insert(start, 0);

    let end = (map[0].len() as i64 - 1, map.len() as i64 - 1);

    while let Some((i, pos)) = open_set
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| f_score.get(v).unwrap_or(&usize::MAX))
    {
        let pos = pos.clone();
        open_set.remove(i);

        if pos == end {
            break;
        }

        let neighbors = [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];

        for neighbor in neighbors {
            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= map[0].len() as i64
                || neighbor.1 >= map[0].len() as i64
            {
                continue;
            }
            let alt = g_score[&pos] + map[pos.1 as usize][pos.0 as usize] as usize;
            if alt < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                prev.insert(neighbor, Some(pos));
                g_score.insert(neighbor, alt);
                f_score.insert(neighbor, alt + dist(neighbor, end));
                open_set.push(neighbor);
            }
        }
    }

    let mut current = end;
    let mut total_risk = 0;
    while current != start {
        total_risk += map[current.1 as usize][current.0 as usize] as usize;
        current = prev[&current].unwrap();
    }

    total_risk
}

pub fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    pathfind(&map)
}

pub fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let mut bigger_map = vec![vec![0; 5 * width]; 5 * height];
    for y in 0..bigger_map.len() {
        for x in 0..bigger_map[0].len() {
            let orig = map[y % height][x % width];
            let ny = (y / height) as u8;
            let nx = (x / width) as u8;
            bigger_map[y][x] = (orig + ny + nx - 1) % 9 + 1;
        }
    }

    pathfind(&bigger_map)
}

#[test]
fn day15_test() {
    let input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
        .trim();

    println!("{:?}", part2(input));
}
