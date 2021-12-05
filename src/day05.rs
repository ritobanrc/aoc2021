use crate::Part::{self, Part2};
use nalgebra::Vector2;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

type Point = Vector2<i32>;

#[derive(Display, Debug, FromStr)]
#[display("{start.x},{start.y} -> {end.x},{end.y}")]
struct Line {
    #[from_str(default)]
    start: Point,
    #[from_str(default)]
    end: Point,
}

pub fn solutions(input: &str, part: Part) -> usize {
    let lines = input
        .lines()
        .filter_map(|line| line.parse::<Line>().ok())
        .collect::<Vec<_>>();

    let mut map = HashMap::<Point, usize>::new();

    for line in &lines {
        let dist = line.end - line.start;
        let direction = if dist.y == 0 {
            Point::new(dist.x.signum(), 0)
        } else if dist.x == 0 {
            Point::new(0, dist.y.signum())
        } else if part == Part2 && dist.x.abs() == dist.y.abs() {
            Point::new(dist.x.signum(), dist.y.signum())
        } else {
            continue;
        };

        let mut current = line.start;
        while current != line.end + direction {
            map.entry(current).and_modify(|x| *x += 1).or_insert(1);
            current += direction;
        }
    }

    map.values().filter(|&&x| x >= 2).count()
}

#[test]
fn day5_test() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .trim();

    assert_eq!(solutions(input, Part1), 5);
    assert_eq!(solutions(input, Part2), 12);
}
