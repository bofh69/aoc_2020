use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::collections::HashSet;

fn create_tiles(input: &str) -> HashMap<(i32, i32), bool> {
    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();
    for line in input.lines() {
        let mut line = line.chars();
        let mut pos = (0, 0);
        while let Some(c) = line.next() {
            match c {
                'e' => pos.0 += 1,
                'w' => pos.0 -= 1,
                'n' => {
                    pos.1 -= 1;
                    if Some('w') == line.next() {
                        pos.0 -= 1;
                    }
                }
                's' => {
                    pos.1 += 1;
                    if Some('e') == line.next() {
                        pos.0 += 1;
                    }
                }
                _ => unreachable!("Unknown char in string"),
            }
        }
        tiles
            .entry(pos)
            .and_modify(|old| *old = !*old)
            .or_insert(true);
    }
    tiles
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &str) -> usize {
    create_tiles(input).into_values().filter(|t| *t).count()
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut current_tiles: HashSet<_> = create_tiles(input)
        .iter()
        .filter_map(|(pos, t)| if *t { Some(*pos) } else { None })
        .collect();
    let mut next_tiles = HashSet::new();
    for day in 1..=100 {
        next_tiles.clear();
        for pos in current_tiles.iter() {
            let mut count_black = 0;
            for (dx, dy) in [(-1, 0), (1, 0), (-1, -1), (0, -1), (1, 1), (0, 1)] {
                let pos = (pos.0 + dx, pos.1 + dy);
                if current_tiles.contains(&pos) {
                    count_black += 1;
                } else {
                    // White
                    let mut count_black = 0;
                    for (dx, dy) in [(-1, 0), (1, 0), (-1, -1), (0, -1), (1, 1), (0, 1)] {
                        let pos = (pos.0 + dx, pos.1 + dy);
                        if current_tiles.contains(&pos) {
                            count_black += 1;
                        }
                    }
                    if count_black == 2 {
                        next_tiles.insert(pos);
                    }
                }
            }
            // black
            /*
            println!(
                "{}.{} is black and has {} black around it",
                pos.0, pos.1, count_black
            );
            */
            if count_black == 1 || count_black == 2 {
                next_tiles.insert(*pos);
            }
        }
        std::mem::swap(&mut current_tiles, &mut next_tiles);
        println!("Day {}: {}", day, current_tiles.len());
    }
    // < 4184
    current_tiles.len()
}
