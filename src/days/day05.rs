use crate::common::Solution;

fn solve_a(seeds: &[u64], maps: &[Vec<(u64, u64, u64)>]) -> u64 {
    maps.iter()
        .fold(seeds.to_vec(), |values, map| {
            values
                .into_iter()
                .map(|v| {
                    map.iter()
                        .filter(|(src, _, len)| *src <= v && v < src + len)
                        .max_by_key(|(src, _, _)| src)
                        .map(|(src, dest, _)| dest + (v - src))
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

    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut next_map = Vec::new();
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            maps.push(next_map);
            next_map = Vec::new();
        } else if !line.ends_with("map:") {
            let mut parts = line.split_whitespace();
            let dest = parts.next().unwrap().parse().unwrap();
            let src = parts.next().unwrap().parse().unwrap();
            let len: u64 = parts.next().unwrap().parse().unwrap();
            next_map.push((src, dest, len));
        }
    }
    if !next_map.is_empty() {
        maps.push(next_map);
    }
    for m in &mut maps {
        m.sort_by_key(|(src, _, _)| *src);
    }

    (
        solve_a(&seeds_a, &maps).to_string(),
        solve_b(&seeds_b, &maps).to_string(),
    )
}
