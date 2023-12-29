use ::regex::*;
use aoc_runner_derive::{aoc, aoc_generator};

type PosType = u8;

#[derive(Debug)]
pub struct Data {
    policy: (char, PosType, PosType),
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Data>, Box<dyn std::error::Error>> {
    let re = Regex::new("^([0-9]+)-([0-9]+) (.): (.*)$")?;

    Ok(input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Data {
                policy: (
                    caps.get(3).unwrap().as_str().chars().next().unwrap(),
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                ),
                password: caps.get(4).unwrap().as_str().to_string(),
            }
        })
        .collect())
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    // Count the number of passwords where
    // the number of the given character
    // should be between the first and second number, inclusive.
    data.iter()
        .filter(|row| {
            let count = row.password.chars().filter(|c| *c == row.policy.0).count();
            count >= row.policy.1 as usize && count <= row.policy.2 as usize
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    // Count the number of passwords where
    // only one of the two given positions
    // should contain the given character.
    // No more, no less.
    data.iter()
        .filter(|row| {
            row.password
                .chars()
                .enumerate()
                .filter(|(_n, c)| *c == row.policy.0)
                .filter(|(n, _c)| {
                    let n = (n + 1) as PosType;
                    n == row.policy.1 || n == row.policy.2
                })
                .count()
                == 1
        })
        .count()
}
