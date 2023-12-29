use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub enum Data {
    Mask(u64, u64),
    Mem(u64, u64),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            if let Some(line) = line.strip_prefix("mask = ") {
                let zeros = u64::from_str_radix(&line.replace('X', "0"), 2).unwrap();
                let ones = u64::from_str_radix(&line.replace('X', "1"), 2).unwrap();
                Data::Mask(zeros, ones)
            } else if let Some(line) = line.strip_prefix("mem[") {
                let mut iter = line.split("] = ");
                let addr = iter.next().unwrap().parse().unwrap();
                let value = iter.next().unwrap().parse().unwrap();
                Data::Mem(addr, value)
            } else {
                panic!("Unknown line");
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(data: &[Data]) -> u64 {
    let mut mem = HashMap::new();

    let mut zeros = 0;
    let mut ones = 0;

    for inst in data {
        match inst {
            Data::Mask(z, o) => {
                zeros = *z;
                ones = *o;
            }
            Data::Mem(addr, val) => {
                let mut val = *val;
                val &= ones;
                val |= zeros;
                mem.insert(addr, val);
            }
        }
    }

    mem.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &[Data]) -> u64 {
    let mut mem = HashMap::new();

    let mut bits = 0;
    let mut mask_bit_pos = vec![];

    for inst in data {
        match inst {
            Data::Mask(z, o) => {
                bits = *z & *o;

                mask_bit_pos.clear();
                let mask = !*z & *o;
                if mask != 0 {
                    for i in 0..36 {
                        if mask & (1 << i) != 0 {
                            mask_bit_pos.push(i);
                        }
                    }
                }
            }
            Data::Mem(addr, val) => {
                let mut addr = *addr;
                addr |= bits;
                // How many bits are floating?
                let n_mask_bits = mask_bit_pos.len();
                // i counts through the number of possible floating values
                for i in 0..(1 << n_mask_bits) {
                    let mut addr2 = addr;
                    // j counts over each floating bit to set/clear.
                    for j in 0..n_mask_bits {
                        let bit = 1 << mask_bit_pos[j];
                        // If the bit is set in i, set the right floating point else clear it.
                        addr2 = if i & (1 << j) != 0 {
                            addr2 | bit
                        } else {
                            addr2 & !bit
                        };
                    }
                    mem.insert(addr2, *val);
                }
            }
        }
    }

    mem.values().sum()
}
