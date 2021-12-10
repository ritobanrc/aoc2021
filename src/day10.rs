#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BracketStyle {
    Parenthesis,
    Square,
    Brace,
    Angle,
}

impl BracketStyle {
    fn syntax_score(&self) -> usize {
        match self {
            &BracketStyle::Parenthesis => 3,
            &BracketStyle::Square => 57,
            &BracketStyle::Brace => 1197,
            &BracketStyle::Angle => 25137,
        }
    }

    fn autocorrect_score(&self) -> usize {
        match self {
            &BracketStyle::Parenthesis => 1,
            &BracketStyle::Square => 2,
            &BracketStyle::Brace => 3,
            &BracketStyle::Angle => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BracketType {
    Open,
    Close,
}

fn parse_bracket(c: char) -> (BracketStyle, BracketType) {
    match c {
        '{' => (BracketStyle::Brace, BracketType::Open),
        '(' => (BracketStyle::Parenthesis, BracketType::Open),
        '[' => (BracketStyle::Square, BracketType::Open),
        '<' => (BracketStyle::Angle, BracketType::Open),
        '}' => (BracketStyle::Brace, BracketType::Close),
        ')' => (BracketStyle::Parenthesis, BracketType::Close),
        ']' => (BracketStyle::Square, BracketType::Close),
        '>' => (BracketStyle::Angle, BracketType::Close),
        c => panic!("unrecognized char {:?}", c),
    }
}

pub fn part1(input: &str) -> usize {
    let mut total_illegal_score = 0;
    for line in input.lines() {
        let mut open = Vec::new();

        for c in line.chars() {
            let (style, t) = parse_bracket(c);
            if t == BracketType::Open {
                open.push(style);
            } else if t == BracketType::Close {
                if style == open[open.len() - 1] {
                    open.pop();
                } else {
                    // ILLEGAL CHARACTER!!!
                    total_illegal_score += style.syntax_score();
                    break;
                }
            }
        }
    }
    total_illegal_score
}

pub fn part2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            let mut open = Vec::new();

            for c in line.chars() {
                let (style, t) = parse_bracket(c);
                if t == BracketType::Open {
                    open.push(style);
                } else if t == BracketType::Close {
                    if style == open[open.len() - 1] {
                        open.pop();
                    } else {
                        // ILLEGAL CHARACTER!!!
                        return None;
                    }
                }
            }

            let mut score = 0;
            for bracket in open.iter().rev() {
                score = score * 5 + bracket.autocorrect_score();
            }
            Some(score)
        })
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn day10_test() {
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .trim();

    println!("{:?}", part2(input));
}
