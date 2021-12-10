// Based off Repl.it Advent of Code Template
use std::env;
use std::fs;
use std::time::{Duration, Instant};

use aoc2021::get_day;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{}ms ", whole_ms) + &fmt_time(rem_ms);
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0)
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}

fn run_day(day: u32) -> std::io::Result<Duration> {
    // Read input file
    let cwd = env::current_dir()?;
    let filename = cwd.join("input").join(format!("day{:02}.txt", day));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(&filename)?;

    let mut total = Duration::new(0, 0);

    println!("---------------------");

    // Get corresponding function
    let to_run = get_day(day);
    // Time it
    if let Some((f1, f2)) = to_run {
        println!("Day {} - Part 1: ", day);
        let part1_start = Instant::now();

        let ans = f1(&input);

        let part1_dur = part1_start.elapsed();
        println!("    {:?}", ans);
        println!("    Time: {}", fmt_dur(part1_dur));
        total += part1_dur;

        println!("---------------------");
        println!("Day {} - Part 2: ", day);

        let part2_start = Instant::now();
        let ans = f2(&input);
        let part2_dur = part2_start.elapsed();

        println!("    {:?}", ans);
        println!("    Time: {}", fmt_dur(part2_dur));
        total += part2_dur;
        println!("---------------------");
    }

    Ok(total)
}

fn main() -> std::io::Result<()> {
    // Get day string
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let day = args[1].trim();
        let day_num: u32 = day.parse().expect("Failed to parse day number");

        run_day(day_num)?;
    } else {
        let mut total = Duration::new(0, 0);
        let mut n = 1;
        while get_day(n).is_some() {
            total += run_day(n)?;
            n += 1;
        }

        println!("Total Duration: {:?}", total);
    }
    Ok(())
}
