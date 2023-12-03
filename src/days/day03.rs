use std::collections::HashMap;

use crate::common::Solution;

fn solve_a(numbered_parts: &HashMap<(usize, usize), (char, Vec<u32>)>) -> u32 {
    numbered_parts.values().flat_map(|(_, nums)| nums).sum()
}

fn solve_b(numbered_parts: &HashMap<(usize, usize), (char, Vec<u32>)>) -> u32 {
    numbered_parts
        .values()
        .filter(|(part, nums)| *part == '*' && nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut parts: HashMap<(usize, usize), char> = HashMap::new();
    let mut nums: Vec<(usize, usize, isize, u32)> = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        let mut current_num = None;
        for (c, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                if let Some((r, c, l, cur)) = current_num {
                    current_num = Some((r, c, l + 1, cur * 10 + chr.to_digit(10).unwrap()));
                } else {
                    current_num = Some((r, c, 1, chr.to_digit(10).unwrap()));
                }
            } else {
                if let Some(num) = current_num {
                    nums.push(num);
                    current_num = None;
                }
                if chr != '.' {
                    parts.insert((r, c), chr);
                }
            }
        }
        if let Some(num) = current_num {
            nums.push(num);
        }
    }

    let mut numbered_parts: HashMap<(usize, usize), (char, Vec<u32>)> = HashMap::new();
    for (numr, numc, numl, num) in nums {
        for (part_pos, chr) in (-1..=1)
            .flat_map(|dr| (-1..=numl).map(move |dc| (dr, dc)))
            .flat_map(|(dr, dc)| {
                let part_pos = (
                    numr.saturating_add_signed(dr),
                    numc.saturating_add_signed(dc),
                );
                parts.get(&part_pos).map(|chr| (part_pos, chr))
            })
        {
            numbered_parts
                .entry(part_pos)
                .or_insert((*chr, Vec::new()))
                .1
                .push(num);
        }
    }

    (
        solve_a(&numbered_parts).to_string(),
        solve_b(&numbered_parts).to_string(),
    )
}
