use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Data {
    decks: [VecDeque<u8>; 2],
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Data {
    let mut iter = input.split("\n\n");
    let mut result = Data {
        decks: [VecDeque::new(), VecDeque::new()],
    };
    for i in 0..2 {
        let lines = iter.next().unwrap();
        result.decks[i] = lines.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    }
    result
}

fn play(decks: &mut [VecDeque<u8>; 2]) {
    let card0 = decks[0].pop_front().unwrap();
    let card1 = decks[1].pop_front().unwrap();
    if card1 < card0 {
        decks[0].push_back(card0);
        decks[0].push_back(card1);
    } else {
        decks[1].push_back(card1);
        decks[1].push_back(card0);
    }
}

fn calc_score(deck: &VecDeque<u8>) -> usize {
    let mut mul = deck.len();
    let mut acc = 0;
    for val in deck {
        acc += mul * *val as usize;
        mul -= 1;
    }
    acc
}

#[aoc(day22, part1)]
pub fn solve_part1(data: &Data) -> usize {
    let mut decks = data.decks.clone();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        play(&mut decks);
    }
    let deck = if decks[0].is_empty() {
        &mut decks[0]
    } else {
        &mut decks[1]
    };
    calc_score(&deck)
}

fn round(decks: &mut [VecDeque<u8>; 2], old_states: &mut HashSet<[Vec<u8>; 2]>) -> (bool, usize) {
    let state = [
        decks[0].iter().copied().collect(),
        decks[1].iter().copied().collect(),
    ];
    if old_states.contains(&state) {
        return (true, 0);
    } else {
        old_states.insert(state);
    }

    let card0 = decks[0].pop_front().unwrap();
    let card1 = decks[1].pop_front().unwrap();

    let winner = if decks[0].len() >= card0 as usize && decks[1].len() >= card1 as usize {
        // Recursive game
        let mut tmp_decks = [VecDeque::new(), VecDeque::new()];
        for i in 0..card0 as usize {
            tmp_decks[0].push_back(decks[0][i]);
        }
        for i in 0..card1 as usize {
            tmp_decks[1].push_back(decks[1][i]);
        }
        game(&mut tmp_decks)
    } else if card1 < card0 {
        0
    } else {
        1
    };
    if winner == 0 {
        decks[0].push_back(card0);
        decks[0].push_back(card1);
    } else {
        decks[1].push_back(card1);
        decks[1].push_back(card0);
    }

    if decks[0].is_empty() {
        (false, 1)
    } else if decks[1].is_empty() {
        (false, 0)
    } else {
        (false, 2) // No winner yet.
    }
}

// Returns the winner (0 or 1)
fn game(decks: &mut [VecDeque<u8>; 2]) -> usize {
    let mut old_states = HashSet::new();
    loop {
        let (p1, winner) = round(decks, &mut old_states);
        if p1 {
            // println!("P1 wins the game because of repetition");
            return winner;
        }
        if winner != 2 {
            // println!("P{} wins the game", winner + 1);
            return winner;
        }
    }
}

#[aoc(day22, part2)]
pub fn solve_part2(data: &Data) -> usize {
    let mut decks = data.decks.clone();
    let winner = game(&mut decks);
    if winner == 0 {
        calc_score(&decks[0])
    } else {
        calc_score(&decks[1])
    }
}
