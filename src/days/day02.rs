use crate::common::Solution;

fn solve_a(lines: &[String]) -> u32 {
    lines
        .iter()
        .filter_map(|line| {
            let mut splits = line.split(":");
            let id: u32 = splits
                .next()
                .unwrap()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            if splits
                .next()
                .unwrap()
                .split(";")
                .map(|s| s.trim())
                .all(|sample| {
                    sample.split(",").map(|s| s.trim()).all(|part| {
                        let mut splits = part.split(" ");
                        let num: u32 = splits.next().unwrap().parse().unwrap();
                        let color = splits.next().unwrap();
                        let limit = match color {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => unreachable!(),
                        };
                        num <= limit
                    })
                })
            {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_b(lines: &[String]) -> u32 {
    0
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), solve_b(lines).to_string())
}
