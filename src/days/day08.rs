use std::collections::HashMap;

use crate::common::Solution;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let gcdab = gcd(a, b);
    (a / gcdab) * b
}

fn solve_a(instructions: &[bool], map: &HashMap<&str, (&str, &str)>) -> usize {
    instructions
        .iter()
        .cycle()
        .scan("AAA", |location, step| {
            if *location == "ZZZ" {
                None
            } else {
                let (left, right) = map[location];
                *location = if *step { right } else { left };
                Some(*location)
            }
        })
        .count()
}

fn solve_b(instructions: &[bool], map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut locations: Vec<(&str, Option<usize>)> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .map(|loc| (loc, None))
        .collect();

    for (i, step) in instructions.iter().cycle().enumerate() {
        for (loc, period) in locations.iter_mut() {
            let (left, right) = map[loc];
            *loc = if *step { right } else { left };
            if period.is_none() && loc.ends_with('Z') {
                *period = Some(i + 1);
            }
        }

        if locations.iter().all(|(_, period)| period.is_some()) {
            return locations
                .into_iter()
                .flat_map(|(_, period)| period)
                .fold(1, lcm);
        }
    }

    unreachable!()
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

    (
        solve_a(&instructions, &map).to_string(),
        solve_b(&instructions, &map).to_string(),
    )
}
