use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use ::regex::*;

type Data = u16;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .rev()
                .enumerate()
                .map(|(pos, c)| match c {
                    'F' => 0,
                    'B' => 1 << pos,
                    'L' => 0,
                    'R' => 1 << pos,
                    _ => panic!("Unknown char"),
                })
                .sum()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &[Data]) -> Data {
    data.iter().fold(0, |acc, &n| std::cmp::max(acc, n))
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &[Data]) -> Data {
    let min = data.iter().fold(Data::MAX, |acc, &n| std::cmp::min(acc, n));

    let data = {
        let mut tmp = HashSet::with_capacity(data.len());
        for n in data {
            tmp.insert(n);
        }
        tmp
    };

    (min..(1 << 10))
        .filter(|i| !data.contains(&i))
        .next()
        .unwrap()
}
