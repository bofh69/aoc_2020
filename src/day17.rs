use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Data = Vec<bool>;
type Pos = i32;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unknown tile"),
                })
                .collect()
        })
        .collect()
}

fn count_neighbours(space: &HashSet<(Pos, Pos, Pos)>, coord: (Pos, Pos, Pos)) -> u8 {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let (x, y, z) = (x + coord.0, y + coord.1, z + coord.2);
                if (x, y, z) != coord && is_active(space, &(x, y, z)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_active(space: &HashSet<(Pos, Pos, Pos)>, coord: &(Pos, Pos, Pos)) -> bool {
    space.contains(coord)
}

#[aoc(day17, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let mut space = HashSet::new();
    let height = data.len() as Pos;
    let width = data[0].len() as Pos;
    for y in 0..height {
        for x in 0..width {
            if data[y as usize][x as usize] {
                space.insert((x, y, 0));
            }
        }
    }

    let mut min_coord = (0, 0, 0);
    let mut max_coord = (width - 1, height - 1, 0);
    for _generation in 1..=6 {
        let mut new_min = max_coord;
        let mut new_max = min_coord;
        let mut next = HashSet::new();
        for x in min_coord.0 - 1..=max_coord.0 + 1 {
            for y in min_coord.1 - 1..=max_coord.1 + 1 {
                for z in min_coord.2 - 1..=max_coord.2 + 1 {
                    let neighbours = count_neighbours(&space, (x, y, z));
                    let mut next_state = false;
                    if is_active(&space, &(x, y, z)) {
                        if neighbours == 2 || neighbours == 3 {
                            next_state = true;
                        }
                    } else if neighbours == 3 {
                        next_state = true;
                    }
                    if next_state {
                        if x < new_min.0 {
                            new_min.0 = x;
                        }
                        if x > new_max.0 {
                            new_max.0 = x;
                        }
                        if y < new_min.1 {
                            new_min.1 = y;
                        }
                        if y > new_max.1 {
                            new_max.1 = y;
                        }
                        if z < new_min.2 {
                            new_min.2 = z;
                        }
                        if z > new_max.2 {
                            new_max.2 = z;
                        }
                        next.insert((x, y, z));
                    }
                }
            }
        }
        min_coord = new_min;
        max_coord = new_max;
        space = next;
    }

    space.len()
}

fn count_neighbours2(space: &HashSet<(Pos, Pos, Pos, Pos)>, coord: (Pos, Pos, Pos, Pos)) -> u8 {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    let (x, y, z, w) = (x + coord.0, y + coord.1, z + coord.2, w + coord.3);
                    if (x, y, z, w) != coord && is_active2(space, &(x, y, z, w)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn is_active2(space: &HashSet<(Pos, Pos, Pos, Pos)>, coord: &(Pos, Pos, Pos, Pos)) -> bool {
    space.contains(coord)
}

#[aoc(day17, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    let mut space = HashSet::new();
    let height = data.len() as Pos;
    let width = data[0].len() as Pos;
    for y in 0..height {
        for x in 0..width {
            if data[y as usize][x as usize] {
                space.insert((x, y, 0, 0));
            }
        }
    }

    let mut min_coord = (0, 0, 0, 0);
    let mut max_coord = (width - 1, height - 1, 0, 0);
    for _generation in 1..=6 {
        let mut new_min = max_coord;
        let mut new_max = min_coord;
        let mut next = HashSet::new();
        for x in min_coord.0 - 1..=max_coord.0 + 1 {
            for y in min_coord.1 - 1..=max_coord.1 + 1 {
                for z in min_coord.2 - 1..=max_coord.2 + 1 {
                    for w in min_coord.3 - 1..=max_coord.3 + 1 {
                        let neighbours = count_neighbours2(&space, (x, y, z, w));
                        let mut next_state = false;
                        if is_active2(&space, &(x, y, z, w)) {
                            if neighbours == 2 || neighbours == 3 {
                                next_state = true;
                            }
                        } else if neighbours == 3 {
                            next_state = true;
                        }
                        if next_state {
                            if x < new_min.0 {
                                new_min.0 = x;
                            }
                            if x > new_max.0 {
                                new_max.0 = x;
                            }
                            if y < new_min.1 {
                                new_min.1 = y;
                            }
                            if y > new_max.1 {
                                new_max.1 = y;
                            }
                            if z < new_min.2 {
                                new_min.2 = z;
                            }
                            if z > new_max.2 {
                                new_max.2 = z;
                            }
                            if w < new_min.3 {
                                new_min.3 = w;
                            }
                            if w > new_max.3 {
                                new_max.3 = w;
                            }
                            next.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
        min_coord = new_min;
        max_coord = new_max;
        space = next;
    }
    space.len()
}
