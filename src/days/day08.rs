use std::collections::HashMap;

use crate::common::Solution;

fn solve_a(instructions: &[bool], map: &HashMap<&str, (&str, &str)>) -> usize {
    instructions
        .iter()
        .cycle()
        .scan("AAA", |location, step| {
            let (left, right) = map[location];
            if *location == "ZZZ" {
                None
            } else {
                *location = if *step { right } else { left };
                Some(*location)
            }
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let instructions: Vec<bool> = lines[0].chars().map(|c| c == 'R').collect();
    let map: HashMap<&str, (&str, &str)> = lines
        .iter()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (from, to) = line.split_once('=').unwrap();
            let (left, right) = to.split_once(',').unwrap();
            (
                from.trim(),
                (
                    left.trim().strip_prefix('(').unwrap().trim(),
                    right.trim().strip_suffix(')').unwrap().trim(),
                ),
            )
        })
        .collect();

    (solve_a(&instructions, &map).to_string(), "".to_string())
}
