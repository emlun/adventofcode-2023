use std::str::FromStr;

use crate::common::Solution;

struct Game {
    id: u32,
    samples: Box<[Sample]>,
}

struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut splits = input.split(':');
        let id: u32 = splits.next().unwrap().split(' ').nth(1).unwrap().parse()?;
        let samples: Vec<Sample> = splits
            .next()
            .unwrap()
            .split(';')
            .map(|s| s.trim())
            .map(|sample| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for part in sample.split(',').map(|s| s.trim()) {
                    let mut splits = part.split(' ');
                    let num: u32 = splits.next().unwrap().parse().unwrap();
                    let color = splits.next().unwrap();
                    let numref = match color {
                        "red" => &mut red,
                        "green" => &mut green,
                        "blue" => &mut blue,
                        _ => unreachable!(),
                    };
                    *numref = num;
                }
                Sample { red, green, blue }
            })
            .collect();
        Ok(Self {
            id,
            samples: samples.into(),
        })
    }
}

fn solve_a(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            if game
                .samples
                .iter()
                .all(|sample| sample.red <= 12 && sample.green <= 13 && sample.blue <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_b(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for sample in game.samples.iter() {
                min_red = std::cmp::max(min_red, sample.red);
                min_green = std::cmp::max(min_green, sample.green);
                min_blue = std::cmp::max(min_blue, sample.blue);
            }
            min_red * min_green * min_blue
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let games: Vec<Game> = lines.iter().map(|line| line.parse().unwrap()).collect();
    (solve_a(&games).to_string(), solve_b(&games).to_string())
}
