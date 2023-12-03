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
    type NumberLocation = (usize, usize, isize, u32);
    let (parts, nums): (HashMap<(usize, usize), char>, Vec<NumberLocation>) = lines
        .iter()
        .enumerate()
        .fold((HashMap::new(), Vec::new()), |(parts, nums), (r, line)| {
            let (parts, mut nums, unfinished_num) = line.chars().enumerate().fold(
                (parts, nums, None),
                |(mut parts, mut nums, mut next_num), (c, chr)| {
                    if let Some(digit) = chr.to_digit(10) {
                        let (_, _, l, cur) = next_num.get_or_insert((r, c, 0, 0));
                        *l += 1;
                        *cur = *cur * 10 + digit;
                    } else {
                        nums.extend(next_num.take());
                        if chr != '.' {
                            parts.insert((r, c), chr);
                        }
                    }
                    (parts, nums, next_num)
                },
            );
            nums.extend(unfinished_num);

            (parts, nums)
        });

    let numbered_parts: HashMap<(usize, usize), (char, Vec<u32>)> = nums.into_iter().fold(
        HashMap::new(),
        |mut numbered_parts, (numr, numc, numl, num)| {
            for (part_pos, chr) in (-1..=1)
                .flat_map(|dr| (-1..=numl).map(move |dc| (dr, dc)))
                .filter(|drc| *drc != (0, 0))
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
            numbered_parts
        },
    );

    (
        solve_a(&numbered_parts).to_string(),
        solve_b(&numbered_parts).to_string(),
    )
}
