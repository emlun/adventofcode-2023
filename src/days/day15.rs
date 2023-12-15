use crate::common::Solution;

fn hash(s: &str) -> u8 {
    s.chars().fold(0, |digest, c| {
        (digest.wrapping_add(c as u8)).wrapping_mul(17)
    })
}

fn solve_a(steps: &[&str]) -> u32 {
    steps.iter().map(|step| u32::from(hash(step))).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let steps: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(','))
        .collect();

    (solve_a(&steps).to_string(), "".to_string())
}
