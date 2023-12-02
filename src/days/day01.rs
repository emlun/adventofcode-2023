use std::collections::HashMap;

use crate::common::Solution;

fn find_pattern<'p, I, V>(line: &str, patterns: &'p HashMap<&str, V>, indices: I) -> Option<&'p V>
where
    I: Iterator<Item = usize>,
{
    indices
        .flat_map(|i| {
            patterns.iter().find_map(|(pattern, digit)| {
                if line[i..].starts_with(pattern) {
                    Some(digit)
                } else {
                    None
                }
            })
        })
        .next()
}

fn calib(lines: &[String], patterns: &HashMap<&str, u32>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let first_digit = find_pattern(line, patterns, 0..line.len()).unwrap();
            let last_digit = find_pattern(line, patterns, (0..line.len()).rev()).unwrap();
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
