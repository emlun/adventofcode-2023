use crate::common::Solution;

fn to_num(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!(),
    }
}

fn calib(lines: &[String], patterns: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let first_digit = to_num(
                patterns[patterns
                    .iter()
                    .enumerate()
                    .flat_map(|(pat_i, pat)| line.find(pat).map(|i| (i, pat_i)))
                    .min_by(|(i1, _), (i2, _)| i1.cmp(i2))
                    .unwrap()
                    .1],
            );
            let last_digit = to_num(
                patterns[patterns
                    .iter()
                    .enumerate()
                    .flat_map(|(pat_i, pat)| line.rfind(pat).map(|i| (i, pat_i)))
                    .max_by(|(i1, _), (i2, _)| i1.cmp(i2))
                    .unwrap()
                    .1],
            );
            10 * first_digit + last_digit
        })
        .sum()
}

fn solve_a(lines: &[String]) -> u32 {
    let digit_pattern = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    calib(lines, digit_pattern)
}

fn solve_b(lines: &[String]) -> u32 {
    let digit_pattern = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    calib(lines, digit_pattern)
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
