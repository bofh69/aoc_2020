use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashMap;
// use ::regex::*;

type Data = Vec<Vec<bool>>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Data {
    input
        .lines()
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_trees(data: &Data, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut trees = 0;
    let width = data[0].len();
    for (y, row) in data.iter().enumerate() {
        if y % dy == 0 {
            if row[x] {
                trees += 1;
            }
            x = (x + dx) % width;
        }
    }
    trees
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &Data) -> usize {
    count_trees(data, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &Data) -> usize {
    count_trees(data, 1, 1)
        * count_trees(data, 3, 1)
        * count_trees(data, 5, 1)
        * count_trees(data, 7, 1)
        * count_trees(data, 1, 2)
}