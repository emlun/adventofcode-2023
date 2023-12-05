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

fn solve_b(seed_ranges: &[(u64, u64)], maps: &[Vec<(u64, u64, u64)>]) -> u64 {
    maps.iter()
        .fold(seed_ranges.to_vec(), |ranges, map| {
            ranges
                .into_iter()
                .fold(Vec::new(), |mut new_ranges, (mut start, end)| {
                    for (src, dest, mapping_len) in map {
                        let intersection_start = std::cmp::max(*src, start);
                        let intersection_end = std::cmp::min(src + mapping_len, end);

                        if intersection_end > intersection_start {
                            if intersection_start > start {
                                new_ranges.push((start, intersection_start));
                            }

                            new_ranges.push((
                                dest + (intersection_start - src),
                                dest + (intersection_end - src),
                            ));
                            start = intersection_end;
                        }
                    }
                    if start < end {
                        new_ranges.push((start, end));
                    }
                    new_ranges
                })
        })
        .into_iter()
        .map(|(start, _)| start)
        .min()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let seeds_a: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();
    let (seeds_b, _): (Vec<(u64, u64)>, _) = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .fold((vec![], None), |(mut pairs, prev), next| {
            if let Some(start) = prev {
                pairs.push((start, start + next));
                (pairs, None)
            } else {
                (pairs, Some(next))
            }
        });

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
    let a = solve_a(&seeds_a, &maps);
    let maps: Vec<Vec<(u64, u64, u64)>> = maps
        .into_iter()
        .map(|map| {
            let mut m: Vec<(u64, u64, u64)> = map
                .into_iter()
                .map(|((src, len), dest)| (src, dest, len))
                .collect();
            m.sort_by_key(|(src, _, _)| *src);
            m
        })
        .collect();
    (a.to_string(), solve_b(&seeds_b, &maps).to_string())
}
