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

fn solve_b(races: &[(u64, u64)]) -> usize {
    let (time, distance) = races
        .iter()
        .fold((0_u64, 0_u64), |(time, distance), (t, d)| {
            (
                time * u64::from(10_u32.pow(t.ilog10() + 1)) + *t,
                distance * u64::from(10_u32.pow(d.ilog10() + 1)) + *d,
            )
        });

    (0..=time)
        .filter(|t| {
            let d = (time - t) * t;
            d > distance
        })
        .count()
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

    (solve_a(&races).to_string(), solve_b(&races).to_string())
}
