pub fn part1(input: String) -> i64 {
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let (ins, num) = line.split_once(' ').expect("parse error");
        let num = num.parse::<i64>().expect("parse error");

        match ins {
            "forward" => x += num,
            "up" => y -= num,
            "down" => y += num,
            e => panic!("unrecognized instruction {:?}", e),
        }
    }

    x * y
}

pub fn part2(input: String) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in input.lines() {
        let (ins, num) = line.split_once(' ').expect("parse error");
        let num = num.parse::<i64>().expect("parse error");

        match ins {
            "forward" => {
                x += num;
                y += aim * num;
            }
            "up" => aim -= num,
            "down" => aim += num,
            e => panic!("unrecognized instruction {:?}", e),
        }
    }

    x * y
}
