use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashMap;
// use ::regex::*;

type Data = usize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[Data]) -> Data {
    const PREAMBLE: usize = 25;
    data.iter()
        .enumerate()
        .skip(PREAMBLE)
        .filter(|(i, &num)| {
            for j in *i - PREAMBLE..*i {
                for k in &data[j + 1..*i] {
                    if data[j] + k == num {
                        return false;
                    }
                }
            }
            true
        })
        .map(|(_, &num)| num)
        .next()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[Data]) -> Data {
    // Result from part 1:
    const INVALID: usize = 133015568;

    for i in 0..data.len() {
        let mut acc = 0;
        for j in i..data.len() {
            acc += data[j];
            // dbg!((i, data[j], acc));
            if acc != data[j] && acc == INVALID {
                // Found it.
                let data = &data[i..=j];
                return data.iter().min().unwrap() + data.iter().max().unwrap();
            }
            if acc > INVALID {
                break;
            }
        }
    }
    panic!("No solution found");
}
