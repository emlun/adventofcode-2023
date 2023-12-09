use crate::{common::Solution, util::iter::WithSliding};

fn predict(values: &[i64], forward: bool) -> i64 {
    let diffs: Vec<i64> = values.iter().sliding2().map(|(a, b)| b - a).collect();

    let predicted_diff = if diffs.iter().all(|a| *a == 0) {
        0
    } else {
        predict(&diffs, forward)
    };

    if forward {
        values.last().unwrap() + predicted_diff
    } else {
        values.first().unwrap() - predicted_diff
    }
}

fn solve_a(series: &[Vec<i64>]) -> i64 {
    series.iter().map(|nums| predict(nums, true)).sum()
}

fn solve_b(series: &[Vec<i64>]) -> i64 {
    series.iter().map(|nums| predict(nums, false)).sum()
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

    (solve_a(&series).to_string(), solve_b(&series).to_string())
}
