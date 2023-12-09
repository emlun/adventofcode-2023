use crate::{common::Solution, util::iter::WithSliding};

fn predict(values: &[i64]) -> i64 {
    let diffs: Vec<i64> = values.iter().sliding2().map(|(a, b)| b - a).collect();
    values.last().unwrap()
        + (if diffs.iter().all(|a| *a == 0) {
            0
        } else {
            predict(&diffs)
        })
}

fn solve_a(series: &[Vec<i64>]) -> i64 {
    series.iter().map(|nums| predict(nums)).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let series: Vec<Vec<i64>> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .flat_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    (solve_a(&series).to_string(), "".to_string())
}
