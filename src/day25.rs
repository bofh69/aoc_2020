use aoc_runner_derive::{aoc, aoc_generator};

type Data = u64;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| line.parse().expect("number"))
        .collect()
}

fn trans(subject_number: Data, loop_size: Data) -> Data {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}
#[aoc(day25, part1)]
pub fn solve_part1(data: &[Data]) -> Data {
    let mut ls = 0;
    let mut value = 1;
    while !data.contains(&value) {
        ls += 1;
        value *= 7;
        value %= 20201227;
    }

    if value == data[0] {
        trans(data[1], ls)
    } else {
        trans(data[0], ls)
    }
}
