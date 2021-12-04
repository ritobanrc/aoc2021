mod day01;
mod day02;
mod day03;
mod day04;

pub fn noop(_inp: String) -> Box<dyn std::fmt::Debug> {
    Box::new(())
}

pub type DayFn = fn(String) -> Box<dyn std::fmt::Debug>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

macro_rules! aoc {
    (with_enum:$day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::solutions(&input, Part::Part1)), |input| Box::new([<day $day>]::solutions(&input, Part::Part2)))
        }
    };
    (with_enum:$day: expr, $ans1: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| Box::new([<day $day>]::solutions(&input, Part::Part2))
        )
    };
    (with_enum:$day: expr, $ans1: expr, $ans2: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans = [<day $day>]::solutions(&input, Part::Part2); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        })
    };


    ($day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::part1(&input)), |input| Box::new([<day $day>]::part2(&input)))
        }
    };
    ($day: expr, $ans1: expr) => {
        paste::item! {
            (|input| {
                let ans =[<day $day>]::part1(&input);
                assert_eq!(ans, $ans1);
                Box::new(ans)
            }, |input| Box::new([<day $day>]::part2(&input)))
        }
    };
    ($day: expr, $ans1: expr, $ans2: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::part1(&input); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans =[<day $day>]::part2(&input); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        })
    };
}

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => aoc!(01, 1722, 1748),
        2 => aoc!(02, 1924923, 1982495697),
        3 => aoc!(03, 4139586, 1800151),
        4 => aoc!(04, 58374, 11377),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
