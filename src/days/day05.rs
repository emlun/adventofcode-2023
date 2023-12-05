use std::collections::HashMap;

use crate::common::Solution;

fn solve_a(seeds: &[u64], maps: &[HashMap<(u64, u64), u64>]) -> u64 {
    maps.iter()
        .fold(seeds.to_vec(), |values, map| {
            values
                .into_iter()
                .map(|v| {
                    map.keys()
                        .filter(|(src, len)| *src <= v && v < src + len)
                        .max_by_key(|(src, _)| src)
                        .and_then(|(src, len)| map.get(&(*src, *len)).map(|dest| dest + (v - src)))
                        .unwrap_or(v)
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let seeds: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();
    let mut lines_iter = lines.iter().skip(2);
    let mut maps: Vec<HashMap<(u64, u64), u64>> = Vec::new();
    let mut next_map = HashMap::new();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            maps.push(next_map);
            next_map = HashMap::new();
        } else if !line.ends_with("map:") {
            let mut parts = line.split_whitespace();
            let dest = parts.next().unwrap().parse().unwrap();
            let src = parts.next().unwrap().parse().unwrap();
            let len: u64 = parts.next().unwrap().parse().unwrap();
            next_map.insert((src, len), dest);
        }
    }
    if !next_map.is_empty() {
        maps.push(next_map);
    }
    (solve_a(&seeds, &maps).to_string(), "".to_string())
}
