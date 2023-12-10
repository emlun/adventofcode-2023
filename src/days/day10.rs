use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Start,
    Vert,
    Horz,
    NE,
    NW,
    SW,
    SE,
    Empty,
}

fn find_main_loop(map: &HashMap<(usize, usize), Tile>) -> HashSet<(usize, usize)> {
    use Tile::*;
    let (start_x, start_y): (usize, usize) = map
        .iter()
        .find_map(|(pos, tile)| {
            if *tile == Tile::Start {
                Some(*pos)
            } else {
                None
            }
        })
        .unwrap();
    (0..)
        .scan(
            vec![((start_x, start_y), (start_x, start_y))],
            |state, _| {
                let new_poss: Vec<((usize, usize), (usize, usize))> = state
                    .iter()
                    .flat_map(|(prev_pos, pos @ (x, y))| {
                        [(-1, 0), (1, 0), (0, -1), (0, 1)]
                            .iter()
                            .filter(move |(dx, dy)| match (dx, dy, &map[&pos]) {
                                (_, 0, Horz) => true,
                                (0, _, Vert) => true,
                                (-1, 0, NW | SW) => true,
                                (1, 0, NE | SE) => true,
                                (0, -1, NE | NW) => true,
                                (0, 1, SE | SW) => true,
                                (_, _, Start) => true,
                                _ => false,
                            })
                            .flat_map(move |(dx, dy)| {
                                let new_pos = (
                                    x.overflowing_add_signed(*dx).0,
                                    y.overflowing_add_signed(*dy).0,
                                );
                                if new_pos == *prev_pos {
                                    None
                                } else {
                                    map.get(&new_pos).and_then(|t| match (dx, dy, t) {
                                        (_, 0, Horz) => Some(new_pos),
                                        (0, _, Vert) => Some(new_pos),
                                        (-1, 0, NE | SE) => Some(new_pos),
                                        (1, 0, NW | SW) => Some(new_pos),
                                        (0, -1, SE | SW) => Some(new_pos),
                                        (0, 1, NE | NW) => Some(new_pos),
                                        _ => None,
                                    })
                                }
                            })
                            .map(|new_pos| (*pos, new_pos))
                    })
                    .take(1)
                    .collect();
                *state = new_poss.clone();
                Some(new_poss)
            },
        )
        .take_while(|poss| {
            if let Some((_, curr)) = poss.first() {
                *curr != (start_x, start_y)
            } else {
                false
            }
        })
        .flat_map(|poss| poss.into_iter().map(|(_, curr)| curr))
        .chain([(start_x, start_y)])
        .collect()
}

fn solve_a(map: &HashMap<(usize, usize), Tile>) -> usize {
    find_main_loop(map).len() / 2
}

fn solve_b(map: &HashMap<(usize, usize), Tile>) -> usize {
    let main_loop = find_main_loop(map);

    map.keys()
        .filter(|pos| !main_loop.contains(pos))
        .filter(|(x, y)| {
            let parity = (0..*x)
                .filter(|xx| {
                    use Tile::*;
                    main_loop.contains(&(*xx, *y))
                        && match map.get(&(*xx, *y)) {
                            Some(Vert | NE | NW) => true,
                            Some(Start) => {
                                let start_tile = match (
                                    map.get(&(xx.wrapping_add_signed(-1), *y)).unwrap_or(&Empty),
                                    map.get(&(xx.wrapping_add_signed(1), *y)).unwrap_or(&Empty),
                                    map.get(&(*xx, y.wrapping_add_signed(-1))).unwrap_or(&Empty),
                                    map.get(&(*xx, y.wrapping_add_signed(1))).unwrap_or(&Empty),
                                ) {
                                    (Horz | NE | SE, _, Vert | SW | SE, _) => NW,
                                    (_, Horz | NW | SW, Vert | SW | SE, _) => NE,

                                    (Horz | NE | SE, _, _, Vert | NW | NE) => SW,
                                    (_, Horz | NW | SW, _, Vert | NW | NE) => SE,

                                    (_, _, Vert | SW | SE, Vert | NW | NE) => Vert,
                                    (Horz | NE | SE, Horz | NW | SW, _, _) => Horz,

                                    _ => unreachable!(),
                                };
                                match start_tile {
                                    Vert | NE | NW => true,
                                    _ => false,
                                }
                            }
                            _ => false,
                        }
                })
                .count()
                % 2;

            parity == 1
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: HashMap<(usize, usize), Tile> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().map(move |(col, chr)| {
                (
                    (col, r),
                    match chr {
                        '|' => Tile::Vert,
                        '-' => Tile::Horz,
                        'L' => Tile::NE,
                        'J' => Tile::NW,
                        '7' => Tile::SW,
                        'F' => Tile::SE,
                        '.' => Tile::Empty,
                        'S' => Tile::Start,
                        _ => unimplemented!(),
                    },
                )
            })
        })
        .collect();

    (solve_a(&map).to_string(), solve_b(&map).to_string())
}
