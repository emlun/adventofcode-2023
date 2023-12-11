use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

fn solve_a(map: &HashSet<(usize, usize)>) -> usize {
    map.iter()
        .flat_map(|pos1| {
            map.iter().filter(move |pos2| pos1 != *pos2).map(|pos2| {
                let (x1, y1) = pos1;
                let (x2, y2) = pos2;
                x1.abs_diff(*x2) + y1.abs_diff(*y2)
            })
        })
        .sum::<usize>()
        / 2
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<bool>> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(_, chr)| chr == '#')
                .collect()
        })
        .collect();

    let map = map.into_iter().fold(Vec::new(), |mut result, row| {
        if row.iter().all(|b| !*b) {
            result.push(row.clone());
        }
        result.push(row);
        result
    });

    let (map, _): (Vec<Vec<bool>>, _) =
        (0..map[0].len()).fold((map, 0), |(mut result, mut dc), c| {
            if result.iter().all(|row| !row[c + dc]) {
                for row in result.iter_mut() {
                    row.insert(c + dc, false);
                }
                dc += 1;
            }
            (result, dc)
        });

    let map: HashSet<(usize, usize)> = map
        .into_iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, b)| *b)
                .map(move |(c, _)| (c, r))
        })
        .collect();

    (solve_a(&map).to_string(), "".to_string())
}
