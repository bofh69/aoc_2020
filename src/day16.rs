use ::regex::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Data {
    // name, from-to, from-to
    pub rules: Vec<(String, u16, u16, u16, u16)>,
    pub ticket: Vec<u16>,
    pub nearby_tickets: Vec<Vec<u16>>,
    /*
    class: 1-3 or 5-7
    row: 6-11 or 33-44

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
     */
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let mut iter = input.lines();
    let re = Regex::new("^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$")?;
    let mut rules = vec![];
    loop {
        let s = iter.next().unwrap();
        if let Some(caps) = re.captures(s) {
            rules.push((
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
                caps.get(5).unwrap().as_str().parse().unwrap(),
            ));
        } else {
            break;
        }
    }
    iter.next();
    let ticket = iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    iter.next();
    iter.next();

    let mut nearby_tickets = vec![];
    for line in iter {
        nearby_tickets.push(line.split(',').map(|n| n.parse().unwrap()).collect());
    }
    Ok(Data {
        rules,
        ticket,
        nearby_tickets,
    })
}

#[aoc(day16, part1)]
pub fn solve_part1(data: &Data) -> usize {
    let mut valid_numbers = HashSet::new();
    for r in &data.rules {
        for i in r.1..=r.2 {
            valid_numbers.insert(i);
        }
        for i in r.3..=r.4 {
            valid_numbers.insert(i);
        }
    }
    data.nearby_tickets
        .iter()
        .flatten()
        .filter(|n| !valid_numbers.contains(n))
        .map(|&n| n as usize)
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(data: &Data) -> usize {
    let mut valid_numbers = HashSet::new();
    for r in &data.rules {
        for i in r.1..=r.2 {
            valid_numbers.insert(i);
        }
        for i in r.3..=r.4 {
            valid_numbers.insert(i);
        }
    }

    let all_rules: HashSet<_> = (0..data.rules.len()).collect();
    let mut all_rules: Vec<_> = (0..data.rules.len()).map(|_| all_rules.clone()).collect();

    let valid_tickets: Vec<_> = data
        .nearby_tickets
        .iter()
        .filter(|nt| {
            !nt.iter().any(|n| !valid_numbers.contains(n))
        })
        .collect();

    for ticket in valid_tickets {
        for (i, n) in ticket.iter().enumerate() {
            for j in 0..data.rules.len() {
                if !(*n >= data.rules[j].1 && *n <= data.rules[j].2
                    || *n >= data.rules[j].3 && *n <= data.rules[j].4)
                {
                    all_rules[j].remove(&i);
                }
            }
        }
    }

    loop {
        let mut removed_any = false;
        for field in 0..all_rules.len() {
            if all_rules[field].len() == 1 {
                let rule = *all_rules[field].iter().next().unwrap();
                for i in 0..all_rules.len() {
                    if i != field && all_rules[i].remove(&rule) {
                        removed_any = true;
                    }
                }
            }
        }
        if !removed_any {
            break;
        }
    }

    // all_rules[rule] = field
    let all_rules: HashMap<_, _> = all_rules
        .iter()
        .enumerate()
        .map(|(r, n)| (r, n.iter().next().unwrap()))
        .collect();

    dbg!(&all_rules, &data.rules, &data.ticket);

    data.rules
        .iter()
        .enumerate()
        .filter(|(_n, v)| v.0.starts_with("departure"))
        .map(|(r, s)| {
            dbg!(r, s, all_rules[&r]);
            dbg!(data.ticket[*all_rules[&r]] as usize)
        })
        .product()
}
