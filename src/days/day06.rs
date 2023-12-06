use crate::common::Solution;

fn solve_a(races: &[(u64, u64)]) -> usize {
    races
        .iter()
        .map(|(time, distance)| {
            (0..=*time)
                .filter(|t| {
                    let d = (time - t) * t;
                    d > *distance
                })
                .count()
        })
        .product()
}

pub fn solve(lines: &[String]) -> Solution {
    let times: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();

    let races: Vec<(u64, u64)> = times.into_iter().zip(distances.into_iter()).collect();

    (solve_a(&races).to_string(), "".to_string())
}
