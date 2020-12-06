use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
// use ::regex::*;

type Data = Vec<HashSet<char>>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().filter(|&c| c != '\n').collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    data.iter()
        .map(|group| {
            let mut union = HashSet::new();
            group.iter().for_each(|person| {
                person.iter().for_each(|answer| {
                    union.insert(answer);
                })
            });
            union.len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    data.iter()
        .map(|group| {
            let acc = group[0].clone();
            let g = group.iter().fold(acc, |acc, person| {
                acc.intersection(person).copied().collect()
            });
            g.len()
        })
        .sum()
}
