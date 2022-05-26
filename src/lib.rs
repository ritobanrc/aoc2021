mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day15;
mod day17;

pub type DayFn = fn(&str) -> Box<dyn std::fmt::Debug>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

macro_rules! aoc {
    (with_enum:$day: expr) => {
        paste::item! {
            Some((|input| Box::new([<day $day>]::solutions(&input, Part::Part1)), |input| Box::new([<day $day>]::solutions(&input, Part::Part2))))
        }
    };
    (with_enum:$day: expr, $ans1: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { Box::new([<day $day>]::solutions(&input, Part::Part2)) }
        }
        ))
    };
    (with_enum:$day: expr, $ans1: expr, $ans2: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part2); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        }))
    };


    ($day: expr) => {
        paste::item! {
            Some((|input| Box::new([<day $day>]::part1(&input)), |input| Box::new([<day $day>]::part2(&input))))
        }
    };
    ($day: expr, $ans1: expr) => {
        paste::item! {
            Some((|input| {
                let ans =[<day $day>]::part1(&input);
                assert_eq!(ans, $ans1);
                Box::new(ans)
            }, |input| Box::new([<day $day>]::part2(&input))))
        }
    };
    ($day: expr, $ans1: expr, $ans2: expr) => {
        Some((|input| {
            paste::item! { let ans = [<day $day>]::part1(&input); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans =[<day $day>]::part2(&input); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        }))
    };
}

pub fn get_day(day: u32) -> Option<(DayFn, DayFn)> {
    return match day {
        1 => aoc!(01, 1722, 1748),
        2 => aoc!(02, 1924923, 1982495697),
        3 => aoc!(03, 4139586, 1800151),
        4 => aoc!(04, 58374, 11377),
        5 => aoc!(with_enum: 05, 6572, 21466),
        6 => aoc!(with_enum: 06, 365862, 1653250886439),
        7 => aoc!(07, 357353, 104822130),
        8 => aoc!(08, 284, 973499),
        9 => aoc!(09, 439, 900900),
        10 => aoc!(10, 374061, 2116639949),
        11 => aoc!(with_enum: 11, 1773, 494),
        12 => aoc!(12),
        13 => aoc!(13, 837),
        15 => aoc!(15, 685, 2995),
        17 => aoc!(with_enum: 17, 3003, 940),
        _ => {
            eprintln!("Unknown day: {}", day);
            return None;
        }
    };
}
