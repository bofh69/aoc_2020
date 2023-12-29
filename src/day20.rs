use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;
use std::collections::HashSet;
//use itertools::*;

const TILE_WIDTH: usize = 10;
const TILE_HEIGHT: usize = 10;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Rotation(u8);

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct TileNum(u16);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Tile {
    num: TileNum,
    data: Vec<Vec<bool>>,
}

// Stores the side of a Tile. Left to Right or Top to Bottom
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct SideValue(u16);

impl SideValue {
    fn reverse(&self) -> Self {
        Self(self.0.reverse_bits() >> (u16::BITS - TILE_WIDTH as u32))
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct SideValues {
    up: SideValue,
    down: SideValue,
    left: SideValue,
    right: SideValue,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct PossibleNeighbors {
    up: Vec<(TileNum, Rotation)>,
    down: Vec<(TileNum, Rotation)>,
    left: Vec<(TileNum, Rotation)>,
    right: Vec<(TileNum, Rotation)>,
}

impl From<&Tile> for SideValues {
    fn from(value: &Tile) -> Self {
        let mut up = 0;
        for v in &value.data[0] {
            up <<= 1;
            if *v {
                up |= 1;
            }
        }
        let mut down = 0;
        for v in &value.data[TILE_HEIGHT - 1] {
            down <<= 1;
            if *v {
                down |= 1;
            }
        }
        let mut left = 0;
        for y in 0..value.data.len() {
            let v = value.data[y][0];
            left <<= 1;
            if v {
                left |= 1;
            }
        }
        let mut right = 0;
        for y in 0..value.data.len() {
            let v = value.data[y][TILE_WIDTH - 1];
            right <<= 1;
            if v {
                right |= 1;
            }
        }
        Self {
            up: SideValue(up),
            down: SideValue(down),
            left: SideValue(left),
            right: SideValue(right),
        }
    }
}

impl SideValues {
    /*
    fn mirror(&self) -> Self {
        Self {
            up: self.up.reverse(),
            down: self.down.reverse(),
            left: self.right,
            right: self.left,
        }
    }
    */

    fn flip_upside_down(&self) -> Self {
        Self {
            up: self.down,
            down: self.up,
            left: self.left.reverse(),
            right: self.right.reverse(),
        }
    }

    fn turn_clockwise(&self) -> Self {
        Self {
            up: self.left.reverse(),
            right: self.up,
            down: self.right.reverse(),
            left: self.down,
        }
    }

    fn _in(&self, list: &[(Rotation, Self)]) -> bool {
        for (_, old_tile) in list {
            if *old_tile == *self {
                return true;
            }
        }
        false
    }

    fn get_rotation(&self, num: u8) -> Self {
        if num == 0 {
            return *self;
        }
        let mut new_tile = *self;
        if num & 4 != 0 {
            new_tile = new_tile.flip_upside_down();
        }
        for _ in 0..(num & 3) {
            new_tile = new_tile.turn_clockwise();
        }
        new_tile
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Tile> {
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

            let data = iter
                .map(|s| s.chars().map(|c| c == '#').collect())
                .collect();

            Tile {
                num: TileNum(num),
                data,
            }
        })
        .collect()
}

fn create_tiles_from_data(data: &[Tile]) -> HashMap<(TileNum, Rotation), SideValues> {
    data.iter()
        .map(|t| {
            let sv = SideValues::from(t);
            (0..8)
                .map(|n| ((t.num, Rotation(n)), sv.get_rotation(n)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn create_sideview_to_tilenum(
    tiles: &HashMap<(TileNum, Rotation), SideValues>,
) -> HashMap<SideValue, Vec<(TileNum, Rotation)>> {
    let mut sv_to_tn: HashMap<SideValue, Vec<(TileNum, Rotation)>> = HashMap::new();
    for (tr, sv) in tiles {
        let list = sv_to_tn.entry(sv.up).or_insert_with(|| Default::default());
        if !list.contains(tr) {
            list.push(*tr);
        }
        let list = sv_to_tn
            .entry(sv.down)
            .or_insert_with(|| Default::default());
        if !list.contains(tr) {
            list.push(*tr);
        }
        let list = sv_to_tn
            .entry(sv.left)
            .or_insert_with(|| Default::default());
        if !list.contains(tr) {
            list.push(*tr);
        }
        let list = sv_to_tn
            .entry(sv.right)
            .or_insert_with(|| Default::default());
        if !list.contains(tr) {
            list.push(*tr);
        }
    }
    sv_to_tn
}

fn fill_map(
    map: &mut Vec<char>,
    x: usize,
    y: usize,
    width: usize,
    tiles: &[Tile],
    tile_rot: (TileNum, Rotation),
) {
    for tile in tiles {
        if tile.num == tile_rot.0 {
            for dy in 0..8 {
                for dx in 0..8 {
                    let mut sx = dx;
                    let mut sy = dy;
                    for _ in 0..tile_rot.1 .0 & 3 {
                        (sy, sx) = (7 - sx, sy)
                    }
                    if tile_rot.1 .0 & 4 != 0 {
                        sy = 7 - sy;
                    }
                    let didx = x * 8 + dx + (y * 8 + dy) * width * 8;
                    map[didx] = if tile.data[sy + 1][sx + 1] { '#' } else { '.' }
                }
            }
            return;
        }
    }
    unreachable!("The tile should exist")
}

fn flip_map(map: &mut Vec<char>, width: usize) {
    let from = map.clone();
    map.clear();
    for y in 0..width {
        for x in 0..width {
            if from[(width - y - 1) * width + x] != '.' {
                map.push('#');
            } else {
                map.push('.');
            }
        }
    }
}

fn rotate_map(map: &mut Vec<char>, width: usize) {
    let from = map.clone();
    map.clear();
    for y in 0..width {
        for x in 0..width {
            let sy = width - x - 1;
            let sx = y;
            if from[sx + sy * width] != '.' {
                map.push('#');
            } else {
                map.push('.');
            }
        }
    }
}

#[aoc(day20, part1)]
pub fn solve_part1(data: &[Tile]) -> usize {
    let tiles = create_tiles_from_data(data);

    // SideValue -> Vec<(TileNum, Rotation>>
    let _sv_to_tn = create_sideview_to_tilenum(&tiles);

    // The solution to this is in a previous commit...

    data.len()
}

#[aoc(day20, part2)]
pub fn solve_part2(data: &[Tile]) -> usize {
    let tiles = create_tiles_from_data(data);

    // SideValue -> Vec<(TileNum, Rotation>>
    let sv_to_tn = create_sideview_to_tilenum(&tiles);

    // (TileNum, Rotation) -> PossibleNeighbors
    let tn_to_pn: HashMap<(TileNum, Rotation), PossibleNeighbors> = tiles
        .iter()
        .map(|(tr, sv)| {
            let mut up: Vec<(TileNum, Rotation)> = vec![];
            for tile in sv_to_tn
                .get(&sv.up)
                .expect("Edge")
                .iter()
                .filter(|&t2| t2.0 != tr.0)
            {
                let sv2 = tiles.get(tile).expect("other tile to exist");
                if sv2.down == sv.up {
                    up.push(*tile);
                }
            }
            let mut down: Vec<(TileNum, Rotation)> = vec![];
            for tile in sv_to_tn
                .get(&sv.down)
                .expect("Edge")
                .iter()
                .filter(|&t2| t2.0 != tr.0)
            {
                let sv2 = tiles.get(tile).expect("other tile to exist");
                if sv2.up == sv.down {
                    down.push(*tile);
                }
            }
            let mut left: Vec<(TileNum, Rotation)> = vec![];
            for tile in sv_to_tn
                .get(&sv.left)
                .expect("Edge")
                .iter()
                .filter(|&t2| t2.0 != tr.0)
            {
                let sv2 = tiles.get(tile).expect("other tile to exist");
                if sv2.right == sv.left {
                    left.push(*tile);
                }
            }
            let mut right: Vec<(TileNum, Rotation)> = vec![];
            for tile in sv_to_tn
                .get(&sv.right)
                .expect("Edge")
                .iter()
                .filter(|&t2| t2.0 != tr.0)
            {
                let sv2 = tiles.get(tile).expect("other tile to exist");
                if sv2.left == sv.right {
                    right.push(*tile);
                }
            }
            (
                *tr,
                PossibleNeighbors {
                    up,
                    down,
                    left,
                    right,
                },
            )
        })
        .collect();

    let mut tile_map = Vec::new();
    tile_map.resize_with(data.len(), || None);

    // Find first corner:
    for (tr, pn) in &tn_to_pn {
        if pn.up.len() == 0 && pn.left.len() == 0 {
            tile_map[0] = Some(*tr);
            break;
        }
    }

    // tile_map[0] = Some(&(TileNum(1951), Rotation(4)));

    let width = f32::from(data.len() as u16).sqrt().round() as usize;

    // Fill row...
    let mut used_tiles = HashSet::new();
    used_tiles.insert(tile_map[0].unwrap().0);
    let mut last_tile_and_rot = tile_map[0].unwrap();
    for x in 1..width {
        let last_sv = tiles.get(&last_tile_and_rot).expect("Exists").right;
        for next_tr in sv_to_tn.get(&last_sv).expect("exists") {
            let next_sv = tiles.get(next_tr).expect("Exists").left;
            if last_sv == next_sv && !used_tiles.contains(&next_tr.0) {
                let pn = tn_to_pn.get(next_tr).expect("exists");
                if pn.up.len() == 0 && pn.down.len() > 0 {
                    used_tiles.insert(next_tr.0);
                    tile_map[x] = Some(*next_tr);
                    last_tile_and_rot = *next_tr;
                    break;
                }
            }
        }
    }
    for y in 1..width {
        'x: for x in 0..width {
            let up_sv = tiles
                .get(&tile_map[(y - 1) * width + x].expect("tilemap above has value"))
                .expect("Exists")
                .down;
            for next_tr in sv_to_tn.get(&up_sv).expect("exists") {
                let next_tile_svs = tiles.get(next_tr).expect("Exists");
                if up_sv != next_tile_svs.up || used_tiles.contains(&next_tr.0) {
                    continue;
                }
                let pn = tn_to_pn.get(next_tr).expect("exists");
                if x == 0 && pn.left.len() != 0 {
                    continue;
                } else if x == width - 1 && pn.right.len() != 0 {
                    continue;
                } else if x > 0 && x < width - 1 && (pn.left.len() == 0 || pn.right.len() == 0) {
                    continue;
                } else if y == width - 1 && pn.down.len() != 0 {
                    continue;
                }
                used_tiles.insert(next_tr.0);
                tile_map[x + y * width] = Some(*next_tr);
                continue 'x;
            }
            panic!("Didn't find a tile");
            // panic!("Didn't find a tile for {x}.{y}, up_sv={up_sv:?}")
        }
    }

    let mut big_map = Vec::new();
    big_map.resize(width * width * 8 * 8, '?');

    for y in 0..width {
        for x in 0..width {
            fill_map(
                &mut big_map,
                x,
                y,
                width,
                data,
                tile_map[y * width + x].expect("tile"),
            );
        }
    }

    let monster = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];

    let mut m_count = 0;
    for i in 0..monster.len() {
        m_count += monster[i].iter().filter(|&c| *c == b'#').count()
    }

    let monster_height = monster.len();
    let monster_width = monster[0].len();

    let mut min = big_map.iter().filter(|c| **c == '#').count();

    for rot in 0..8 {
        if rot == 4 {
            rotate_map(&mut big_map, width * 8);
            flip_map(&mut big_map, width * 8);
        }
        if rot & 3 != 0 {
            rotate_map(&mut big_map, width * 8);
        }
        let mut found_monsters = false;
        for y in 0..width * 8 - monster_height {
            for x in 0..width * 8 - monster_width {
                let mut count = 0;
                for my in 0..monster.len() {
                    for mx in 0..monster[0].len() {
                        if monster[my][mx] == b'#' && big_map[mx + x + (my + y) * width * 8] == '#'
                        {
                            count += 1;
                        }
                    }
                }
                if count == m_count {
                    found_monsters = true;
                    for my in 0..monster.len() {
                        for mx in 0..monster[0].len() {
                            if monster[my][mx] == b'#' {
                                big_map[mx + x + (my + y) * width * 8] = 'O';
                            }
                        }
                    }
                }
            }
        }
        min = min.min(big_map.iter().filter(|c| **c == '#').count());
        if found_monsters {
            break;
        }
    }

    /*
    for y in 0..8 * width {
        if (y > 0) && (y % 8 == 0) {
            println!();
        }
        for x in 0..8 * width {
            if (x > 0) && (x % 8 == 0) {
                print!(" ");
            }
            print!("{}", big_map[x + y * width * 8]);
        }
        println!();
    }

    for y in 0..width {
        for x in 0..width {
            print!(
                "{:4} {} ",
                tile_map[x + y * width].unwrap().0 .0,
                tile_map[x + y * width].unwrap().1 .0
            );
        }
        println!();
    }
    */

    min
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_turn() {
        let sv = SideValues {
            up: SideValue(1),
            right: SideValue(1),
            down: SideValue(512),
            left: SideValue(512),
        };

        let sv2 = sv.turn_clockwise();

        assert_eq!(sv, sv2);
    }
}
