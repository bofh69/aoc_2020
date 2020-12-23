use aoc_runner_derive::{aoc, aoc_generator};

type Data = Vec<u32>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().map(|c| 1 << (c as u32 - 'a' as u32)).sum())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[Data]) -> u32 {
    data.iter()
        .map(|group| {
            group
                .iter()
                .fold(0, |acc, person| acc | person)
                .count_ones()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[Data]) -> u32 {
    data.iter()
        .map(|group| {
            group
                .iter()
                .fold(!0, |acc, person| acc & person)
                .count_ones()
        })
        .sum()
}
