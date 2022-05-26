use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
}

type Point = (i64, i64);

#[derive(Debug)]
struct Input {
    points: Vec<Point>,
    folds: Vec<(Axis, i64)>,
}

fn parse(input: &str) -> Input {
    let (points, folds) = input.split("\n\n").collect_tuple().unwrap();
    let points = points
        .lines()
        .filter_map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<i64>().ok())
                .collect_tuple()
                .into()
        })
        .collect();

    let folds: Vec<_> = folds
        .lines()
        .filter_map(|line| {
            let (prefix, num) = line.split('=').collect_tuple()?;
            let num = num.parse::<i64>().ok()?;
            let axis = match prefix.chars().last() {
                Some('x') => Axis::X,
                Some('y') => Axis::Y,
                c => {
                    eprintln!("Unrecognized fold axis: {:?}", c);
                    return None;
                }
            };

            Some((axis, num))
        })
        .collect();

    Input { points, folds }
}

fn apply_fold(points: &mut Vec<Point>, fold: (Axis, i64)) {
    for point in points.iter_mut() {
        match fold {
            (Axis::X, x) => {
                if point.0 - x > 0 {
                    point.0 = x - (point.0 - x);
                }
            }
            (Axis::Y, y) => {
                if point.1 - y > 0 {
                    point.1 = y - (point.1 - y);
                }
            }
        }
    }

    points.sort();
    points.dedup();
}

pub fn part1(input: &str) -> usize {
    let mut input = parse(input);

    apply_fold(&mut input.points, *input.folds.first().unwrap());

    input.points.len()
}

fn print_points(points: &[(i64, i64)]) {
    let max_x = *points.iter().map(|(x, _y)| x).max().unwrap() + 1;
    let max_y = *points.iter().map(|(_x, y)| y).max().unwrap() + 1;

    let points = HashSet::<_>::from_iter(points);

    for y in 0..max_y {
        for x in 0..max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

pub fn part2(input: &str) {
    let mut input = parse(input);

    for fold in input.folds {
        apply_fold(&mut input.points, fold);
    }

    print_points(&input.points);
}

#[test]
fn day13_test() {
    const INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    part2(INPUT.trim());
}
