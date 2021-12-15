use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cave {
    Start,
    End,
    Smol([char; 2]),
    Big([char; 2]),
}

impl Cave {
    fn from_str(cave: &str) -> Self {
        match cave {
            "start" => Self::Start,
            "end" => Self::End,
            a if a.chars().all(|c| c.is_lowercase()) => {
                let mut chars = a.chars();
                Cave::Smol([
                    chars.next().unwrap().to_ascii_uppercase(),
                    chars.next().unwrap().to_ascii_uppercase(),
                ])
            }
            a if a.chars().all(|c| c.is_uppercase()) => {
                let mut chars = a.chars();
                Cave::Big([chars.next().unwrap(), chars.next().unwrap()])
            }
            _ => panic!("weird cave {:?}", cave),
        }
    }
}

pub fn part1(input: &str) {
    let edge_list = input
        .lines()
        .map(|s| {
            let (a, b) = s.split('-').collect_tuple().unwrap();
            (Cave::from_str(a), Cave::from_str(b))
        })
        .collect::<Vec<_>>();

    println!("{:?}", edge_list);
}

pub fn part2(input: &str) {}
