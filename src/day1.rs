use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[i32]) -> i32 {
    for i in 0..data.len() {
        let a = data[i];
        if a < 2020 / 2 {
            for b in data.iter().skip(i) {
                if a + b == 2020 {
                    // dbg!((a, b, a * b));
                    return a * b;
                }
            }
        }
    }
    panic!("Didn't find a solution");
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[i32]) -> i32 {
    for i in 0..data.len() {
        let a = data[i];
        if a < 2020 / 2 {
            for j in i..data.len() {
                let b = data[j];
                if a + b >= 2020 {
                    continue;
                } else {
                    for c in data.iter().skip(j) {
                        if a + b + c == 2020 {
                            // dbg!((a, b, c, a * b * c));
                            return a * b * c;
                        }
                    }
                }
            }
        }
    }
    panic!("No solution found");
}
