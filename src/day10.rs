use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Data = usize;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(data: &[Data]) -> Data {
    // Sort the data, count the differences.
    let mut data: Vec<_> = data.iter().copied().collect();
    data.sort_unstable();
    data.push(data[data.len() - 1] + 3);

    let mut diffs: HashMap<Data, Data> = HashMap::new();

    // From 0 to first adapter's joltage, there's 1 such difference
    // at the start.
    diffs.insert(data[0], 1);

    // Could be optimized to count in two variables, but meh
    for diff in data.windows(2).map(|win| win[1] - win[0]) {
        if let Some(val) = diffs.get_mut(&diff) {
            *val += 1;
        } else {
            diffs.insert(diff, 1);
        }
    }

    *diffs.get(&1).unwrap() * *diffs.get(&3).unwrap()
}

// Recursively find the number of allowed permutations.
// The cache[data.len()-1) = 1, so the recursion will always
// terminate.
fn count_permutations(pos: usize, data: &[Data], cache: &mut [Data]) -> Data {
    if cache[pos] != 0 {
        cache[pos]
    } else {
        let mut acc = 0;
        for i in (pos + 1)..=std::cmp::min(pos + 3, data.len() - 1) {
            let curr = data[pos];
            if data[i] - curr <= 3 {
                acc += count_permutations(i, data, cache);
            } else {
                break;
            }
        }
        cache[pos] = acc;
        acc
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(data: &[Data]) -> Data {
    let mut data: Vec<_> = data.iter().copied().collect();
    data.push(0); // The wall's jolt.
    data.sort_unstable();
    data.push(data[data.len() - 1] + 3); // The needed jolt.
    let mut cache = vec![0_usize; data.len()];
    // Last adapter can only be arranged in one way.
    cache[data.len() - 1] = 1;

    count_permutations(0, &data, &mut cache)
}
