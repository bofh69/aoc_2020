use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Data = usize;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    const GOAL: usize = 2020;
    let mut previous = HashMap::new();
    for (i, number) in data.iter().enumerate() {
        previous.insert(*number, i);
    }
    let mut last: usize = data[data.len() - 1];
    let mut before: Option<usize> = None;
    for turn in data.len()..GOAL {
        if let Some(prev_turn) = before {
            last = turn - 1 - prev_turn;
        } else {
            last = 0;
        }
        before = previous.get(&last).copied();
        previous.insert(last, turn);
        dbg!(&(turn, last));
    }
    last
}

#[aoc(day15, part2)]
pub fn solve_part2(data: &[Data]) -> u32 {
    const GOAL: u32 = 30_000_000;
    let mut previous = vec![GOAL; GOAL as usize];
    for (i, number) in data.iter().enumerate() {
        previous[*number] = i as u32;
    }
    let mut last: u32 = data[data.len() - 1] as u32;
    let mut before: u32 = GOAL;
    for turn in (data.len() as u32)..GOAL {
        if before != GOAL {
            last = turn - 1 - before;
        } else {
            last = 0;
        }
        before = previous[last as usize];
        previous[last as usize] = turn;
    }
    last
}
