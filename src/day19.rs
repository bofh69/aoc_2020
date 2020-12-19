use ::regex::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Rule {
    Char(bool),
    Seq(Vec<u8>),
    Or(Vec<u8>, Vec<u8>),
}

#[derive(Debug)]
pub struct Data {
    rules: HashMap<u8, Rule>,
    msgs: Vec<Vec<bool>>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Data {
    let mut iter = input.lines();

    let mut rules = HashMap::new();

    let re_char = Regex::new("^([0-9]+): \"(.)\"").unwrap();
    let re_or = Regex::new("^([0-9]+):(( [0-9]+)+) \\|(( [0-9]+)+)$").unwrap();
    let re_seq = Regex::new("^([0-9]+):(( [0-9]+)+)$").unwrap();

    loop {
        let line = iter.next().unwrap();
        if line == "" {
            break;
        }
        if let Some(caps) = re_char.captures(line) {
            let n = caps.get(1).unwrap().as_str().parse().unwrap();
            let c = caps.get(2).unwrap().as_str().chars().next().unwrap();
            rules.insert(n, Rule::Char(c == 'a'));
        } else if let Some(caps) = re_or.captures(line) {
            let n = caps.get(1).unwrap().as_str().parse().unwrap();
            let seq1 = caps.get(2).unwrap().as_str();
            let seq2 = caps.get(4).unwrap().as_str();

            rules.insert(
                n,
                Rule::Or(
                    seq1.trim().split(" ").map(|s| s.parse().unwrap()).collect(),
                    seq2.trim().split(" ").map(|s| s.parse().unwrap()).collect(),
                ),
            );
        } else if let Some(caps) = re_seq.captures(line) {
            let n = caps.get(1).unwrap().as_str().parse().unwrap();
            let seq = caps.get(2).unwrap().as_str();

            rules.insert(
                n,
                Rule::Seq(seq.trim().split(" ").map(|s| s.parse().unwrap()).collect()),
            );
        } else {
            panic!("Unknown rule");
        }
    }

    Data {
        rules,
        msgs: iter
            .map(|s| s.chars().map(|c| c == 'a').collect())
            .collect(),
    }
}

fn print_rules(rule: u8, rules: &HashMap<u8, Rule>) {
    if rule == 8 {
        print!("(");
        print_rules(42, rules);
        print!(")+");
    } else if rule == 11 {
        print_rules(42, rules);
        print!("XXX");
        print_rules(31, rules);
    } else {
    match rules.get(&rule) {
        Some(Rule::Char(a)) => {
            if *a {
                print!("a");
            } else {
                print!("b");
            }
        }
        Some(Rule::Seq(seq)) => {
            for rule in seq {
                print_rules(*rule, rules);
            }
        }
        Some(Rule::Or(seq1, seq2)) => {
            print!("((");
            for rule in seq1 {
                print_rules(*rule, rules);
            }
            print!(")|(");
            for rule in seq2 {
                print_rules(*rule, rules);
            }
            print!("))");
        }
        _ => panic!("Unkown"),
    }
    }
}

fn is_matching(rules: &HashMap<u8, Rule>, msg: &[bool]) -> bool {
    // Convert rules to NFA

    print!("^");
    print_rules(0, rules);
    println!("$");

    false
}

#[aoc(day19, part1)]
pub fn solve_part1(data: &Data) -> usize {
    0
}

#[aoc(day19, part2)]
pub fn solve_part2(_data: &Data) -> usize {
    let prefix = "(((a((b((((((b((b((bb)|(a((b)|(a)))))|(a((ab)|(bb)))))|(a((a((b)|(a))((b)|(a)))|(baa))))b)|(((((((aa)|(b((b)|(a))))a)|(((bb)|(a((b)|(a))))b))a)|(((a((ba)|(((b)|(a))b)))|(b((aa)|(ba))))b))a))a)|(((((bbaa)|(((baa)|(((bb)|(aa))b))b))a)|(((((aba)|(aab))b)|(((bba)|(a((ba)|(bb))))a))b))b)))|(a((a((((a((b)|(a))((bb)|(a((b)|(a)))))|(b((baa)|(abb))))b)|(((b((ab)|(bb))a)|(((((ba)|(bb))b)|(((ba)|(((b)|(a))b))a))b))a)))|(b((b((((a((bb)|(a((b)|(a)))))|(baa))a)|(((((b)|(a))((b)|(a))a)|(((aa)|(ba))b))b)))|(a((b((b((bb)|(a((b)|(a)))))|(a((ab)|(bb)))))|(a((bbb)|(((ba)|(bb))a)))))))))))|(b((((((b((b((bbb)|(a((ba)|(((b)|(a))b)))))|(a((((aa)|(b((b)|(a))))b)|(((aa)|(ba))a)))))|(a((((b)|(a))((bb)|(a((b)|(a))))a)|(((((ba)|(((b)|(a))b))a)|(bbb))b))))b)|(((a((a((abb)|(aaa)))|(b((b)|(a))((b)|(a))((b)|(a)))))|(b((((((ba)|(bb))b)|(bba))b)|(((((aa)|(ba))a)|(bbb))a))))a))b)|(((((a((a((((ba)|(ab))b)|(((aa)|(b((b)|(a))))a)))|(b((bba)|(a((b)|(a))((b)|(a)))))))|(b((((((ab)|(bb))a)|(bab))a)|(((a((ab)|(bb)))|(bab))b))))a)|(((b((b((a((ba)|(((b)|(a))b)))|(b((ba)|(ab)))))|(a((((aa)|(ba))a)|(aab)))))|(a((a((((aa)|(ba))b)|(baa)))|(b((bba)|(a((ba)|(bb))))))))b))a)))))";
    let suffix = "((a((b((((a((((aab)|(baa))a)|(((((ba)|(bb))b)|(bba))b)))|(b((((bbb)|(a((ba)|(bb))))a)|(((bba)|(a((aa)|(ba))))b))))a)|(((((((((aa)|(b((b)|(a))))b)|(((aa)|(ba))a))a)|(((((ba)|(((b)|(a))b))b)|(((aa)|(b((b)|(a))))a))b))b)|(((((b((bb)|(a((b)|(a)))))|(a((b)|(a))((b)|(a))))a)|(((a((aa)|(b((b)|(a)))))|(b((ba)|(ab))))b))a))b)))|(a((b((b((b((((ba)|(((b)|(a))b))a)|(bbb)))|(a((bbb)|(baa)))))|(a((((aab)|(((ab)|(aa))a))b)|(((((aa)|(b((b)|(a))))b)|(((aa)|(ba))a))a)))))|(a((((a((b((aa)|(b((b)|(a)))))|(a((b)|(a))((b)|(a)))))|(b((baa)|(aaa))))a)|(((((bbb)|(((ba)|(bb))a))b)|(((b((bb)|(a((b)|(a)))))|(a((ab)|(bb))))a))b)))))))|(b((a((a((((b((((ba)|(bb))b)|(bba)))|(a((b((bb)|(a((b)|(a)))))|(a((ba)|(ab))))))a)|(((((a((ba)|(ab)))|(b((b)|(a))((b)|(a))))a)|(((b((bb)|(aa)))|(a((ab)|(bb))))b))b)))|(b((((baa)|(abb))bb)|(((((((aa)|(b((b)|(a))))b)|(((aa)|(ba))a))b)|(((((ba)|(bb))b)|(baa))a))a)))))|(b((((((a((((aa)|(ba))b)|(((bb)|(a((b)|(a))))a)))|(b((((bb)|(aa))a)|(((aa)|(b((b)|(a))))b))))a)|(((a((b((ba)|(((b)|(a))b)))|(a((ba)|(bb)))))|(b((((aa)|(ba))a)|(bbb))))b))a)|(((((b((bab)|(((aa)|(b((b)|(a))))a)))|(a((bba)|(a((ba)|(bb))))))b)|(((((a((ba)|(((b)|(a))b)))|(b((ba)|(ab))))a)|(((a((b)|(a))((b)|(a)))|(baa))b))a))b))))))";
    for i in 1..50 {
        println!("^{}{{{},}}{}{{{}}}$", prefix, i+1, suffix, i);
    }
    0
}
