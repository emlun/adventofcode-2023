use std::collections::HashMap;

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

fn solve_a(map: &HashMap<(usize, usize), Tile>) -> usize {
    use Tile::*;
    let (start_x, start_y) = map
        .iter()
        .find_map(|(pos, tile)| {
            if *tile == Tile::Start {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();
    1 + (0..)
        .scan(
            vec![((*start_x, *start_y), (*start_x, *start_y))],
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
                                    match (dx, dy, &map[&new_pos]) {
                                        (_, 0, Horz) => Some(new_pos),
                                        (0, _, Vert) => Some(new_pos),
                                        (-1, 0, NE | SE) => Some(new_pos),
                                        (1, 0, NW | SW) => Some(new_pos),
                                        (0, -1, SE | SW) => Some(new_pos),
                                        (0, 1, NE | NW) => Some(new_pos),
                                        _ => None,
                                    }
                                }
                            })
                            .map(|new_pos| (*pos, new_pos))
                    })
                    .collect();
                *state = new_poss.clone();
                Some(new_poss)
            },
        )
        .take_while(|poss| {
            !(poss.iter().all(|(_, pos)| *pos == poss[0].1)
                || (poss[0].0 == poss[1].1 && poss[1].0 == poss[0].1))
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

    (solve_a(&map).to_string(), "".to_string())
}
