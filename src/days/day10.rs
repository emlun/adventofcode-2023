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

impl Tile {
    fn connects_to(&self, (dx, dy): (isize, isize)) -> bool {
        use Tile::*;
        matches!(
            (dx, dy, self),
            (_, 0, Horz)
                | (0, _, Vert)
                | (1, 0, NE | SE)
                | (-1, 0, NW | SW)
                | (0, 1, SE | SW)
                | (0, -1, NE | NW)
                | (_, _, Start)
        )
    }

    fn connects_from(&self, (dx, dy): (isize, isize)) -> bool {
        use Tile::*;
        matches!(
            (dx, dy, self),
            (_, 0, Horz)
                | (0, _, Vert)
                | (-1, 0, NE | SE)
                | (1, 0, NW | SW)
                | (0, -1, SE | SW)
                | (0, 1, NE | NW)
                | (_, _, Start)
        )
    }
}

fn find_main_loop(map: &HashMap<(usize, usize), Tile>) -> HashSet<(usize, usize)> {
    use Tile::*;
    let (start_x, start_y): (usize, usize) = map
        .iter()
        .find_map(|(pos, tile)| if *tile == Start { Some(*pos) } else { None })
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
                            .filter(move |dxy| map[pos].connects_to(**dxy))
                            .flat_map(move |(dx, dy)| {
                                let new_pos =
                                    (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));
                                Some(new_pos).filter(|np| {
                                    np != prev_pos
                                        && map
                                            .get(np)
                                            .map(|t| t.connects_from((*dx, *dy)))
                                            .unwrap_or(false)
                                })
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

fn identify_start((x, y): (usize, usize), map: &HashMap<(usize, usize), Tile>) -> Tile {
    use Tile::*;
    let xw = x.wrapping_add_signed(-1);
    let xe = x.wrapping_add_signed(1);
    let yn = y.wrapping_add_signed(-1);
    let ys = y.wrapping_add_signed(1);
    match (
        map.get(&(xw, y)).map(|t| t.connects_from((-1, 0))),
        map.get(&(xe, y)).map(|t| t.connects_from((1, 0))),
        map.get(&(x, yn)).map(|t| t.connects_from((0, -1))),
        map.get(&(x, ys)).map(|t| t.connects_from((0, 1))),
    ) {
        (Some(true), _, Some(true), _) => NW,
        (_, Some(true), Some(true), _) => NE,

        (Some(true), _, _, Some(true)) => SW,
        (_, Some(true), _, Some(true)) => SE,

        (_, _, Some(true), Some(true)) => Vert,
        (Some(true), Some(true), _, _) => Horz,

        _ => unreachable!(),
    }
}

fn solve_a(map: &HashMap<(usize, usize), Tile>) -> usize {
    find_main_loop(map).len() / 2
}

fn solve_b(map: &HashMap<(usize, usize), Tile>) -> usize {
    let main_loop = find_main_loop(map);

    let minx = map.keys().map(|(x, _)| *x).min().unwrap();
    let maxx = map.keys().map(|(x, _)| *x).max().unwrap();
    let miny = map.keys().map(|(_, y)| *y).min().unwrap();
    let maxy = map.keys().map(|(_, y)| *y).max().unwrap();

    (miny..maxy)
        .map(|y| {
            let (_, enclosed) = (minx..=maxx).fold((0, 0), |(crossings, enclosed), x| {
                use Tile::*;
                let is_in_main_loop = main_loop.contains(&(x, y));
                let is_crossing = is_in_main_loop
                    && match map.get(&(x, y)) {
                        Some(Vert | NE | NW) => true,
                        Some(Start) => {
                            let start_tile = identify_start((x, y), map);
                            matches!(start_tile, Vert | NE | NW)
                        }
                        _ => false,
                    };
                if is_crossing {
                    (crossings + 1, enclosed)
                } else if !is_in_main_loop {
                    (crossings, enclosed + (crossings % 2))
                } else {
                    (crossings, enclosed)
                }
            });
            enclosed
        })
        .sum()
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
