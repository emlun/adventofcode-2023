use std::collections::HashMap;

use crate::common::Solution;

fn find_pattern<I>(line: &str, patterns: &HashMap<&str, u32>, indices: I) -> Option<u32>
where
    I: Iterator<Item = (usize, char)>,
{
    indices
        .flat_map(|(i, c)| {
            if c.is_numeric() {
                Some(c.to_digit(10).unwrap())
            } else {
                patterns.iter().find_map(|(pattern, digit)| {
                    if line[i..].starts_with(pattern) {
                        Some(*digit)
                    } else {
                        None
                    }
                })
            }
        })
        .next()
}

fn calib(lines: &[String], patterns: &HashMap<&str, u32>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let first_digit = find_pattern(line, patterns, line.char_indices()).unwrap();
            let last_digit = find_pattern(line, patterns, line.char_indices().rev()).unwrap();
            10 * first_digit + last_digit
        })
        .sum()
}

fn solve_a(lines: &[String]) -> u32 {
    calib(lines, &HashMap::new())
}

fn solve_b(lines: &[String]) -> u32 {
    let digit_pattern: HashMap<&str, u32> = [
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
