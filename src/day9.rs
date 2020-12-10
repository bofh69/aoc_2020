use aoc_runner_derive::{aoc, aoc_generator};

type Data = usize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[Data]) -> Data {
    const PREAMBLE: usize = 25;
    data.windows(PREAMBLE + 1)
        .filter(|win| {
            let result = win[PREAMBLE];
            for i in 0..PREAMBLE {
                let k = win[i];
                if k < result {
                    for j in &win[i + 1..] {
                        let sum = k + j;
                        if sum == result {
                            return false;
                        }
                    }
                }
            }
            // This can not be the sum of two numbers in the window,
            // keep it.
            true
        })
        .map(|win| win[PREAMBLE])
        .next()
        .unwrap()
    // result: 133015568
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[Data]) -> Data {
    // Result from part 1:
    const INVALID: usize = 133015568;

    for i in 0..data.len() {
        let mut acc = 0;
        for j in i..data.len() {
            acc += data[j];
            if acc != data[j] && acc == INVALID {
                // Found it, return the sum of the minimum and maximum
                // in the span.
                let data = &data[i..=j];
                return data.iter().min().unwrap() + data.iter().max().unwrap();
            }
            if acc > INVALID {
                // Too large, this span isn't the one we're looking for.
                break;
            }
        }
    }
    panic!("No solution found");
    // result: 16107959
}
