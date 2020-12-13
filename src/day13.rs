use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashMap;

type Data = (usize, Vec<usize>);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Data {
    let mut lines = input.lines();
    let data : usize = lines.next().unwrap().parse().unwrap();
    let mut lines : Vec<_> = lines.next().unwrap().split(',').filter(|&s| s != "x").map(|s| s.parse().unwrap()).collect();
    lines.sort_unstable();
    (data, lines)
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &Data) -> usize {
    let (time, lines) = data;
    let mut line_and_time : Vec<(usize, usize)> = lines.iter().map(|&t| (t, (t - time % t) + time)).collect();
    line_and_time.sort_by_key(|(_line, time)| *time);

    let (line, departure) = line_and_time[0];

    line * (departure - time)
}

#[aoc(day13, part2)]
pub fn solve_part2(_data: &Data) -> usize {
    0
}
