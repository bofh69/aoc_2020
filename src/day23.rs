use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashSet;
// use std::collections::VecDeque;

type Data = u8;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn find_dest(cups: &[u8; 9], pos: usize, val: u8) -> Option<usize> {
    for i in 0..6 {
        let pos = (pos + 4 + i) % 9;
        if cups[pos] % 9 == val {
            return Some(pos);
        }
    }
    None
}

fn play_round(cups: &mut [[u8; 9]; 2], idx: usize, pos: &mut usize) {
    let oidx = (idx + 1) & 1;
    let mut curr_val = (cups[idx][*pos] + 8) % 9;
    // Find destination
    let dest;
    loop {
        if let Some(d) = find_dest(&cups[idx], *pos, curr_val) {
            dest = d;
            break;
        }
        curr_val = (curr_val + 8) % 9;
    }
    let a = cups[idx][(*pos + 1) % 9];
    let b = cups[idx][(*pos + 2) % 9];
    let c = cups[idx][(*pos + 3) % 9];
    // Move
    let n = (dest + 18 - *pos - 3) % 9;
    let n2 = 5 - n;

    // 6: 92584(1)367
    //
    // From:
    // 0 .. pos <a> <b> <c> ... dest ... 8
    // pos <a> <b> <c> ... dest ... 8 ... 0
    //                 n        (9 - n)
    // To:
    // 0 .. pos ... dest <a> <b> <c> ... 8
    // pos ... dest <a> <b> <c> ... 8 ... 0
    //     n                    (9 - n)

    cups[oidx][*pos] = cups[idx][*pos];
    for i in 0..n {
        cups[oidx][(*pos + i + 1) % 9] = cups[idx][(*pos + i + 4) % 9];
    }
    cups[oidx][(*pos + n + 1) % 9] = a;
    cups[oidx][(*pos + n + 2) % 9] = b;
    cups[oidx][(*pos + n + 3) % 9] = c;
    for i in 0..n2 {
        cups[oidx][(*pos + n + i + 4) % 9] = cups[idx][(dest + i + 1) % 9];
    }

    // New pos
    *pos = (*pos + 1) % 9;
}

fn print_cups(cups: &[u8], pos: usize) {
    for (i, v) in cups.iter().enumerate() {
        if pos == i {
            print!("({})", v);
        } else {
            print!("{}", v);
        }
    }
    println!();
}

#[aoc(day23, part1)]
pub fn solve_part1(data: &[Data]) -> String {
    let mut cups = [[0_u8; 9]; 2];
    for i in 0..9 {
        cups[0][i] = data[i];
    }
    let mut idx = 0;
    let mut pos = 0;
    for i in 0..100 {
        print!("{:2}: ", i + 1);
        play_round(&mut cups, idx, &mut pos);
        idx = (idx + 1) & 1;
        print_cups(&cups[idx], pos);
    }
    // Fix result, find 1, then push the rest of the chars.
    let mut result = String::new();
    let pos = cups[idx]
        .iter()
        .enumerate()
        .find(|(_i, &v)| v == 1)
        .unwrap()
        .0;
    for i in 0..8 {
        result.push((0x30 + cups[idx][(i + pos + 1) % 9]) as char);
    }
    result
}

#[aoc(day23, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    const N: usize = 1_000_000;
    let mut prev = Vec::with_capacity(N);
    let mut next = Vec::with_capacity(N);
    for i in 0..N {
        prev.push((i + N - 1) % N);
        next.push((i + 1) % N);
    }

    // Start: 7 1 6 8 9 2 5 4 3 10 11 ... 1M
    // 7 (9) 2 5 1 6 8 4 3 10 11
    // 7 9 (6) 8 2 5 1 4 3 10 11
    // 7 9 6 (1) 4 8 2 5 3 10 11
    // 4 8 2 7 9 6 1 (5) 3 10 11
    // 4 3 10 11 8 2 7 9 6 1 5 (12) 13 14 15
    // 4 3 10 11 13 14 15 8 2 7 9 6 1 5 (16) 17 18 19
    //
    let mut current = (N + data[0] as usize - 1) % N;

    next[N - 1] = current;
    for i in 0..data.len() - 1 {
        next[data[i] as usize - 1] = data[i + 1] as usize - 1;
    }
    next[data[data.len() - 1] as usize - 1] = data.len();

    for i in 1..data.len() {
        prev[data[i] as usize - 1] = data[i - 1] as usize - 1;
    }
    prev[data[0] as usize - 1] = N - 1;
    prev[data.len()] = data[data.len() - 1] as usize - 1;

    for _move in 0..10_000_000 {
        let pickup1 = next[current];
        let pickup2 = next[pickup1];
        let pickup3 = next[pickup2];
        let following = next[pickup3];
        next[current] = following;
        prev[following] = current;
        let mut dest = (N + current - 1) % N;
        while [pickup1, pickup2, pickup3].contains(&dest) {
            dest = (N + dest - 1) % N;
        }
        let old_next = next[dest];
        next[dest] = pickup1;
        prev[pickup1] = dest;
        next[pickup3] = old_next;
        prev[old_next] = pickup3;
        current = next[current];

        /*
        println!("After {_move} moves");

        let mut pos = N-1;
        for _i in 0..12 {
            if next[prev[pos]] != pos {
                println!("prev+next is invalid");
            }
            if prev[next[pos]] != pos {
                println!("next+prev is invalid");
            }
            if pos == current {
                print!("({:}) ", pos + 1);
            } else {
                print!("{:2} ", pos + 1);
            }
            pos = next[pos];
        }
        */

        /*
        for i in 0..12 {
            let i = (N + i - 1) % N;
            println!("{}.next = {}, prev={}", i, next[i], prev[i]);
        }
        */
    }
    println!("{} {}", next[0] + 1, next[next[0]] + 1);

    (next[0] + 1) * (next[next[0]] + 1)
}
