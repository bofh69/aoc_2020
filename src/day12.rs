use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub enum Inst {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

type Data = Inst;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| match line.chars().next() {
            Some('N') => Inst::North(line[1..].parse().unwrap()),
            Some('S') => Inst::South(line[1..].parse().unwrap()),
            Some('E') => Inst::East(line[1..].parse().unwrap()),
            Some('W') => Inst::West(line[1..].parse().unwrap()),
            Some('F') => Inst::Forward(line[1..].parse().unwrap()),
            Some('L') => Inst::Left(line[1..].parse().unwrap()),
            Some('R') => Inst::Right(line[1..].parse().unwrap()),
            _ => panic!("Unknown instruction"),
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[Data]) -> i32 {
    let mut dir = 90;
    let mut pos = (0, 0);

    for inst in data {
        match inst {
            Inst::North(n) => pos = (pos.0, pos.1 + n),
            Inst::South(n) => pos = (pos.0, pos.1 - n),
            Inst::East(n) => pos = (pos.0 + n, pos.1),
            Inst::West(n) => pos = (pos.0 - n, pos.1),
            Inst::Left(n) => dir = (360 + dir - n) % 360,
            Inst::Right(n) => dir = (dir + n) % 360,
            Inst::Forward(n) => match dir {
                0 => pos = (pos.0, pos.1 + n),
                180 => pos = (pos.0, pos.1 - n),
                90 => pos = (pos.0 + n, pos.1),
                270 => pos = (pos.0 - n, pos.1),
                _ => panic!("Unknown dir {}", dir),
            },
        }
    }

    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &[Data]) -> i32 {
    let mut pos = (10, 1);
    let mut ship = (0, 0);

    for inst in data {
        match inst {
            Inst::North(n) => pos = (pos.0, pos.1 + n),
            Inst::South(n) => pos = (pos.0, pos.1 - n),
            Inst::East(n) => pos = (pos.0 + n, pos.1),
            Inst::West(n) => pos = (pos.0 - n, pos.1),
            Inst::Left(n) => match n {
                0 => (),
                180 => pos = (-pos.0, -pos.1),
                90 => pos = (-pos.1, pos.0),
                270 => pos = (pos.1, -pos.0),
                _ => panic!("Unknown dir {}", n),
            },
            Inst::Right(n) => match n {
                0 => (),
                180 => pos = (-pos.0, -pos.1),
                270 => pos = (-pos.1, pos.0),
                90 => pos = (pos.1, -pos.0),
                _ => panic!("Unknown dir {}", n),
            },
            Inst::Forward(n) => ship = (ship.0 + n * pos.0, ship.1 + n * pos.1),
        }
    }

    ship.0.abs() + ship.1.abs()
}
