use aoc_runner_derive::{aoc, aoc_generator};

type Data = (usize, Vec<(usize, usize)>);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Data {
    let mut lines = input.lines();
    let data: usize = lines.next().unwrap().parse().unwrap();
    let mut lines: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(n, s)| (n, s.parse().unwrap()))
        .collect();
    lines.sort_unstable();
    (data, lines)
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &Data) -> usize {
    let (time, lines) = data;
    let mut line_and_time: Vec<(usize, usize)> = lines
        .iter()
        .map(|(_, t)| (*t, (t - time % t) + time))
        .collect();
    line_and_time.sort_by_key(|(_line, time)| *time);

    let (line, departure) = line_and_time[0];

    line * (departure - time)
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &Data) -> usize {
    let lines = &data.1;
    let mut t = 0;
    let mut cycle = lines[0].1;
    for (offset, line) in lines.iter().skip(1) {
        while (t + offset) % line != 0 {
            t += cycle;
        }
        cycle *= line;
    }
    t
}
