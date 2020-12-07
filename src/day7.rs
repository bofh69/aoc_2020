use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
// use ::regex::*;

type Data = HashMap<String, Vec<(usize, String)>>;

/*
dim orange bags contain 1 faded turquoise bag.
drab aqua bags contain 1 faded black bag, 4 clear lavender bags.

 */

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(" bags contain ");
            let key = iter.next().unwrap().to_string();
            let mut contains = vec![];
            let rest = iter.next().unwrap();

            // bright yellow bags contain no other bags.
            if rest != "no other bags." {
                let rest = rest.replace(".", "").replace("bags", "").replace("bag", "");
                contains = rest
                    .split(", ")
                    .map(|bag| {
                        let pos = bag.find(' ').unwrap();
                        let num: usize = bag[0..pos].parse().unwrap();
                        (num, bag[pos..].trim().to_string())
                    })
                    .collect();
            }

            (key, contains)
        })
        .collect()
}

fn dfs(from: &str, data: &Data, goal: &str, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = cache.get(from) {
        return result;
    }
    for (_n, child) in &data[from] {
        if dfs(&child, data, goal, cache) {
            cache.insert(from.into(), true);
            return true;
        }
    }
    cache.insert(from.into(), false);
    return false;
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &Data) -> u32 {
    const GOAL: &str = "shiny gold";
    let mut cache = HashMap::new();
    cache.insert(GOAL.into(), true);
    let mut count = 0;
    for (key, _) in data {
        if key != GOAL && dfs(key, data, GOAL, &mut cache) {
            count += 1;
        }
    }
    count
}

// Number of children and one for the current bag.
fn count_children(from_bag: &str, data: &Data, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(&result) = cache.get(from_bag) {
        return result;
    }
    let mut children = 1;
    for (n, child) in &data[from_bag] {
        children += n * count_children(&child, data, cache);
    }
    cache.insert(from_bag.into(), children);
    children
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &Data) -> usize {
    count_children("shiny gold", data, &mut HashMap::new()) - 1
}
