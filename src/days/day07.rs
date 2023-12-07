use std::collections::HashMap;

use crate::common::Solution;
use crate::util::iter::Countable;

fn categorize(hand: &[u32], joker_value: u32) -> u32 {
    let jokers = hand.iter().filter(|card| **card == joker_value).count();
    let counts: HashMap<u32, usize> = hand
        .iter()
        .copied()
        .filter(|card| *card != joker_value)
        .counts_into(HashMap::with_capacity(5));

    if jokers >= 4
        || counts
            .values()
            .any(|count| *count >= 5_usize.saturating_sub(jokers))
    {
        7
    } else if counts
        .values()
        .any(|count| *count >= 4_usize.saturating_sub(jokers))
    {
        6
    } else if counts.len() <= 2 {
        5
    } else if counts
        .values()
        .any(|count| *count >= 3_usize.saturating_sub(jokers))
    {
        4
    } else if counts.len() <= 3 {
        3
    } else if counts
        .values()
        .any(|count| *count == 2_usize.saturating_sub(jokers))
    {
        2
    } else {
        1
    }
}

fn demote_jokers(mut hand: [u32; 5]) -> [u32; 5] {
    for card in &mut hand {
        if *card == 11 {
            *card = 1
        }
    }
    hand
}

fn solve_a(hands: &mut [(u32, [u32; 5], usize)]) -> usize {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum()
}

fn solve_b(hands: Vec<(u32, [u32; 5], usize)>) -> usize {
    let mut hands: Vec<(u32, [u32; 5], usize)> = hands
        .into_iter()
        .map(|(_, hand, bid)| {
            let category = categorize(&hand, 11);
            (category, demote_jokers(hand), bid)
        })
        .collect();
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
                categorize(&hand, 20),
                hand,
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (solve_a(&mut hands).to_string(), solve_b(hands).to_string())
}
