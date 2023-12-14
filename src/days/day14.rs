use std::collections::HashMap;
use std::collections::HashSet;

use crate::{common::Solution, util::iter::Countable};

fn solve_a(rocks: &HashSet<(usize, usize)>, map: &[Vec<usize>], map_height: usize) -> usize {
    let mut rolled_rs: Vec<Vec<usize>> = vec![vec![]; map.len()];
    for (r, c) in rocks {
        let rolled_r = map[*c]
            .iter()
            .take_while(|rr| *rr < r)
            .last()
            .map(|r| r + 1)
            .unwrap_or(0);
        rolled_rs[*c].push(rolled_r);
    }
    rolled_rs
        .into_iter()
        .map(|rs| rs.into_iter().counts())
        .map(|rolled_counts| {
            rolled_counts
                .into_iter()
                .map(|(r, count)| {
                    (map_height - r) * count
                        - (count.saturating_sub(0) * count.saturating_sub(1) / 2)
                })
                .sum::<usize>()
        })
        .sum()
}

fn spin_cycle(
    mut rocks: Vec<(usize, usize)>,
    map_ns: &[Vec<usize>],
    map_we: &[Vec<usize>],
    map_height: usize,
    map_width: usize,
) -> Vec<(usize, usize)> {
    let mut roll_counts: HashMap<(usize, usize), usize> = HashMap::with_capacity(rocks.len());

    for (r, c) in rocks.iter_mut() {
        let rolled_r = map_ns[*c]
            .iter()
            .take_while(|rr| *rr < r)
            .last()
            .map(|r| r + 1)
            .unwrap_or(0);
        let colliding_count = roll_counts.entry((rolled_r, *c)).or_insert(0);
        *r = rolled_r + *colliding_count;
        *colliding_count += 1;
    }

    roll_counts.clear();
    for (r, c) in rocks.iter_mut() {
        let rolled_c = map_we[*r]
            .iter()
            .take_while(|cc| *cc < c)
            .last()
            .map(|c| c + 1)
            .unwrap_or(0);
        let colliding_count = roll_counts.entry((*r, rolled_c)).or_insert(0);
        *c = rolled_c + *colliding_count;
        *colliding_count += 1;
    }

    roll_counts.clear();
    for (r, c) in rocks.iter_mut() {
        let rolled_r = map_ns[*c]
            .iter()
            .find(|rr| *rr > r)
            .map(|r| r - 1)
            .unwrap_or(map_height - 1);
        let colliding_count = roll_counts.entry((rolled_r, *c)).or_insert(0);
        *r = rolled_r - *colliding_count;
        *colliding_count += 1;
    }

    roll_counts.clear();
    for (r, c) in rocks.iter_mut() {
        let rolled_c = map_we[*r]
            .iter()
            .find(|cc| *cc > c)
            .map(|c| c - 1)
            .unwrap_or(map_width - 1);
        let colliding_count = roll_counts.entry((*r, rolled_c)).or_insert(0);
        *c = rolled_c - *colliding_count;
        *colliding_count += 1;
    }

    rocks
}

fn solve_b(
    rocks: &HashSet<(usize, usize)>,
    map_ns: &[Vec<usize>],
    map_we: &[Vec<usize>],
    map_height: usize,
    map_width: usize,
) -> usize {
    let mut rocks: Vec<(usize, usize)> = rocks.iter().copied().collect();
    let mut states_seen: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut i = 0;
    const SPINS: usize = 1_000_000_000;
    while i < SPINS {
        if let Some(prev_i) = states_seen.get(&rocks) {
            let cycle_len = i - prev_i;
            i = SPINS - ((SPINS - i) % cycle_len)
        }

        states_seen.insert(rocks.clone(), i);
        rocks = spin_cycle(rocks, map_ns, map_we, map_height, map_width);
        rocks.sort();

        i += 1;
    }

    rocks.into_iter().map(|(r, _)| map_height - r).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (rocks, map_ns, map_height): (HashSet<(usize, usize)>, Vec<Vec<usize>>, usize) = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            (HashSet::new(), Vec::new(), 0),
            |(mut rocks, mut map, map_height), (r, line)| {
                for (c, chr) in line.chars().enumerate() {
                    match chr {
                        'O' => {
                            rocks.insert((r, c));
                        }
                        '#' => {
                            map.resize_with(std::cmp::max(map.len(), c + 1), Vec::new);
                            map[c].push(r);
                        }
                        _ => {}
                    }
                }
                (rocks, map, map_height + 1)
            },
        );
    let map_we: Vec<Vec<usize>> =
        map_ns
            .iter()
            .enumerate()
            .fold(vec![vec![]; map_height], |mut map_we, (c, column)| {
                for r in column.iter().copied() {
                    map_we[r].push(c);
                }
                map_we
            });
    let map_width = map_ns.len();

    (
        solve_a(&rocks, &map_ns, map_height).to_string(),
        solve_b(&rocks, &map_ns, &map_we, map_height, map_width).to_string(),
    )
}
