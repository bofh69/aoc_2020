use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

const IMG_WIDTH: usize = 12;
const IMG_HEIGHT: usize = 12;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MAX_TILE_VARIANTS: usize = 4 * 4;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

#[derive(Debug)]
pub struct Data {
    num: i32,
    data: [bool; WIDTH * HEIGHT],
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut iter = tile.lines();
            let num = iter.next().unwrap();
            let num = num
                .split(' ')
                .skip(1)
                .next()
                .unwrap()
                .split(':')
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let mut data = [false; WIDTH * HEIGHT];
            for (i, v) in iter
                .map(|s| s.chars())
                .flatten()
                .map(|c| c == '#')
                .enumerate()
            {
                data[i] = v;
            }

            Data { num, data }
        })
        .collect()
}

fn rev_side_val(side: u16) -> u16 {
    side.reverse_bits() >> (16 - WIDTH)
}

fn side_values(tile: &Data) -> [[u16; 4]; MAX_TILE_VARIANTS] {
    let mut result: [[u16; 4]; MAX_TILE_VARIANTS] = [[0; 4]; MAX_TILE_VARIANTS];
    // Set top/bottom
    for x in 0..WIDTH {
        if tile.data[x] {
            result[0][UP] |= 1 << x;
        }
        if tile.data[x + WIDTH * (HEIGHT - 1)] {
            result[0][DOWN] |= 1 << x;
        }
    }
    // Set left and right sides
    for y in 0..HEIGHT {
        if tile.data[y * WIDTH] {
            result[0][LEFT] |= 1 << y;
        }
        if tile.data[y * WIDTH + (WIDTH - 1)] {
            result[0][RIGHT] |= 1 << y;
        }
    }

    // Rotate
    for rot in 1..4 {
        result[rot][UP] = rev_side_val(result[rot - 1][LEFT]);
        result[rot][RIGHT] = result[rot - 1][UP];
        result[rot][DOWN] = rev_side_val(result[rot - 1][RIGHT]);
        result[rot][LEFT] = result[rot - 1][DOWN];
    }

    // Mirror
    for tile in 0..4 {
        result[tile + 4][UP] = rev_side_val(result[tile][UP]);
        result[tile + 4][DOWN] = rev_side_val(result[tile][DOWN]);
        result[tile + 4][LEFT] = result[tile][RIGHT];
        result[tile + 4][RIGHT] = result[tile][LEFT];
    }
    // Upside down:
    for tile in 0..4 * 2 {
        result[tile + 4 * 2][LEFT] = rev_side_val(result[tile][LEFT]);
        result[tile + 4 * 2][RIGHT] = rev_side_val(result[tile][RIGHT]);
        result[tile + 4 * 2][UP] = result[tile][DOWN];
        result[tile + 4 * 2][DOWN] = result[tile][UP];
    }

    result
}

#[aoc(day20, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let tiles: HashMap<i32, Vec<[u16; 4]>> = data
        .iter()
        .map(|tile| {
            let num = tile.num;
            let result: HashSet<_> = side_values(tile).iter().map(|&s| s).collect();
            (num, result.iter().copied().collect())
        })
        .collect();

    let mut sides_to_num: HashMap<u16, HashSet<(i32, usize, usize)>> = HashMap::new();

    for (num, rotations) in &tiles {
        for (n, sides) in rotations.iter().enumerate() {
            for side in 0..4 {
                sides_to_num
                    .entry(sides[side])
                    .or_insert(HashSet::new())
                    .insert((*num, n, side));
            }
        }
    }

    // Num, rotation, dir => num, rotation
    let mut neighbours: HashMap<(i32, usize, usize), HashSet<(i32, usize)>> = HashMap::new();
    let mut num_rots = HashSet::new();

    for values in sides_to_num.values() {
        for (num, rot, side) in values {
            num_rots.insert((*num, *rot));
            let entry = neighbours
                .entry((*num, *rot, *side))
                .or_insert(HashSet::new());
            for (num2, rot2, side2) in values {
                if num != num2 {
                    if *side == ((side2 + 2) % 4) {
                        entry.insert((*num2, *rot2));
                    }
                }
            }
        }
    }

    let mut result = HashSet::new();
    for (num, tile) in &num_rots {
        if neighbours.get(&(*num, *tile, UP)).unwrap().len() == 0
            && neighbours.get(&(*num, *tile, LEFT)).unwrap().len() == 0
            && neighbours.get(&(*num, *tile, DOWN)).unwrap().len() > 0
            && neighbours.get(&(*num, *tile, RIGHT)).unwrap().len() > 0
        {
            result.insert(*num);
        }
    }
    assert_eq!(4, result.len());

    result.iter().map(|s| *s as usize).product()
}

