use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
// use ::regex::*;

type Data = Vec<HashMap<String, String>>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Data {
    input
        .split("\n\n")
        .map(|entry| {
            entry
                .split(|c| c == ' ' || c == '\n')
                .map(|entry| {
                    let mut i = entry.split(':');
                    (i.next().unwrap().into(), i.next().unwrap().into())
                })
                .collect::<HashMap<String, String>>()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &Data) -> usize {
    /*
     * byr (Birth Year)
     * iyr (Issue Year)
     * eyr (Expiration Year)
     * hgt (Height)
     * hcl (Hair Color)
     * ecl (Eye Color)
     * pid (Passport ID)
     */
    data.iter()
        .filter(|entries| entries.contains_key("byr"))
        .filter(|entries| entries.contains_key("iyr"))
        .filter(|entries| entries.contains_key("eyr"))
        .filter(|entries| entries.contains_key("hgt"))
        .filter(|entries| entries.contains_key("hcl"))
        .filter(|entries| entries.contains_key("ecl"))
        .filter(|entries| entries.contains_key("pid"))
        .count()
}

fn is_num_between(entry: &str, min: u32, max: u32) -> bool {
    if let Ok(num) = entry.parse::<u32>() {
        num >= min && num <= max
    } else {
        false
    }
}
fn is_entry_between(entries: &HashMap<String, String>, name: &str, min: u32, max: u32) -> bool {
    if let Some(entry) = entries.get(name) {
        is_num_between(entry, min, max)
    } else {
        false
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &Data) -> usize {
    data.iter()
        .filter(|entries| {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            is_entry_between(entries, "byr", 1920, 2002)
        })
        .filter(|entries| {
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            is_entry_between(entries, "iyr", 2010, 2020)
        })
        .filter(|entries| {
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            is_entry_between(entries, "eyr", 2020, 2030)
        })
        .filter(|entries| {
            if let Some(entry) = entries.get("hgt") {
                //  hgt (Height) - a number followed by either cm or in:
                // If in, the number must be at least 59 and at most 76.
                // If cm, the number must be at least 150 and at most 193.

                if entry.ends_with("in") {
                    let (entry, _) = entry.split_at(entry.len() - 2);
                    is_num_between(entry, 59, 76)
                } else if entry.ends_with("cm") {
                    let (entry, _) = entry.split_at(entry.len() - 2);
                    is_num_between(entry, 150, 193)
                } else {
                    false
                }
            } else {
                false
            }
        })
        .filter(|entries| {
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            if let Some(entry) = entries.get("hcl") {
                if entry.len() == 7 && entry.starts_with("#") {
                    entry
                        .chars()
                        .skip(1)
                        // is_digit or is_ascii_lowercase ?
                        .filter(char::is_ascii_hexdigit)
                        .count()
                        == 6
                } else {
                    false
                }
            } else {
                false
            }
        })
        .filter(|entries| {
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            if let Some(entry) = entries.get("ecl") {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&entry.as_str())
            } else {
                false
            }
        })
        .filter(|entries| entries.contains_key("pid"))
        .filter(|entries| {
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            if let Some(entry) = entries.get("pid") {
                entry.chars().filter(char::is_ascii_digit).count() == 9
            } else {
                false
            }
        })
        .count()
}
