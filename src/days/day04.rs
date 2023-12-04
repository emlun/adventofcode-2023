use std::collections::HashSet;
use std::str::FromStr;

use crate::common::Solution;

struct Card {
    id: u32,
    win: HashSet<u32>,
    have: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut splits = input.split(':');
        let id: u32 = splits
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()?;
        let (s_win, s_have) = splits.next().unwrap().split_once('|').unwrap();

        let win = s_win.split_whitespace().flat_map(str::parse).collect();
        let have = s_have.split_whitespace().flat_map(str::parse).collect();

        Ok(Self { id, win, have })
    }
}

impl Card {
    fn num_wins(&self) -> usize {
        self.win.intersection(&self.have).count()
    }
    fn score(&self) -> u32 {
        (1 << self.num_wins()) >> 1
    }
}

fn solve_a(cards: &[Card]) -> u32 {
    cards.iter().map(Card::score).sum()
}

fn solve_b(cards: &[Card]) -> usize {
    let card_qty =
        cards
            .iter()
            .enumerate()
            .fold(vec![1; cards.len()], |mut card_qty, (i, card)| {
                let num_wins = card.num_wins();
                for wini in (i + 1)..std::cmp::min(card_qty.len(), (i + 1 + num_wins)) {
                    card_qty[wini] += card_qty[i] * 1;
                }
                card_qty
            });
    card_qty.into_iter().sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let cards: Vec<Card> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    (solve_a(&cards).to_string(), solve_b(&cards).to_string())
}
