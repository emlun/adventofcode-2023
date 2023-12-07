use std::collections::HashMap;

use crate::common::Solution;

fn categorize(hand: &[u32]) -> u32 {
    let counts: HashMap<u32, u32> =
        hand.iter()
            .fold(HashMap::with_capacity(5), |mut counts, card| {
                *counts.entry(*card).or_default() += 1;
                counts
            });
    if counts.values().any(|count| *count == 5) {
        7
    } else if counts.values().any(|count| *count == 4) {
        6
    } else if counts.values().all(|count| *count >= 2) {
        5
    } else if counts.values().any(|count| *count == 3) {
        4
    } else if counts.values().filter(|count| **count == 2).count() == 2 {
        3
    } else if counts.values().any(|count| *count == 2) {
        2
    } else {
        1
    }
}

fn solve_a(hands: &mut Vec<(u32, [u32; 5], usize)>) -> usize {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut hands: Vec<(u32, [u32; 5], usize)> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand: [u32; 5] = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    other => other.to_digit(10).unwrap(),
                })
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            (
                categorize(&hand),
                hand,
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (solve_a(&mut hands).to_string(), "".to_string())
}
