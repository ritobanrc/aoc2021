use nalgebra::Vector2;
use std::ops::RangeInclusive;

type Target = [RangeInclusive<i64>; 2];
type IV = Vector2<i64>;

fn simulate(mut vel: IV, target: Target) -> Option<i64> {
    let mut pos = IV::zeros();
    let mut highest = 0;
    for _ in 0..10000 {
        pos += vel;
        vel.x -= vel.x.signum();
        vel.y -= 1;
        if pos.y > highest {
            highest = pos.y;
        }

        if target[0].contains(&pos.x) && target[1].contains(&pos.y) {
            return Some(highest);
        }
    }
    None
}

pub fn solutions(_input: &str, part: crate::Part) -> i64 {
    let x_range = 236..=262i64;
    let y_range = -78..=-58i64;

    let mut highest = 0;
    let mut count = 0;

    for x_vel in 0..300 {
        for y_vel in -300..300 {
            let out = simulate(IV::new(x_vel, y_vel), [x_range.clone(), y_range.clone()]);
            if let Some(peak) = out {
                count += 1;
                if peak > highest {
                    highest = peak;
                }
            }
        }
    }

    match part {
        crate::Part::Part1 => highest,
        crate::Part::Part2 => count,
    }
}
