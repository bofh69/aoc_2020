use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;
// use ::regex::*;

#[derive(Debug)]
pub struct Data {
    ingred: Vec<String>,
    alergen: Vec<String>,
}

/*
dim orange bags contain 1 faded turquoise bag.
drab aqua bags contain 1 faded black bag, 4 clear lavender bags.

 */

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(" (contains ");
            let ingred = iter.next().unwrap().to_string();
            let alergen = iter.next().unwrap();

            let ingred = ingred.split(' ').map(|s| s.to_string()).collect();
            let alergen = alergen.replace(')', "");
            let alergen = alergen.split(", ").map(|s| s.to_string()).collect();

            Data { ingred, alergen }
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let mut alergens = HashSet::new();
    let mut ingreds = HashSet::new();
    for food in data {
        for alergen in &food.alergen {
            alergens.insert(alergen.clone());
        }
        for ingred in &food.ingred {
            ingreds.insert(ingred.clone());
        }
    }
    // Ingreds to possible alergens:
    let mut ingred_to_pos_alergens = HashMap::new();
    for food in data {
        for ingred in &food.ingred {
            ingred_to_pos_alergens.insert(ingred, alergens.clone());
        }
    }

    for alergen in &alergens {
        let mut possible_ingreds = ingreds.clone();
        for food in data {
            if food.alergen.contains(alergen) {
                possible_ingreds = possible_ingreds
                    .iter()
                    .filter(|i| food.ingred.contains(i))
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        for (ingred, alergens) in &mut ingred_to_pos_alergens {
            if !possible_ingreds.contains(*ingred) {
                alergens.remove(alergen);
            }
        }
    }

    loop {
        let mut any_change = false;
        for ingred in &ingreds {
            if { ingred_to_pos_alergens[ingred].len() } == 1 {
                let alergen = {
                    ingred_to_pos_alergens[ingred]
                        .iter()
                        .next()
                        .unwrap()
                        .clone()
                };
                // Only one alergen, remove it from all other ingreds.
                for (ingred2, alergens) in &mut ingred_to_pos_alergens {
                    if *ingred2 != ingred && alergens.contains(&alergen) {
                        alergens.remove(&alergen);
                        any_change = true;
                    }
                }
            }
        }
        if !any_change {
            break;
        }
    }

    let safe_ingreds: HashSet<_> = ingred_to_pos_alergens
        .iter()
        .filter(|(_i, a)| a.is_empty())
        .map(|(i, _a)| i)
        .collect();

    let mut count = 0;
    for food in data {
        for ingred in &safe_ingreds {
            if food.ingred.contains(ingred) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day21, part2)]
pub fn solve_part2(data: &[Data]) -> String {
    let mut alergens = HashSet::new();
    let mut ingreds = HashSet::new();
    for food in data {
        for alergen in &food.alergen {
            alergens.insert(alergen.clone());
        }
        for ingred in &food.ingred {
            ingreds.insert(ingred.clone());
        }
    }
    // Ingreds to possible alergens:
    let mut ingred_to_pos_alergens = HashMap::new();
    for food in data {
        for ingred in &food.ingred {
            ingred_to_pos_alergens.insert(ingred, alergens.clone());
        }
    }

    for alergen in &alergens {
        let mut possible_ingreds = ingreds.clone();
        for food in data {
            if food.alergen.contains(alergen) {
                possible_ingreds = possible_ingreds
                    .iter()
                    .filter(|i| food.ingred.contains(i))
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        for (ingred, alergens) in &mut ingred_to_pos_alergens {
            if !possible_ingreds.contains(*ingred) {
                alergens.remove(alergen);
            }
        }
    }

    loop {
        let mut any_change = false;
        for ingred in &ingreds {
            if { ingred_to_pos_alergens[ingred].len() } == 1 {
                let alergen = {
                    ingred_to_pos_alergens[ingred]
                        .iter()
                        .next()
                        .unwrap()
                        .clone()
                };
                // Only one alergen, remove it from all other ingreds.
                for (ingred2, alergens) in &mut ingred_to_pos_alergens {
                    if *ingred2 != ingred && alergens.contains(&alergen) {
                        alergens.remove(&alergen);
                        any_change = true;
                    }
                }
            }
        }
        if !any_change {
            break;
        }
    }

    let mut tmp: Vec<(&str, &str)> = ingred_to_pos_alergens
        .iter()
        .filter(|(_i, a)| a.len() == 1)
        .map(|(i, a)| (i.as_str(), a.iter().next().unwrap().as_str()))
        .collect();
    tmp.sort_by_key(|(_i, a)| *a);
    let s: Vec<_> = tmp.iter().map(|(i, _a)| *i).collect();
    s.join(",")
}