fn remove_from_neigh(
    num: i32,
    neighbours: &mut HashMap<(i32, usize, usize), HashSet<(i32, usize)>>,
) {
    for neigh in neighbours.values_mut() {
        for i in 0..MAX_TILE_VARIANTS {
            neigh.remove(&(num, i));
        }
    }
}

fn fits(image: &[Option<(i32, usize)>; IMG_WIDTH * IMG_HEIGHT],
        tile: &(i32, usize),
        neighbours: &HashMap<(i32, usize, usize), HashSet<(i32, usize)>>,
        x: usize, y: usize) -> bool {
    let mut is_possible = false;
    if y == 0 {
        return false;
    } else {
        if let Some(above) = image[x + (y-1)*IMG_WIDTH] {
        } else {
            return false;
        }
    }
    is_possible
}

#[aoc(day20, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    let tiles: HashMap<i32, Vec<[u16; 4]>> = data
        .iter()
        .map(|tile| {
            let num = tile.num;
            let result: HashSet<_> = side_values(tile).iter().map(|&s| s).collect();
            (num, result.iter().copied().collect())
        })
        .collect();

    let mut sides_to_num: HashMap<u16, HashSet<(i32, usize, usize)>> = HashMap::new();

    for (num, rotations) in &tiles {
        for (n, sides) in rotations.iter().enumerate() {
            for side in 0..4 {
                sides_to_num
                    .entry(sides[side])
                    .or_insert(HashSet::new())
                    .insert((*num, n, side));
            }
        }
    }

    // Num, rotation, dir => num, rotation
    let mut neighbours: HashMap<(i32, usize, usize), HashSet<(i32, usize)>> = HashMap::new();
    let mut num_rots = HashSet::new();

    for values in sides_to_num.values() {
        for (num, rot, side) in values {
            num_rots.insert((*num, *rot));
            let entry = neighbours
                .entry((*num, *rot, *side))
                .or_insert(HashSet::new());
            for (num2, rot2, side2) in values {
                if num != num2 {
                    if *side == ((side2 + 2) % 4) {
                        entry.insert((*num2, *rot2));
                    }
                }
            }
        }
    }

    let mut image = [None; IMG_WIDTH * IMG_HEIGHT];
    for (num, tile) in &num_rots {
        if neighbours.get(&(*num, *tile, UP)).unwrap().len() == 0
            && neighbours.get(&(*num, *tile, LEFT)).unwrap().len() == 0
            && neighbours.get(&(*num, *tile, DOWN)).unwrap().len() > 0
            && neighbours.get(&(*num, *tile, RIGHT)).unwrap().len() > 0
        {
            image[0] = Some((*num, *tile));
            remove_from_neigh(*num, &mut neighbours);
            break;
        }
    }

    loop {
        let mut any = false;
        for y in 0..IMG_HEIGHT - 1 {
            for x in 0..IMG_WIDTH - 1 {
                if let Some((num, tile)) = image[x + y * IMG_WIDTH] {
                    if image[x + 1 + y * IMG_WIDTH].is_none() {
                        if let Some(neighs) = neighbours.get(&(num, tile, RIGHT)) {
                            for possible in neighs {
                                if fits(&image, possible, &neighbours, x + 1, y) {
                                    image[x + 1 + y * IMG_WIDTH] = Some((possible.0, possible.1));
                                    remove_from_neigh(possible.0, &mut neighbours);
                                    any = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        if !any {
            break;
        }
    }
    dbg!(&image);

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inv() {
        assert_eq!(5, rev_side_val(rev_side_val(5)));
        assert!(5 != rev_side_val(5));
        assert_eq!(512, rev_side_val(1));
    }
}
