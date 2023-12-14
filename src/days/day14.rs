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

pub fn solve(lines: &[String]) -> Solution {
    let (rocks, map, map_height): (HashSet<(usize, usize)>, Vec<Vec<usize>>, usize) = lines
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

    (
        solve_a(&rocks, &map, map_height).to_string(),
        "".to_string(),
    )
}
