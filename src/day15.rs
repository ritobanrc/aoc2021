use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, Ord)]
struct QueueEntry {
    pos: (usize, usize),
    f_score: usize,
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.f_score.eq(&other.f_score)
    }
}
impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f_score.partial_cmp(&other.f_score)
    }
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let d0 = b.0 - a.0;
    let d1 = b.1 - a.1;
    ((d0 * d0 + d1 * d1) as f64).sqrt().round() as usize
}

fn pathfind(map: &[Vec<u8>]) -> usize {
    let start = (0, 0);

    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse(QueueEntry {
        pos: start,
        f_score: 0,
    }));

    let width = map[0].len();
    let height = map.len();

    let mut prev = vec![vec![None; width]; height];

    let mut g_score = vec![vec![usize::MAX << 1; width]; height];
    let mut f_score = vec![vec![usize::MAX << 1; width]; height];

    g_score[start.1][start.0] = 0;

    let end = (map[0].len() - 1, map.len() - 1);

    while let Some(Reverse(entry)) = open_set.pop() {
        let pos = entry.pos;

        if pos == end {
            break;
        }

        let neighbors = [
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0, pos.1 + 1),
        ];

        for neighbor in neighbors {
            if neighbor.0 >= map[0].len() || neighbor.1 >= map[0].len() {
                continue;
            }
            let alt = g_score[pos.1][pos.0] + map[pos.1][pos.0] as usize;
            if alt < g_score[neighbor.1][neighbor.0] {
                prev[neighbor.1][neighbor.0] = Some(pos);
                g_score[neighbor.1][neighbor.0] = alt;

                let new_f_score = alt + dist(neighbor, end);
                f_score[neighbor.1][neighbor.0] = new_f_score;

                open_set.push(Reverse(QueueEntry {
                    pos: neighbor,
                    f_score: new_f_score,
                }));
            }
        }
    }

    let mut current = end;
    let mut total_risk = 0;
    while current != start {
        total_risk += map[current.1][current.0] as usize;
        current = prev[current.1][current.0].unwrap();
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
