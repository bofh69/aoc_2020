use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Floor,
    Empty,
    Occupied,
}

type Data = Vec<State>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => State::Occupied,
                    'L' => State::Empty,
                    '.' => State::Floor,
                    _ => panic!("Unknown tile"),
                })
                .collect()
        })
        .collect()
}

fn contains(x: usize, y: usize, from: &[State], width: usize, height: usize, state: State) -> bool {
    if x < width && y < height {
        from[x + y * width] == state
    } else {
        false
    }
}

fn count_around(
    x: usize,
    y: usize,
    data: &[State],
    width: usize,
    height: usize,
    state: State,
) -> usize {
    let mut count = 0;

    if x > 0 && contains(x - 1, y, data, width, height, state) {
        count += 1
    }
    if x > 0 && y > 0 && contains(x - 1, y - 1, data, width, height, state) {
        count += 1
    }
    if y > 0 && contains(x, y - 1, data, width, height, state) {
        count += 1
    }
    if y > 0 && contains(x + 1, y - 1, data, width, height, state) {
        count += 1
    }
    if contains(x + 1, y, data, width, height, state) {
        count += 1
    }
    if contains(x + 1, y + 1, data, width, height, state) {
        count += 1
    }
    if contains(x, y + 1, data, width, height, state) {
        count += 1
    }
    if x > 0 && contains(x - 1, y + 1, data, width, height, state) {
        count += 1
    }

    count
}

fn run_once(from: &[State], to: &mut [State], width: usize, height: usize) -> bool {
    let mut changed = false;

    for y in 0..height {
        for x in 0..width {
            let idx = x + y * width;
            to[idx] = match from[idx] {
                State::Floor => State::Floor,
                State::Empty => {
                    if count_around(x, y, from, width, height, State::Occupied) == 0 {
                        changed = true;
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Occupied => {
                    if count_around(x, y, from, width, height, State::Occupied) >= 4 {
                        changed = true;
                        State::Empty
                    } else {
                        State::Occupied
                    }
                }
            }
        }
    }

    changed
}

/*
fn print_it(data: &[State], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            match data[x + y * width] {
                State::Empty => print!("L"),
                State::Occupied => print!("#"),
                State::Floor => print!("."),
            }
        }
        println!("");
    }
    println!("");
}
*/

#[aoc(day11, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut data: Vec<State> = data.iter().flatten().copied().collect();
    let mut data2: Vec<State> = vec![State::Floor; data.len()];

    let mut d1 = &mut data;
    let mut d2 = &mut data2;

    while run_once(d1, d2, width, height) {
        std::mem::swap(&mut d1, &mut d2);
    }
    d2.iter().filter(|&s| State::Occupied == *s).count()
}

fn contains2(
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    from: &[State],
    width: usize,
    height: usize,
) -> bool {
    let mut x = x as i32 + dx;
    let mut y = y as i32 + dy;
    while x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
        match from[x as usize + y as usize * width] {
            State::Occupied => return true,
            State::Empty => return false,
            State::Floor => (),
        }
        x += dx;
        y += dy;
    }
    false
}

fn count_around2(
    x: usize,
    y: usize,
    data: &[State],
    width: usize,
    height: usize,
    _state: State,
) -> usize {
    let mut count = 0;

    if contains2(x, y, -1, 0, data, width, height) {
        count += 1
    }
    if contains2(x, y, -1, -1, data, width, height) {
        count += 1
    }
    if contains2(x, y, 0, -1, data, width, height) {
        count += 1
    }
    if contains2(x, y, 1, -1, data, width, height) {
        count += 1
    }
    if contains2(x, y, 1, 0, data, width, height) {
        count += 1
    }
    if contains2(x, y, 1, 1, data, width, height) {
        count += 1
    }
    if contains2(x, y, 0, 1, data, width, height) {
        count += 1
    }
    if contains2(x, y, -1, 1, data, width, height) {
        count += 1
    }

    count
}

fn run_once2(from: &[State], to: &mut [State], width: usize, height: usize) -> bool {
    let mut changed = false;

    for y in 0..height {
        for x in 0..width {
            let idx = x + y * width;
            to[idx] = match from[idx] {
                State::Floor => State::Floor,
                State::Empty => {
                    if count_around2(x, y, from, width, height, State::Occupied) == 0 {
                        changed = true;
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Occupied => {
                    if count_around2(x, y, from, width, height, State::Occupied) >= 5 {
                        changed = true;
                        State::Empty
                    } else {
                        State::Occupied
                    }
                }
            }
        }
    }

    changed
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut data: Vec<State> = data.iter().flatten().copied().collect();
    let mut data2: Vec<State> = vec![State::Floor; data.len()];

    let mut d1 = &mut data;
    let mut d2 = &mut data2;

    while run_once2(d1, d2, width, height) {
        std::mem::swap(&mut d1, &mut d2);
    }
    d2.iter().filter(|&s| State::Occupied == *s).count()
}
