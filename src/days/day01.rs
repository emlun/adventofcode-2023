use std::collections::HashMap;

use crate::common::Solution;

fn calib(lines: &[String], patterns: &HashMap<&str, u32>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let first_digit = patterns[patterns
                .keys()
                .flat_map(|pat| line.find(pat).map(|i| (i, pat)))
                .min_by(|(i1, _), (i2, _)| i1.cmp(i2))
                .unwrap()
                .1];
            let last_digit = patterns[patterns
                .keys()
                .flat_map(|pat| line.rfind(pat).map(|i| (i, pat)))
                .max_by(|(i1, _), (i2, _)| i1.cmp(i2))
                .unwrap()
                .1];
            10 * first_digit + last_digit
        })
        .sum()
}

fn solve_a(lines: &[String]) -> u32 {
    let digit_pattern: HashMap<&str, u32> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .copied()
    .collect();
    calib(lines, &digit_pattern)
}

fn solve_b(lines: &[String]) -> u32 {
    let digit_pattern: HashMap<&str, u32> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .copied()
    .collect();
    calib(lines, &digit_pattern)
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
